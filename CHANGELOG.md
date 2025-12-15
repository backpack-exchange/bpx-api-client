## [0.15.1] - 2025-12-15

### 🐛 Bug Fixes

- Use correct auto_lend parameter ([#90](https://github.com/backpack-exchange/bpx-api-client/issues/90))
## [0.15.0] - 2025-12-15

### 🐛 Bug Fixes

- *(ci)* Remove the incorrect working directory for the CI ([#88](https://github.com/backpack-exchange/bpx-api-client/issues/88))
## [0.14.0] - 2025-12-15

### 🐛 Bug Fixes

- *(rfq)* Correctly add signatures for rfq operations ([#87](https://github.com/backpack-exchange/bpx-api-client/issues/87))
## [0.13.0] - 2025-12-03

### 🐛 Bug Fixes

- Get_markets function ([#85](https://github.com/backpack-exchange/bpx-api-client/issues/85))
## [0.12.0] - 2025-12-03

### 🐛 Bug Fixes

- *(markets)* Remove unnecessary print statement ([#84](https://github.com/backpack-exchange/bpx-api-client/issues/84))
## [0.11.2] - 2025-12-03

### 🚀 Features

- *(rfq)* Add endpoints for refresh, cancel ([#83](https://github.com/backpack-exchange/bpx-api-client/issues/83))
## [0.11.1] - 2025-12-03

### 🚀 Features

- Add MarketType filter to get_markets ([#80](https://github.com/backpack-exchange/bpx-api-client/issues/80))

### ⚡ Performance

- Use depot runners
## [0.11.0] - 2025-12-02

### 🚀 Features

- Add missing fields to PriceFilters ([#81](https://github.com/backpack-exchange/bpx-api-client/issues/81))
## [0.10.6] - 2025-12-01

### ⚙️ Miscellaneous Tasks

- Fix release action CHANGELOG.md tagging
## [0.10.5] - 2025-11-26

### 🐛 Bug Fixes

- Github action

### ⚙️ Miscellaneous Tasks

- Github actions fixes
- Remove gpg signing in release action
- Remove push and publish in release.toml
- Push tags in release action
- Fix action
- Add cargo registry token secret to release action
## [0.10.4] - 2025-11-26

### 🚀 Features

- *(rfq)* Add missing quote fields ([#75](https://github.com/backpack-exchange/bpx-api-client/issues/75))

### ⚙️ Miscellaneous Tasks

- Git-cliff integration ([#76](https://github.com/backpack-exchange/bpx-api-client/issues/76))
- Update cargo-release config to support git-cliff pre-commit hook
- Add release workflow
- Use github hosted runner
- Github action fixes
- *(release)* V0.10.4
## [0.10.3] - 2025-11-26

### 🚀 Features

- Add method verifying_key() which was previously called verifier() and removed in 509c6bf0eb98619a82d139559bbd8bc5131ab2bb ([#74](https://github.com/backpack-exchange/bpx-api-client/issues/74))

### 🐛 Bug Fixes

- Deps

### ⚙️ Miscellaneous Tasks

- Use cargo-release
- Release
## [0.10.2] - 2025-11-22

### 🚀 Features

- Add futures 'get open positions'
- Add type for depth websocket updates
- Add TickerUpdate struct
- Make TickerUpdate fields public
- Add get_funding_interval_rates
- Add serde to rust_decimal, define funding_rate as Decimal
- Add total method to Balance
- Add borrow&lend, margin
- Remove json data
- Use MarginFunction for Future Position
- Add MarkPrices
- Add auto lend & borrow flags to ExecuteOrderPayload
- Add new blockchains
- Update Order & OrderStatus
- Add TriggerBy
- Add MarkPriceUpdate struct
- Add OrderUpdate type
- Add new convert dust api
- Add TradeUpdate struct
- Update ExecuteOrderPayload to match current api
- Allow unauthenticated client, creation of client via builder ([#64](https://github.com/backpack-exchange/bpx-api-client/issues/64))

### 🐛 Bug Fixes

- *(client)* Fix market URL typos
- Various small fixes
- Remove clone
- Remove debug print
- Enable rustls-tls-native-roots feature for websockets
- Comments on OrderBookDepthUpdate struct
- Return new type Asset on "get_assets"
- Use Decimal instead of String in TickerUpdate
- Make fields of OrderBookDepthUpdate public
- Make "reduce_only" an Option
- Use correct type for trigger_by fields
- *(client)* Allow(dead_code) on ws_url when not feature disabled ([#67](https://github.com/backpack-exchange/bpx-api-client/issues/67))
- Return NotAuthenticated error in WS subscribe for private streams if no key configured ([#70](https://github.com/backpack-exchange/bpx-api-client/issues/70))
- Fix URLs not joining base_url to path correctly (contained '//') which was causing 404 ([#71](https://github.com/backpack-exchange/bpx-api-client/issues/71))

### 💼 Other

- Fixes
- Fixes for 'uninlined_format_args' rule

### 🚜 Refactor

- Change OrderBookDepth::last_update_id to be i64 and OrderBookDepthUpdate::first_update_id and last_update_id to be i64 ([#65](https://github.com/backpack-exchange/bpx-api-client/issues/65))

### 📚 Documentation

- Add trailing .

### 🧪 Testing

- Add tests for custom TriggerQuantity Serialize implementation

### ⚙️ Miscellaneous Tasks

- Rename Position -> FuturePosition
- Update to rust 2024 ([#63](https://github.com/backpack-exchange/bpx-api-client/issues/63))
- *(client)* Use path + version to refer to local types crate ([#66](https://github.com/backpack-exchange/bpx-api-client/issues/66))
- Bump version to 0.1.0 ([#68](https://github.com/backpack-exchange/bpx-api-client/issues/68))
- Expose public urls + cleanup ([#69](https://github.com/backpack-exchange/bpx-api-client/issues/69))
- Bump version for depdendency
