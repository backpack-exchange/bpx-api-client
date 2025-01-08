[![Build Badge]][build] [![Crates Badge]][crates] [![Docs Badge]][docs] [![License Badge]][license]

[Build Badge]: https://github.com/backpack-exchange/bpx-api-client/actions/workflows/rust.yml/badge.svg
[build]: https://github.com/backpack-exchange/bpx-api-client/actions

[Crates Badge]: https://img.shields.io/crates/v/bpx_api_client.svg
[crates]: https://crates.io/crates/bpx_api_client

[Docs Badge]: https://docs.rs/bpx_api_client/badge.svg
[docs]: https://docs.rs/bpx_api_client

[License Badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[license]: ../LICENSE

# Backpack Exchange API Crate

This crate provides both REST and WebSocket APIs for interacting with the Backpack Exchange:

## Features

- **REST API**: Access public and private (authenticated) endpoints.
- **WebSocket API**: Subscribe to private streams for real-time updates (requires `ws` feature).

The official API documentation is available at [https://docs.backpack.exchange/](https://docs.backpack.exchange/).

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
bpx_api_client = "x.y.z" # Replace with the latest version
```

To enable WebSocket support:

```toml
[dependencies]
bpx_api_client = { version = "x.y.z", features = ["ws"] }
```

## Usage

REST API example:

```rust
use bpx_api_client::{BpxClient, BACKPACK_API_BASE_URL};
use std::env;

#[tokio::main]
async fn main() {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::init(base_url, secret, None)
        .expect("Failed to initialize Backpack API client");

    match client.get_open_orders(Some("SOL_USDC")).await {
        Ok(orders) => println!("Open Orders: {:?}", orders),
        Err(err) => tracing::error!("Error: {:?}", err),
    }
}
```

WebSocket API example:

```rust
use anyhow::Result;
use bpx_api_client::{BpxClient, BACKPACK_API_BASE_URL, BACKPACK_WS_URL};
use bpx_api_types::rfq::RequestForQuote;
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| BACKPACK_API_BASE_URL.to_string());
    let ws_url = env::var("WS_URL").unwrap_or_else(|_| BACKPACK_WS_URL.to_string());
    let secret = env::var("SECRET").expect("Missing SECRET environment variable");

    let client = BpxClient::init_with_ws(base_url, ws_url, &secret, None)?;

    let (tx, mut rx) = mpsc::channel::<RequestForQuote>(100);
    tokio::spawn(async move {
        while let Some(rfq) = rx.recv().await {
            println!("Received RFQ: {:?}", rfq);
        }
    });

    client.subscribe_to_rfqs(tx).await;

    Ok(())
}
```

## Development

This project uses [Just](https://github.com/casey/just) to manage various build and development tasks.

To see the available commands, run:

```shell
just
```

