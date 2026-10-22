// Modified to propagate subscription failures and sign after connection establishment.
use crate::error::Result;
use base64ct::{Base64, Encoding};
use ed25519_dalek::Signer;
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::protocol::{Message, frame::coding::CloseCode};
#[cfg(not(feature = "permessage-deflate"))]
use tokio_tungstenite::{connect_async, tungstenite::Utf8Bytes};
#[cfg(feature = "permessage-deflate")]
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::{
        Utf8Bytes, extensions::compression::deflate::DeflateConfig, protocol::WebSocketConfig,
    },
};

use crate::{BpxClient, DEFAULT_WINDOW, Error, now_millis};

impl BpxClient {
    /// Subscribes to a WebSocket stream and sends messages of type `T` through a transmitter channel.
    ///
    /// Dropping the receiver cancels setup or an established subscription. A close
    /// without a status, or with status 1000 (normal) or 1001 (going away), returns
    /// `Ok(())`; other close statuses return [`Error::WebSocketClosed`].
    /// Private subscriptions are signed after the handshake. Acknowledgements
    /// and other messages without data or an error are ignored. Messages that
    /// are not valid JSON, and event payloads that cannot decode as `T`, are
    /// logged and skipped without ending the subscription. A full receiver channel
    /// pauses socket reads; callers should keep draining it to process control
    /// frames and errors promptly.
    ///
    /// No automatic reconnect or resynchronization is performed. After any exit,
    /// callers must decide whether to reconnect and refresh their state.
    ///
    /// # Errors
    /// Returns [`Error::NotAuthenticated`] for private streams without credentials,
    /// [`Error::WebSocket`] for connection, send or read failures,
    /// [`Error::WebSocketApiError`] for server error responses, and
    /// [`Error::WebSocketClosed`] for unsuccessful close statuses.
    pub async fn subscribe<T>(&self, stream: &str, tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(&[stream], tx).await
    }

    /// Subscribes to multiple WebSocket streams using the same lifecycle and error
    /// behavior as [`Self::subscribe`]. Events that do not decode as `T` are
    /// skipped, so `T` should cover every subscribed stream.
    ///
    /// If any stream is private, credentials are required for the subscription.
    pub async fn subscribe_multiple<T>(&self, streams: &[&str], tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(streams, tx).await
    }

    async fn internal_subscribe<T>(&self, streams: &[&str], tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let is_private = streams.iter().any(|s| is_private_stream(s));
        let signing_key = if is_private {
            Some(self.signing_key.as_ref().ok_or(Error::NotAuthenticated)?)
        } else {
            None
        };

        let ws_url = self.ws_url.as_str();
        #[cfg(not(feature = "permessage-deflate"))]
        let connect = connect_async(ws_url);
        // Offer permessage-deflate on the handshake. The server selects the
        // extension when it supports it and falls back to an uncompressed
        // connection when it does not, so the offer is always safe.
        #[cfg(feature = "permessage-deflate")]
        let connect = {
            let mut config = WebSocketConfig::default();
            config.extensions.permessage_deflate = Some(DeflateConfig::default());
            connect_async_with_config(ws_url, Some(config), false)
        };
        let (mut ws_stream, _) = tokio::select! {
            _ = tx.closed() => return Ok(()),
            result = connect => result?,
        };
        let mut subscribe_message = json!({"method": "SUBSCRIBE", "params": streams});
        if let Some(signing_key) = signing_key {
            let timestamp = now_millis();
            let window = DEFAULT_WINDOW;
            let message = format!("instruction=subscribe&timestamp={timestamp}&window={window}");
            let verifying_key = Base64::encode_string(&signing_key.verifying_key().to_bytes());
            let signature = Base64::encode_string(&signing_key.sign(message.as_bytes()).to_bytes());
            subscribe_message["signature"] = json!([
                verifying_key,
                signature,
                timestamp.to_string(),
                window.to_string()
            ]);
        }
        let message = Message::Text(Utf8Bytes::from(subscribe_message.to_string()));
        tokio::select! {
            _ = tx.closed() => return Ok(()),
            result = ws_stream.send(message) => result?,
        }

        tracing::debug!("Subscription sent for {streams:#?}");

        loop {
            let message = tokio::select! {
                _ = tx.closed() => return Ok(()),
                message = ws_stream.next() => message,
            };
            let Some(message) = message else { break };
            match message? {
                Message::Text(text) => {
                    let value = match serde_json::from_str::<Value>(&text) {
                        Ok(value) => value,
                        Err(err) => {
                            tracing::error!("Could not parse ws message: {err}");
                            continue;
                        }
                    };
                    if let Some(error) = value.get("error").filter(|error| !error.is_null()) {
                        return Err(Error::WebSocketApiError(Box::new(error.clone())));
                    }
                    if let Some(payload) = value.get("data") {
                        match T::deserialize(payload) {
                            Ok(data) => {
                                if tx.send(data).await.is_err() {
                                    break;
                                }
                            }
                            Err(err) => {
                                tracing::error!("Could not deserialize ws payload: {err}");
                            }
                        }
                    }
                }
                Message::Close(frame) => {
                    if let Some(frame) = frame
                        && !matches!(frame.code, CloseCode::Normal | CloseCode::Away)
                    {
                        return Err(Error::WebSocketClosed {
                            code: frame.code.into(),
                            reason: frame.reason.to_string().into(),
                        });
                    }
                    break;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn is_private_stream(stream: &str) -> bool {
    stream.starts_with("account.")
}
