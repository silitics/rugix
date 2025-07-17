#!/bin/bash

set -euo pipefail

BOOT_DIR="${RUGIX_LAYER_DIR}/roots/boot"

rm -rf "${RUGIX_ROOT_DIR}/boot"
cp -rp "${BOOT_DIR}" "${RUGIX_ROOT_DIR}/boot"
