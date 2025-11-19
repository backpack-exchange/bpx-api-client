use crate::error::Result;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use ed25519_dalek::Signer;
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, tungstenite::Utf8Bytes};

use crate::{BpxClient, DEFAULT_WINDOW, Error, now_millis};

impl BpxClient {
    /// Subscribes to a private WebSocket stream and sends messages of type `T` through a transmitter channel.
    pub async fn subscribe<T>(&self, stream: &str, tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(&[stream], tx).await
    }

    /// Subscribes to multiple private WebSocket streams and sends messages of type `T` through a transmitter channel.
    pub async fn subscribe_multiple<T>(&self, stream: &[&str], tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(stream, tx).await
    }

    async fn internal_subscribe<T>(&self, stream: &[&str], tx: Sender<T>) -> Result<()>
    where
        T: DeserializeOwned + Send + 'static,
    {
        let timestamp = now_millis();
        let window = DEFAULT_WINDOW;
        let message = format!("instruction=subscribe&timestamp={timestamp}&window={window}");
        let Some(signing_key) = &self.signing_key else {
            return Err(Error::NotAuthenticated);
        };

        let verifying_key = STANDARD.encode(signing_key.verifying_key().to_bytes());
        let signature = STANDARD.encode(signing_key.sign(message.as_bytes()).to_bytes());

        let subscribe_message = json!({
            "method": "SUBSCRIBE",
            "params": stream,
            "signature": [verifying_key, signature, timestamp.to_string(), window.to_string()],
        });

        let ws_url = self.ws_url.as_str();
        let (mut ws_stream, _) = connect_async(ws_url)
            .await
            .expect("Error connecting to WebSocket");
        ws_stream
            .send(Message::Text(Utf8Bytes::from(
                subscribe_message.to_string(),
            )))
            .await
            .expect("Error subscribing to WebSocket");

        tracing::debug!("Subscribed to {stream:#?} streams...");

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
                                tracing::error!(?payload, "Websocket Error Response");
                            }
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                },
                Err(error) => tracing::error!(%error, "WebSocket error"),
            }
        }

        Ok(())
    }
}
