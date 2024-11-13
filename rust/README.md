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

- **REST API**: Includes public and private (authenticated) endpoints.
- **WebSocket API**: Offers public and private streams.

The official API documentation is available at [https://docs.backpack.exchange/](https://docs.backpack.exchange/).

## Usage

This project uses [Just](https://github.com/casey/just) to manage various build and development tasks.

To see the available commands, run:

```shell
just
```
