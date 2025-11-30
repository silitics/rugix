#!/usr/bin/env bash
set -euo pipefail

if [ "${RECIPE_PARAM_BOOTLOADER_VARIANT}" == "" ]; then
    echo "[armbian-setup] Bootloader variant not set!"
    exit 1
fi

echo "[armbian-setup] Disabling Armbian interactive first boot scripts (firstrun/firstlogin) if present"

# Mask possible systemd units to ensure they never trigger.
for svc in armbian-firstrun.service armbian-firstlogin.service; do
  for dir in /lib/systemd/system /usr/lib/systemd/system; do
    if [ -f "$dir/$svc" ]; then
      mkdir -p /etc/systemd/system
      ln -sf /dev/null "/etc/systemd/system/$svc"
      echo "[armbian-setup] Masked $svc"
    fi
  done
done

# Remove trigger/state files that would spawn interactive setup.
rm -f /root/.not_logged_in_yet /etc/armbian-first-run || true

if [ "${RECIPE_PARAM_WITH_SQUASHFS}" == "true" ]; then
    apt-get install -y initramfs-tools
    echo "squashfs" > "/usr/share/initramfs-tools/modules.d/rugix"
fi

# copy boot configuration files
BOOT_DIR="${RUGIX_LAYER_DIR}/roots/boot/"

mkdir -p "/boot"
mkdir -p "${BOOT_DIR}"

bootloader_variant="${RECIPE_PARAM_BOOTLOADER_VARIANT}"
echo "[armbian-setup] Using bootloader variant: ${bootloader_variant}"

# Create U-Boot script for first stage bootloader
mkimage -C none -A arm -T script -d "${RECIPE_DIR}/files/${bootloader_variant}/first-stage.bootloader.sh" /boot/first-stage.boot.scr
cp /boot/first-stage.boot.scr "/${BOOT_DIR}/first-stage.boot.scr"

# Create U-Boot script for second stage bootloader
mkimage -C none -A arm -T script -d "${RECIPE_DIR}/files/${bootloader_variant}/second-stage.bootloader.sh" /boot/boot.scr
cp /boot/boot.scr "/${BOOT_DIR}/boot.scr"

# Create rugix-ctrl system config
mkdir -p /etc/rugix
cat << EOF >> /etc/rugix/system.toml
[boot-flow]
type = "armbian-uboot"
EOF