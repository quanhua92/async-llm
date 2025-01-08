set dotenv-required

set dotenv-load := true

install:
    cargo install cargo-watch

build:
    cargo build

generate:
    cargo run --example generate

run example="$DEFAULT_EXAMPLE":
    cargo run --example {{example}}

dev example="$DEFAULT_EXAMPLE":
    RUST_LOG=debug cargo watch -w src -w tests -w examples -s "cargo run --example {{example}}"

test:
    cargo watch -d 1 -w src -w tests -w examples -x test

test-one NAME:
    RUST_LOG=debug cargo watch -d 1 -w src -w tests -w examples -s "cargo test {{NAME}}"
