init:
    git submodule update --init

init-dev: init
    npm install
    git config --local core.hooksPath .githooks || echo 'Could not set git hooks'

dev-init: init-dev

lint: fmt check-license-headers clippy

test:
    cargo test

build:
    cargo build --release

install: build
    cargo install --path crates/cli/

fmt:
    rustfmt crates/**/src/*.rs
    rustfmt crates/**/src/**/*.rs

clippy:
    cargo clippy

check-license-headers:
    ./scripts/check-license-header-all-files.sh

check-runtime-deps:
    ./scripts/check-runtime-deps.sh
