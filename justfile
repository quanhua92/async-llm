set dotenv-required

set dotenv-load := true

install:
    cargo install cargo-watch
    cargo install git-cliff

build:
    cargo build

generate:
    cargo run --example generate

changelog tag:
    git cliff -t {{tag}} -o CHANGELOG.md

run example="$DEFAULT_EXAMPLE":
    cargo run --example {{example}}

dev example="$DEFAULT_EXAMPLE":
    RUST_LOG=debug cargo watch -w src -w tests -w examples -s "cargo run --example {{example}}"

trace example="$DEFAULT_EXAMPLE":
    RUST_LOG=trace cargo watch -w src -w tests -w examples -s "cargo run --example {{example}}"

test:
    cargo watch -d 1 -w src -w tests -w examples -x test

test-one NAME:
    RUST_LOG=debug cargo watch -d 1 -w src -w tests -w examples -s "cargo test {{NAME}}"
