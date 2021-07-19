#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

# apt-get install -y libbz2-dev libreadline-dev libsqlite3-dev
if ! [ -x "$(command -v twint)" ]; then
pip3 install --user --upgrade git+https://github.com/twintproject/twint.git@origin/master#egg=twint
asdf reshim
fi



