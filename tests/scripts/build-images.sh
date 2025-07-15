#!/usr/bin/env bash

set -euo pipefail

./run-bakery bake image tryboot-pi4
./run-bakery bake image tryboot

sudo rm -rf .rugix

./run-bakery bake image u-boot
./run-bakery bake image u-boot-armhf

sudo rm -rf .rugix