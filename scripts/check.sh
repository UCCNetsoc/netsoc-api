#!/usr/bin/env bash
set -eux

cargo fmt --check
cargo clippy --color=always --tests --examples --bins --benches --release --all-features --no-deps -- -D warnings
cargo test --release --all-features
