#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

cargo build --release
#export RAYON_NUM_THREADS=8
target/release/train
mv d.zst ../rmw-utf8/src
