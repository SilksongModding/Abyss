.PHONY: help build release run install uninstall test integration-test clean fmt clippy

## Makefile for Abyss - developer convenience
## Usage: make <target>

help:
	@echo "Available targets:"
	@echo "  make build             # cargo build (debug)"
	@echo "  make release           # cargo build --release"
	@echo "  make run               # run debug binary with args: make run ARGS='detect --quiet'"
	@echo "  make install           # cargo install --path . (installs to ~/.cargo/bin)"
	@echo "  make uninstall         # cargo uninstall abyss"
	@echo "  make test              # run unit tests"
	@echo "  make integration-test  # run integration tests (ENABLE_INTEGRATION_TESTS=1)"
	@echo "  make fmt               # run rustfmt"
	@echo "  make clippy            # run clippy"
	@echo "  make clean             # cargo clean"

build:
	cargo build

release:
	cargo build --release

run:
	# Example: make run ARGS='detect --quiet'
	@# If ARGS is empty, print a short hint and do nothing (avoid non-zero exit when run accidentally)
	@if [ -z "${ARGS}" ]; then \
		echo "No ARGS provided. Example: make run ARGS='detect --quiet'"; \
	else \
		cargo run -- ${ARGS}; \
	fi

install:
	cargo install --path .

uninstall:
	cargo uninstall abyss || true

test:
	cargo test

integration-test:
	# This target will run the optional integration test that exercises steamlocate.
	# It only runs the test when ENABLE_INTEGRATION_TESTS=1 is set.
	ENABLE_INTEGRATION_TESTS=1 cargo test --test integration

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

clean:
	cargo clean
