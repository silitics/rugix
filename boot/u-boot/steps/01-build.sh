#!/bin/bash

set -euo pipefail

mkdir -p out

function build_uboot() {
    local name=$1;
    local arch=$2;
    local config=$3;

    make clean

    case ${arch} in
        "armhf")
            export CROSS_COMPILE=/opt/gcc-13.2.0-nolibc/arm-linux-gnueabi/bin/arm-linux-gnueabi-
            ;;
        "arm64")
            export CROSS_COMPILE=/opt/gcc-13.2.0-nolibc/aarch64-linux/bin/aarch64-linux-
            ;;
    esac

    make ${config}
    make -j$(nproc)

    mv u-boot.bin out/u-boot-${name}.bin
}

build_uboot arm64 arm64 rpi_arm64_rugpi_defconfig

build_uboot armhf-zerow armhf rpi_armhf_zerow_rugpi_defconfig
build_uboot armhf-pi1 armhf rpi_armhf_pi1_rugpi_defconfig
build_uboot armhf-pi2 armhf rpi_armhf_pi2_rugpi_defconfig
build_uboot armhf-pi3 armhf rpi_armhf_pi3_rugpi_defconfig
