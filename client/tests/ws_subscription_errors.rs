#![cfg(feature = "ws")]

use bpx_api_client::{BpxClient, Error};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::time::Duration;
use tokio::{net::TcpListener, sync::mpsc, time::timeout};
use tokio_tungstenite::{accept_async_with_config, tungstenite};
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::any};

const TEST_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::test]
async fn rejected_handshake_returns_the_http_error_for_both_subscription_methods() {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(503))
        .expect(2)
        .mount(&server)
        .await;
    let client = BpxClient::builder()
        .ws_url(server.uri().replacen("http://", "ws://", 1))
        .build()
        .unwrap();

    for multiple in [false, true] {
        let (tx, _rx) = mpsc::channel::<Value>(1);
        let result = timeout(TEST_TIMEOUT, async {
            if multiple {
                client
                    .subscribe_multiple(&["bookTicker.SOL_USDC"], tx)
                    .await
            } else {
                client.subscribe("bookTicker.SOL_USDC", tx).await
            }
        })
        .await
        .expect("handshake should finish");
        match result {
            Err(Error::WebSocket(error)) => match *error {
                tungstenite::Error::Http(response) => assert_eq!(response.status().as_u16(), 503),
                other => panic!("expected HTTP rejection, got {other:?}"),
            },
            other => panic!("expected WebSocket error, got {other:?}"),
        }
    }
}

#[tokio::test]
async fn connection_closed_during_handshake_returns_an_error() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .build()
        .unwrap();
    let (tx, _rx) = mpsc::channel::<Value>(1);
    let peer = async {
        let (socket, _) = listener.accept().await.unwrap();
        drop(socket);
    };
    let (_, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe("bookTicker.SOL_USDC", tx))
    })
    .await
    .expect("closed handshake should finish");
    assert!(matches!(result, Err(Error::WebSocket(_))), "{result:?}");
}

#[tokio::test]
async fn successful_subscription_still_delivers_data_and_closes_normally() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .build()
        .unwrap();
    let streams = ["bookTicker.SOL_USDC", "bookTicker.BTC_USDC"];
    let data = json!({"symbol": "SOL_USDC", "price": "100.00"});
    let (tx, mut rx) = mpsc::channel::<Value>(1);
    let peer = async {
        let (socket, _) = listener.accept().await.unwrap();
        let config = tungstenite::protocol::WebSocketConfig::default();
        #[cfg(feature = "permessage-deflate")]
        let config = {
            let mut config = config;
            config.extensions.permessage_deflate = Some(Default::default());
            config
        };
        let mut ws = accept_async_with_config(socket, Some(config))
            .await
            .unwrap();
        let message = ws.next().await.unwrap().unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(message.to_text().unwrap()).unwrap(),
            json!({"method": "SUBSCRIBE", "params": streams})
        );
        ws.send(tungstenite::Message::text(
            json!({"data": data}).to_string(),
        ))
        .await
        .unwrap();
        ws.close(None).await.unwrap();
    };
    let (_, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe_multiple(&streams, tx))
    })
    .await
    .expect("subscription should finish");
    result.unwrap();
    assert_eq!(rx.try_recv().unwrap(), data);
}
