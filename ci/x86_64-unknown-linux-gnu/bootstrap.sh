#!/bin/sh
set -x

apt-get update 
apt-get install --yes \
    gcc \
    g++ \
    libfontconfig1-dev \
    libgraphite2-dev \
    libharfbuzz-dev \
    libicu-dev \
    libssl-dev \
    zlib1g-dev

