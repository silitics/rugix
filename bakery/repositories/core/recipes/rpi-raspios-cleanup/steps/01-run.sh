#!/bin/bash

set -euo pipefail

sed -i '/^auto_initramfs=/d' "${RUGIX_LAYER_DIR}/roots/boot/config.txt"
