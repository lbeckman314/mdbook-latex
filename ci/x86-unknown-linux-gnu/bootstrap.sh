#!/bin/sh
set -x

harfbuzz="harfbuzz-2.6.4"

dpkg --add-architecture x86_64
apt-get update
apt-get install --yes \
    g++ \
    gcc \
    libcairo2-dev \
    libfreetype6-dev \
    libglib2.0-dev \
    libgraphite2-dev \
    libharfbuzz-dev \
    libssl-dev \
    make \
    tar \
    wget \
    xz-utils

