#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

if ! hash cargo-watch 2>/dev/null; then
cargo install cargo-watch
fi

RUST_BACKTRACE=1 cargo +nightly watch -cx run

