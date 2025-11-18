use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use ed25519_dalek::Signer;
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, tungstenite::Utf8Bytes};

use crate::{BACKPACK_WS_URL, BpxClient, DEFAULT_WINDOW, Error, error::Result, now_millis};

impl BpxClient {
    /// Subscribes to a private WebSocket stream and sends messages of type `T` through a transmitter channel.
    pub async fn subscribe<T>(&self, stream: &str, tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(&[stream], tx).await
    }

    /// Subscribes to multiple private WebSocket streams and sends messages of type `T` through a transmitter channel.
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
        let subscribe_message = if is_private {
            let auth_keys: &crate::AuthKeyPair = self.auth_keys.as_ref().ok_or_else(|| {
                let private_streams = streams
                    .iter()
                    .filter(|s| is_private_stream(s))
                    .copied()
                    .collect::<Vec<_>>()
                    .join(", ");
                Error::NoSecretKey(format!("WebSocket subscription to private stream(s): {private_streams}").into())
            })?;

            let timestamp = now_millis();
            let window = DEFAULT_WINDOW;
            let message = format!("instruction=subscribe&timestamp={timestamp}&window={window}");

            let verifying_key = STANDARD.encode(auth_keys.verifier.to_bytes());
            let signature = STANDARD.encode(auth_keys.signer.sign(message.as_bytes()).to_bytes());

            json!({
                "method": "SUBSCRIBE",
                "params": streams,
                "signature": [verifying_key, signature, timestamp.to_string(), window.to_string()],
            })
        } else {
            json!({
                "method": "SUBSCRIBE",
                "params": streams
            })
        };

        let ws_url = self.ws_url.as_deref().unwrap_or(BACKPACK_WS_URL);
        let (mut ws_stream, _) = connect_async(ws_url).await.expect("Error connecting to WebSocket");
        ws_stream
            .send(Message::Text(Utf8Bytes::from(subscribe_message.to_string())))
            .await
            .expect("Error subscribing to WebSocket");

        tracing::debug!("Subscribed to {streams:#?} streams...");

        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(msg) => match msg {
                    Message::Text(text) => {
                        if let Ok(value) = serde_json::from_str::<Value>(&text) {
                            if let Some(payload) = value.get("data") {
                                if let Ok(data) = T::deserialize(payload)
                                    && tx.send(data).await.is_err()
                                {
                                    tracing::error!("Failed to send message through the channel");
                                }
                            } else if let Some(payload) = value.get("error") {
                                tracing::error!("Websocket Error Response: {}", payload);
                            }
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                },
                Err(error) => tracing::error!("WebSocket error: {}", error),
            }
        }
        Ok(())
    }
}

fn is_private_stream(stream: &str) -> bool {
    stream.starts_with("account.")
}
