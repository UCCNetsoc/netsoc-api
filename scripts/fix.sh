#!/usr/bin/env bash
set -eux

cargo fmt
cargo clippy --color=always --tests --examples --bins --benches --release --all-features --no-deps --fix
