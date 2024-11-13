use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::Signer;
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde_json::json;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::now_millis;
use crate::BpxClient;
use crate::BACKPACK_WS_URL;
use crate::DEFAULT_WINDOW;

impl BpxClient {
    /// Helper function to subscribe to a private WebSocket stream.
    pub async fn subscribe(&self, stream: &str) {
        let timestamp = now_millis();
        let window = DEFAULT_WINDOW;
        let message = format!("instruction=subscribe&timestamp={}&window={}", timestamp, window);

        let verifying_key = STANDARD.encode(self.verifier.to_bytes());
        let signature = STANDARD.encode(self.signer.sign(message.as_bytes()).to_bytes());

        let subscribe_message = json!({
            "method": "SUBSCRIBE",
            "params": [stream.to_string()],
            "signature": [verifying_key, signature, timestamp.to_string(), window.to_string()],
        });

        let ws_url = self.ws_url.as_deref().unwrap_or(BACKPACK_WS_URL);
        let (mut ws_stream, _) = connect_async(ws_url).await.expect("Error connecting to WebSocket");
        ws_stream
            .send(Message::Text(subscribe_message.to_string()))
            .await
            .expect("Error subscribing to WebSocket");

        println!("Subscribed to {stream}");
        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(msg) => println!("{}", msg),
                Err(error) => eprintln!("{}", error),
            }
        }
    }
}
