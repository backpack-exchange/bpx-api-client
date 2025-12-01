# print options
default:
    @just --list --unsorted

# check formatting, clippy and taplo
check:
    cargo check
    cargo +nightly fmt --all -- --check
    cargo clippy --all-targets --all-features

# automatically fix check warnings
fix:
    cargo +nightly fmt --all
    cargo clippy --allow-dirty --allow-staged --fix

# execute the tests
test:
    cargo test

# build project (debug profile)
build:
    cargo build --all-targets

# build project (release profile)
release:
    cargo build --all-targets --release

# run example
example name:
    cargo run --bin {{name}} -p bpx-api-examples
