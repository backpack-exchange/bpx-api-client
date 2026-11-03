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
async fn dropping_receiver_cancels_a_stalled_handshake() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .build()
        .unwrap();
    let (tx, rx) = mpsc::channel::<Value>(1);
    let peer = async {
        let (socket, _) = listener.accept().await.unwrap();
        drop(rx);
        socket
    };
    // Hold the TCP connection open without replying to the upgrade request.
    let (socket, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe("bookTicker.SOL_USDC", tx))
    })
    .await
    .expect("receiver cancellation should not wait for the handshake");
    result.unwrap();
    drop(socket);
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
        let mut ws = accept_async_with_config(socket, Some(server_config()))
            .await
            .unwrap();
        let message = ws.next().await.unwrap().unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(message.to_text().unwrap()).unwrap(),
            json!({"method": "SUBSCRIBE", "params": streams})
        );
        ws.send(tungstenite::Message::text(r#"{"result":null,"id":1}"#))
            .await
            .unwrap();
        ws.send(tungstenite::Message::Ping(vec![1, 2, 3].into()))
            .await
            .unwrap();
        assert_eq!(
            ws.next().await.unwrap().unwrap(),
            tungstenite::Message::Pong(vec![1, 2, 3].into())
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

fn server_config() -> tungstenite::protocol::WebSocketConfig {
    let config = tungstenite::protocol::WebSocketConfig::default();
    #[cfg(feature = "permessage-deflate")]
    let config = {
        let mut config = config;
        config.extensions.permessage_deflate = Some(Default::default());
        config
    };
    config
}

async fn accept_subscription(
    listener: &TcpListener,
) -> tokio_tungstenite::WebSocketStream<tokio::net::TcpStream> {
    let (socket, _) = listener.accept().await.unwrap();
    let mut ws = accept_async_with_config(socket, Some(server_config()))
        .await
        .unwrap();
    assert!(matches!(
        ws.next().await,
        Some(Ok(tungstenite::Message::Text(_)))
    ));
    ws
}

// Keep the peer open to prove errors return immediately, without waiting for EOF.
async fn receive_message<T: serde::de::DeserializeOwned + Send + 'static>(
    message: tungstenite::Message,
) -> (bpx_api_client::Result<()>, Option<T>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .build()
        .unwrap();
    let (tx, mut rx) = mpsc::channel(1);
    let peer = async {
        let mut ws = accept_subscription(&listener).await;
        ws.send(message).await.unwrap();
        ws
    };
    let (ws, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe("bookTicker.SOL_USDC", tx))
    })
    .await
    .expect("subscription should finish while the peer is still open");
    drop(ws);
    (result, rx.try_recv().ok())
}

#[tokio::test]
async fn server_rejection_returns_its_payload_without_waiting_for_close() {
    let error = json!({"code": "UNAUTHORIZED", "message": "invalid signature"});
    let (result, event) = receive_message::<Value>(tungstenite::Message::text(
        json!({"error": error}).to_string(),
    ))
    .await;
    match result {
        Err(Error::WebSocketApiError(payload)) => assert_eq!(*payload, error),
        other => panic!("expected server error, got {other:?}"),
    }
    assert!(event.is_none());
}

// Undecodable messages stay non-fatal, as decided upstream in #72.
#[tokio::test]
async fn malformed_json_and_incompatible_events_are_skipped() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .build()
        .unwrap();
    let (tx, mut rx) = mpsc::channel::<u64>(4);
    let peer = async {
        let mut ws = accept_subscription(&listener).await;
        for text in [
            "{",
            r#"{"data":{"unexpected":"event"}}"#,
            r#"{"error":null,"data":7}"#,
        ] {
            ws.send(tungstenite::Message::text(text)).await.unwrap();
        }
        ws.close(None).await.unwrap();
    };
    let (_, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe("bookTicker.SOL_USDC", tx))
    })
    .await
    .expect("subscription should survive undecodable messages");
    result.unwrap();
    assert_eq!(rx.try_recv().unwrap(), 7);
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn unknown_order_variants_are_delivered_to_the_receiver() {
    use base64ct::{Base64, Encoding};
    use bpx_api_client::types::order::{OrderStatus, OrderUpdate, OrderUpdateType};

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .secret(Base64::encode_string(&[7; 32]))
        .build()
        .unwrap();
    let (tx, mut rx) = mpsc::channel::<OrderUpdate>(1);
    let peer = async {
        let mut ws = accept_subscription(&listener).await;
        // Same forward-compatible order shape covered by upstream #119.
        let data = json!({
            "E": 1748288615134547_i64, "O": "USER", "S": "Ask",
            "T": 1748288615133255_i64, "V": "RejectTaker", "X": "SomeFutureStatus",
            "Z": "0", "e": "someFutureEvent", "f": "GTC",
            "i": "114575842681290753", "o": "LIMIT", "q": "20.03",
            "s": "SOL_USDC", "z": "0"
        });
        ws.send(tungstenite::Message::text(
            json!({"data": data}).to_string(),
        ))
        .await
        .unwrap();
        ws.close(None).await.unwrap();
    };
    let (_, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe("account.orderUpdate", tx))
    })
    .await
    .expect("unknown variants should not stop the subscription");
    result.unwrap();
    let update = rx.try_recv().expect("unknown variants should be delivered");
    assert_eq!(
        update.event_type,
        OrderUpdateType::Unknown("someFutureEvent".into())
    );
    assert_eq!(
        update.order_status,
        OrderStatus::Unknown("SomeFutureStatus".into())
    );
}

#[tokio::test]
async fn abrupt_disconnect_after_subscription_returns_a_transport_error() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .build()
        .unwrap();
    let (tx, _rx) = mpsc::channel::<Value>(1);
    let peer = async { drop(accept_subscription(&listener).await) };
    let (_, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe("bookTicker.SOL_USDC", tx))
    })
    .await
    .expect("disconnect should finish");
    assert!(matches!(result, Err(Error::WebSocket(_))), "{result:?}");
}

#[tokio::test]
async fn close_status_distinguishes_normal_completion_from_failure() {
    use tungstenite::protocol::{CloseFrame, frame::coding::CloseCode};

    for code in [
        CloseCode::Normal,
        CloseCode::Away,
        CloseCode::Policy,
        CloseCode::Error,
    ] {
        let (result, _) = receive_message::<Value>(tungstenite::Message::Close(Some(CloseFrame {
            code,
            reason: "server closing".into(),
        })))
        .await;
        if matches!(code, CloseCode::Normal | CloseCode::Away) {
            result.unwrap();
        } else {
            match result {
                Err(Error::WebSocketClosed {
                    code: actual,
                    reason,
                }) => {
                    assert_eq!(actual, u16::from(code));
                    assert_eq!(&*reason, "server closing");
                }
                other => panic!("expected close error, got {other:?}"),
            }
        }
    }
}

#[tokio::test]
async fn private_stream_without_credentials_fails_before_connecting() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .build()
        .unwrap();
    let (tx, _rx) = mpsc::channel::<Value>(1);
    let result = timeout(
        TEST_TIMEOUT,
        client.subscribe_multiple(&["bookTicker.SOL_USDC", "account.orderUpdate"], tx),
    )
    .await
    .unwrap();
    assert!(matches!(result, Err(Error::NotAuthenticated)), "{result:?}");
    assert!(
        timeout(Duration::from_millis(50), listener.accept())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn private_signature_is_generated_after_a_delayed_handshake() {
    use base64ct::{Base64, Encoding};
    use ed25519_dalek::{Signature, SigningKey};
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    };
    let key = SigningKey::from_bytes(&[7; 32]);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = BpxClient::builder()
        .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
        .secret(Base64::encode_string(&key.to_bytes()))
        .build()
        .unwrap();
    let streams = ["bookTicker.SOL_USDC", "account.orderUpdate"];
    let (tx, _rx) = mpsc::channel::<Value>(1);
    let peer = async {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        let handshake_start = now();
        let mut ws = accept_async_with_config(socket, Some(server_config()))
            .await
            .unwrap();
        let text = ws.next().await.unwrap().unwrap().into_text().unwrap();
        let message: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(message["method"], "SUBSCRIBE");
        assert_eq!(message["params"], json!(streams));
        let auth = message["signature"].as_array().unwrap();
        let timestamp = auth[2].as_str().unwrap().parse::<u64>().unwrap();
        assert!(
            timestamp >= handshake_start,
            "signature predates the handshake"
        );
        assert!(timestamp <= now());
        assert_eq!(
            auth[0],
            Base64::encode_string(&key.verifying_key().to_bytes())
        );
        assert_eq!(auth[3], "5000");
        let bytes = Base64::decode_vec(auth[1].as_str().unwrap()).unwrap();
        let signature = Signature::from_slice(&bytes).unwrap();
        key.verifying_key()
            .verify_strict(
                format!("instruction=subscribe&timestamp={timestamp}&window=5000").as_bytes(),
                &signature,
            )
            .unwrap();
        ws.close(None).await.unwrap();
    };
    let (_, result) = timeout(TEST_TIMEOUT, async {
        tokio::join!(peer, client.subscribe_multiple(&streams, tx))
    })
    .await
    .expect("private subscription should finish");
    result.unwrap();
}

#[tokio::test]
async fn dropping_receiver_stops_idle_and_backpressured_subscriptions() {
    for messages in [0, 2] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = BpxClient::builder()
            .ws_url(format!("ws://{}", listener.local_addr().unwrap()))
            .build()
            .unwrap();
        let (tx, rx) = mpsc::channel::<Value>(1);
        let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
        let peer = async {
            let mut ws = accept_subscription(&listener).await;
            for _ in 0..messages {
                ws.send(tungstenite::Message::text(r#"{"data":1}"#))
                    .await
                    .unwrap();
            }
            ready_tx.send(()).unwrap();
            ws
        };
        let consumer = async {
            ready_rx.await.unwrap();
            if messages > 0 {
                // Ensure the channel is full before dropping its receiver.
                while rx.is_empty() {
                    tokio::task::yield_now().await;
                }
            }
            drop(rx);
        };
        let (ws, (), result) = timeout(TEST_TIMEOUT, async {
            tokio::join!(peer, consumer, client.subscribe("bookTicker.SOL_USDC", tx))
        })
        .await
        .expect("dropping receiver should stop the subscription");
        result.unwrap();
        drop(ws);
    }
}
