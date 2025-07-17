#!/bin/bash
set -euo pipefail

# Step 1: Determine boot/system partitions based on system info and table type
system_info=$(rugix-ctrl system info --json)
root_device=$(findmnt -n -o SOURCE --target /run/rugix/mounts/system)
table_type=$(lsblk -no PTTYPE "$root_device")
active_group=$(echo "$system_info" | jq -r ".boot.activeGroup")

if [ "$active_group" = "a" ]; then
    if [ "$table_type" = "gpt" ]; then
        boot_partition="3"
        system_partition="5"
    else
        boot_partition="3"
        system_partition="6"
    fi
else
    if [ "$table_type" = "gpt" ]; then
        boot_partition="2"
        system_partition="4"
    else
        boot_partition="2"
        system_partition="5"
    fi
fi

# Step 2: Resolve base device and interpolate partition paths
base_device="/dev/$(lsblk -no PKNAME "$root_device")"

if echo "$base_device" | grep -q '[0-9]$'; then
    boot_device="${base_device}p${boot_partition}"
    system_device="${base_device}p${system_partition}"
else
    boot_device="${base_device}${boot_partition}"
    system_device="${base_device}${system_partition}"
fi

# Step 3: Create temporary mount points
boot_mount=$(mktemp -d)
system_mount=$(mktemp -d)

cleanup() {
    echo "Cleaning up..."
    umount -q "$boot_mount" || true
    umount -q "$system_mount" || true
    rm -rf "$boot_mount" "$system_mount"
}
trap cleanup EXIT

# Step 4: Create new FAT32 filesystem on boot partition
echo "Creating FAT32 filesystem on $boot_device..."
mkfs.vfat -F32 -n BOOT "$boot_device"

# Step 5: Mount system and boot partitions
echo "Mounting $system_device to $system_mount..."
mount "$system_device" "$system_mount"

echo "Mounting $boot_device to $boot_mount..."
mount "$boot_device" "$boot_mount"

# Step 6: Copy /boot contents
echo "Copying /boot contents from system partition to boot partition..."
cp -a "$system_mount/boot/." "$boot_mount/"

echo "Boot content copied successfully."