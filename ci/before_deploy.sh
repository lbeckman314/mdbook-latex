#!/bin/sh
set -ex

# This script takes care of building your crate and packaging it for release
main() {
    install
    build_docs
    package
}

install() {
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
}

build_docs() {
    cargo install mdbook
    "$HOME/.cargo/bin/mdbook build docs"
}

package() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cp target/$TARGET/release/mdbook-latex $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main

