#!/bin/sh
set -ex

if [ $TRAVIS_OS_NAME = 'osx' ]
then
    export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
    export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
    export DEP_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
    export PKG_CONFIG_PATH=/usr/local/opt/icu4c/lib/pkgconfig
fi

rustup self update
. $HOME/.cargo/env
cargo build --release --target $TARGET

