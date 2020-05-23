#!/bin/sh
set -ex

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

#curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup self update
. $HOME/.cargo/env
cargo install mdbook-latex
