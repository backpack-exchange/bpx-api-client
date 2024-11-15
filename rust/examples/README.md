# Backpack Exchange API Examples

This repository demonstrates how to interact with Backpack Exchange’s APIs.

## Configuration

Set the following environment variables:

- `BASE_URL` (optional, default: `https://api.backpack.exchange`)
- `WS_URL` (optional, default: `wss://ws.backpack.exchange`)
- `SECRET` (required): Your API secret key.

## Running the Examples

- To retrieve all the open orders, run: `just orders`
- To subscribe to the RFQ private stream, run: `just rfq`
