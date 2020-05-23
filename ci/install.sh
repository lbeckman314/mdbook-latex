#!/bin/sh
set -ex

pwd
sudo apt-get update
sudo apt-get install --yes \
    curl \
    gcc \
    g++ \
    libfontconfig1-dev \
    libgraphite2-dev \
    libharfbuzz-dev \
    libicu-dev \
    libssl-dev \
    zlib1g-dev

rustup self update
. $HOME/.cargo/env
cargo build
