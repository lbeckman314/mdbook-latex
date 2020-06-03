#!/bin/sh
set -ex

if [ $TRAVIS_OS_NAME = 'linux' ]
then
    sudo apt-get update
    sudo apt-get install --yes \
        gcc \
        g++ \
        libfontconfig1-dev \
        libgraphite2-dev \
        libharfbuzz-dev \
        libicu-dev \
        libssl-dev \
        zlib1g-dev

elif [ $TRAVIS_OS_NAME = 'osx' ]
then
    export DEP_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
    export PKG_CONFIG_PATH=/usr/local/opt/icu4c/lib/pkgconfig
    brew install --only-dependencies tectonic
fi

rustup self update
. $HOME/.cargo/env
cargo install mdbook-latex

