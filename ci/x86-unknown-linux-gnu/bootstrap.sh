#!/bin/sh

harfbuzz="harfbuzz-2.6.4"

dpkg --add-architecture x86_64
apt-get update
apt-get install --yes \
    g++ \
    gcc \
    libcairo2-dev
    libfreetype6-dev \
    libfreetype6-dev \
    libglib2.0-dev \
    libgraphite2-dev \
    make \
    tar \
    wget

wget "https://www.freedesktop.org/software/harfbuzz/release/$harfbuzz.tar.xz"
tar -xvf "$harfbuzz.tar.xz"
cd "$harfbuzz"
./configure
make
make install
