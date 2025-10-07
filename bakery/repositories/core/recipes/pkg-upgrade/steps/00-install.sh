#!/bin/sh

set -eu

if command -v apt-get; then
    apt-get upgrade -y
fi

if command -v apk; then
    apk upgrade
fi

if command -v pacman; then
    pacman -Syu --noconfirm
fi