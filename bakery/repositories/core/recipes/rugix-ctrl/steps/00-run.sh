#!/bin/bash

set -euo pipefail

if [ "${RECIPE_PARAM_USE_MUSL}" = "true" ]; then
    case "${RUGIX_ARCH}" in
        "amd64")
            TARGET="x86_64-unknown-linux-musl"
            ;;
        "arm64")
            TARGET="aarch64-unknown-linux-musl"
            ;;
        "armv7")
            TARGET="armv7-unknown-linux-musleabihf"
            ;;
        "armhf")
            TARGET="arm-unknown-linux-musleabihf"
            ;;
        "arm")
            TARGET="arm-unknown-linux-musleabi"
            ;;
        *)
            echo "Unsupported architecture '${RUGIX_ARCH}' (MUSL)."
            exit 1
    esac
else
    case "${RUGIX_ARCH}" in
        "amd64")
            TARGET="x86_64-unknown-linux-gnu"
            ;;
        "arm64")
            TARGET="aarch64-unknown-linux-gnu"
            ;;
        "armv7")
            TARGET="armv7-unknown-linux-gnueabihf"
            ;;
        "armhf")
            TARGET="arm-unknown-linux-gnueabihf"
            ;;
        "arm")
            TARGET="arm-unknown-linux-gnueabi"
            ;;
        *)
            echo "Unsupported architecture '${RUGIX_ARCH}' (GNU)."
            exit 1
    esac
fi

cp "/usr/share/rugix/binaries/${TARGET}/rugix-ctrl" "${RUGIX_ROOT_DIR}/usr/bin"

if [ "${RECIPE_PARAM_RUGIX_ADMIN}" = "true" ]; then
    cp "/usr/share/rugix/binaries/${TARGET}/rugix-admin" "${RUGIX_ROOT_DIR}/usr/bin"
fi
