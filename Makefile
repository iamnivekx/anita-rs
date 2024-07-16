.DEFAULT_GOAL := help

GIT_TAG ?= $(shell git describe --tags --abbrev=0)

.PHONY: install
install: ## Build and install the binary
	cargo install --path bin --bin anita --force --locked

.PHONY: build
build: ## Build the anita binary into `target` directory.
	cargo build --bin anita