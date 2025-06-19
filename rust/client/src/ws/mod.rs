use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::Signer;
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, tungstenite::Utf8Bytes};

use crate::{now_millis, BpxClient, BACKPACK_WS_URL, DEFAULT_WINDOW};

impl BpxClient {
    /// Subscribes to a private WebSocket stream and sends messages of type `T` through a transmitter channel.
    pub async fn subscribe<T>(&self, stream: &str, tx: Sender<T>)
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(&[stream], tx).await
    }

    /// Subscribes to multiple private WebSocket streams and sends messages of type `T` through a transmitter channel.
    pub async fn subscribe_multiple<T>(&self, stream: &[&str], tx: Sender<T>)
    where
        T: DeserializeOwned + Send + 'static,
    {
        self.internal_subscribe(stream, tx).await
    }

    async fn internal_subscribe<T>(&self, stream: &[&str], tx: Sender<T>)
    where
        T: DeserializeOwned + Send + 'static,
    {
        let timestamp = now_millis();
        let window = DEFAULT_WINDOW;
        let message = format!("instruction=subscribe&timestamp={}&window={}", timestamp, window);

        let verifying_key = STANDARD.encode(self.verifier.to_bytes());
        let signature = STANDARD.encode(self.signer.sign(message.as_bytes()).to_bytes());

        let subscribe_message = json!({
            "method": "SUBSCRIBE",
            "params": stream,
            "signature": [verifying_key, signature, timestamp.to_string(), window.to_string()],
        });

        let ws_url = self.ws_url.as_deref().unwrap_or(BACKPACK_WS_URL);
        let (mut ws_stream, _) = connect_async(ws_url).await.expect("Error connecting to WebSocket");
        ws_stream
            .send(Message::Text(Utf8Bytes::from(subscribe_message.to_string())))
            .await
            .expect("Error subscribing to WebSocket");

        tracing::debug!("Subscribed to {stream:#?} streams...");

        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(msg) => match msg {
                    Message::Text(text) => {
                        if let Ok(value) = serde_json::from_str::<Value>(&text) {
                            if let Some(payload) = value.get("data") {
                                if let Ok(data) = T::deserialize(payload) {
                                    if tx.send(data).await.is_err() {
                                        tracing::error!("Failed to send message through the channel");
                                    }
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
    }
}
