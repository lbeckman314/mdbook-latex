#!/bin/sh
set -x

apt-get update 
apt-get install --yes \
    curl \
    gcc \
    g++ \
    libfontconfig1-dev \
    libgraphite2-dev \
    libharfbuzz-dev \
    libicu-dev \
    libssl-dev \
    zlib1g-dev

curl https://sh.rustup.rs -sSf | sh -s -- -y
. $HOME/.cargo/env
cargo install mdbook-latex

