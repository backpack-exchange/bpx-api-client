## [bpx-api-types-v0.10.3] - 2025-11-26

### 🚀 Features

- Add method verifying_key() which was previously called verifier() and removed in 509c6bf0eb98619a82d139559bbd8bc5131ab2bb ([#74](https://github.com/backpack-exchange/bpx-api-client/issues/74))

### ⚙️ Miscellaneous Tasks

- Use cargo-release
- Release
## [0.10.2] - 2025-11-22

### 🐛 Bug Fixes

- Return NotAuthenticated error in WS subscribe for private streams if no key configured ([#70](https://github.com/backpack-exchange/bpx-api-client/issues/70))
- Fix URLs not joining base_url to path correctly (contained '//') which was causing 404 ([#71](https://github.com/backpack-exchange/bpx-api-client/issues/71))

### ⚙️ Miscellaneous Tasks

- Expose public urls + cleanup ([#69](https://github.com/backpack-exchange/bpx-api-client/issues/69))
- Bump version for depdendency
