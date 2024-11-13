# Backpack Exchange API Examples

This repository demonstrates how to interact with Backpack Exchange’s APIs.

## Configuration

Set the following environment variables:

- `BASE_URL` (optional, default: `https://api.backpack.exchange`)
- `WS_URL` (optional, default: `wss://ws.backpack.exchange`)
- `SECRET` (required): Your API secret key.
- `STREAM` (required): The WebSocket stream to connect to.

## Running the RFQ Example

To subscribe to the `account.rfqUpdate` private stream, run:

```bash
just rfq
````
