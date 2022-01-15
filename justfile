init:
    npm install
    git config --local core.hooksPath .githooks || echo 'Could not set git hooks'

lint: fmt clippy check-license-headers

test:
    cargo test

build:
    cargo build --release

fmt:
    rustfmt crates/**/src/*.rs

clippy:
    cargo clippy

check-license-headers:
    for rust_file in $(ls crates/**/src/*.rs); do \
        ./scripts/check-license-header.sh "$rust_file"; \
    done
