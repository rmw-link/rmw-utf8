#!/usr/bin/env bash

set -e

_DIR=$(dirname $(realpath "$0"))

cd $_DIR
wget -c https://dumps.wikimedia.org/zhwiki/20210620/zhwiki-20210620-pages-articles-multistream.xml.bz2
