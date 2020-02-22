#!/bin/sh
set -ex

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

