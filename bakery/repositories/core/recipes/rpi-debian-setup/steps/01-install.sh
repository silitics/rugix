#!/bin/bash

set -euo pipefail

BOOT_DIR="${RUGIX_LAYER_DIR}/roots/boot"

if [ "${RECIPE_PARAM_WITH_SQUASHFS}" == "true" ]; then
    apt-get install -y initramfs-tools
    echo "squashfs" > "/usr/share/initramfs-tools/modules.d/rugix"
fi

mkdir -p "${BOOT_DIR}"

RELEASE="$(. /etc/os-release && echo "$VERSION_CODENAME")"

install -m 644 "${RECIPE_DIR}/files/raspberrypi.list" "/etc/apt/sources.list.d/"
sed -i "s/RELEASE/$RELEASE/g" "/etc/apt/sources.list.d/raspberrypi.list"

apt-get update -y
apt-get install -y raspberrypi-archive-keyring

if [ "${RECIPE_PARAM_WITH_NONFREE}" == "true" ]; then
    # Make sure that the non-free sources are available.
    sed -i '/main/!b; /non-free/b; s/$/ non-free/' /etc/apt/sources.list
    sed -i '/main/!b; /non-free-firmware/b; s/$/ non-free-firmware/' /etc/apt/sources.list

    apt-get update -y

    if [ "${RECIPE_PARAM_WITH_FIRMWARE}" == "true" ]; then
        echo "Installing nonfree firmware..."
        apt-get install -y \
            firmware-atheros \
            firmware-brcm80211 \
            firmware-libertas \
            firmware-misc-nonfree \
            firmware-realtek
    fi
else
    echo "Without non-free firmware, WiFi cards will likely not work."
fi

apt-get install -y \
    initramfs-tools \
    raspi-firmware \
    linux-image-rpi-v8 \
    linux-headers-rpi-v8 \
    linux-image-rpi-2712 \
    linux-headers-rpi-2712

install -m 644 "${RECIPE_DIR}/files/cmdline.txt" "/boot/firmware/"
install -m 644 "${RECIPE_DIR}/files/config.txt" "/boot/firmware/"

mkdir -p "${BOOT_DIR}"
cp -rp /boot/firmware/* "${BOOT_DIR}"
