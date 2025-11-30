##################################################################
# Harcoded values copied from Armbian U-Boot script for Asus Tinkerboard

setenv ramdisk_addr_r "0x21000000"
setenv load_addr "0x39000000"
setenv overlay_error "false"
# default values
setenv rootdev "/dev/mmcblk0p1"
setenv verbosity "1"
setenv console "both"
setenv bootlogo "false"
setenv rootfstype "ext4"
setenv docker_optimizations "on"
setenv earlycon "off"

# ensure that devnum is set
# should set be initialized by the bootloader to the first "bootable" device
if test "${devnum}" = ""; then
    setenv devnum 0
	echo "WARN: variable devnum is not set, defaulting to ${devnum}"
else
    echo "Using devnum: ${devnum}"
fi

# print partition list for whole device
echo "Partition list for mmc 0:"
part list mmc 0

echo "Partition list for mmc 1:"
part list mmc 1

##################################################################
# Rugix U-Boot First Stage
##################################################################

echo "== Rugpi U-Boot First Stage =="

if load mmc ${devnum}:1 ${load_addr} bootpart.default.env; then
    env import -c ${load_addr} ${filesize}
fi
if load mmc ${devnum}:1 ${load_addr} boot_spare.env; then
    env import -c ${load_addr} ${filesize}
fi
if test "${bootpart}" = ""; then
    setenv bootpart 2
fi

echo "Boot Spare: ${boot_spare}"
if test "${boot_spare}" = "1"; then
    # emulate: bootpart = 5 - bootpart
    if test "${bootpart}" = "1"; then
        setenv bootpart 4
    elif test "${bootpart}" = "2"; then
        setenv bootpart 3
    elif test "${bootpart}" = "3"; then
        setenv bootpart 2
    elif test "${bootpart}" = "4"; then
        setenv bootpart 1
    else
        echo "bootpart=${bootpart} not remapped (out of valid range for 5 - x)"
    fi

    if load mmc ${devnum}:1 ${load_addr} boot_spare.disabled.env; then
        save mmc ${devnum}:1 ${load_addr} boot_spare.env ${filesize}
    else
        # If loading `boot_spare.disabled.env` fails, simply write an empty file.
        save mmc ${devnum}:1 ${load_addr} boot_spare.env 0
    fi
fi
echo "Bootpart: ${bootpart}"

# Load boot environment and hand off to second boot stage.
if load mmc ${devnum}:${bootpart} ${load_addr} boot.scr; then
    source ${load_addr}
fi

echo "Executing second boot stage failed. Rebooting..."
sleep 10
reset
