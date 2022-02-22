SHELL := bash
.ONESHELL:
	.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
	MAKEFLAGS += --warn-undefined-variables
	MAKEFLAGS += --no-builtin-rules

install: build check-runtime-deps
	cargo install --path crates/cli/
.PHONY: install

build: init
	cargo build --release
.PHONY: build

clean:
	cargo clean
.PHONY: clean

init:
	git submodule update --init
.PHONY: init

init-dev: init
	git config --local core.hooksPath .githooks || echo 'Could not set git hooks'
.PHONY: init-dev

dev-init: init-dev
.PHONY: dev-init

lint: fmt check-license-headers
	cargo clippy --fix --allow-staged
.PHONY: lint

lint-ci: fmt check-license-headers
	cargo clippy
.PHONY: lint

test:
	cargo test
.PHONY: test

fmt:
	rustfmt crates/**/src/*.rs
	rustfmt crates/**/src/**/*.rs
.PHONY: fmt

check-license-headers:
	./scripts/check-license-header-all-files.sh
.PHONY: check-license-headers

check-runtime-deps:
	./scripts/check-runtime-deps.sh
.PHONY: check-runtime-deps

release:
	./scripts/release.sh
.PHONY: release
