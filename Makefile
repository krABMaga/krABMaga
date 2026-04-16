SHELL := /bin/bash

CARGO ?= cargo

.DEFAULT_GOAL := help

.PHONY: help build check check-vis check-wasm check-mpi check-bayesian check-parallel check-all \
	test test-vis clippy clippy-vis fmt fmt-check doc clean ci ci-vis

help:
	@echo "Common commands:"
	@echo "  make build          Build release artifacts"
	@echo "  make check          Check default build"
	@echo "  make check-vis      Check with visualization feature"
	@echo "  make check-wasm     Check with visualization_wasm feature"
	@echo "  make check-mpi      Check with distributed_mpi feature"
	@echo "  make check-bayesian Check with bayesian feature"
	@echo "  make check-parallel Check with parallel feature"
	@echo "  make check-all      Run common feature checks"
	@echo "  make test           Run tests in release mode"
	@echo "  make test-vis       Run tests with visualization feature"
	@echo "  make clippy         Run clippy for all targets"
	@echo "  make clippy-vis     Run clippy with visualization feature"
	@echo "  make fmt            Format all Rust files"
	@echo "  make fmt-check      Check formatting"
	@echo "  make doc            Build docs (no deps, bayesian feature)"
	@echo "  make clean          Clean target artifacts"
	@echo "  make ci             Local CI subset (fmt/check/test/clippy)"
	@echo "  make ci-vis         Local visualization subset"
	@echo "  make run            Run default binary in release mode"
	@echo "  make run-bin name=bin_name  Run specified binary in release mode

build:
	$(CARGO) build --release

check:
	$(CARGO) check --release

check-vis:
	$(CARGO) check --release --features visualization

check-wasm:
	$(CARGO) check --release --features visualization_wasm --target wasm32-unknown-unknown

check-mpi:
	$(CARGO) check --release --features distributed_mpi

check-bayesian:
	$(CARGO) check --release --features bayesian

check-parallel:
	$(CARGO) check --release --features parallel

check-all: check check-vis check-mpi check-bayesian check-parallel

test:
	$(CARGO) test --release

test-vis:
	$(CARGO) test --release --features visualization

clippy:
	$(CARGO) clippy --all-targets

clippy-vis:
	$(CARGO) clippy --all-targets --features visualization

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

doc:
	$(CARGO) doc --no-deps --features bayesian

clean:
	$(CARGO) clean

run:
	$(CARGO) run --release

run-bin:
	$(CARGO) run --release --bin $(name)

ci: fmt-check check test clippy

ci-vis: check-vis clippy-vis
