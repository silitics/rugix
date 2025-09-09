use std::path::Path;

use reportify::{Report, ResultExt};
use xscript::{cmd_os, read_str, run, ParentEnv, Run};

pub fn is_dir(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_dir()
}

reportify::new_whatever_type! {
    DiskError
}

/// The `sfdisk` executable.
const SFDISK: &str = "/usr/sbin/sfdisk";

/// Returns the disk id of the provided image or device.
pub fn get_disk_id(path: impl AsRef<Path>) -> Result<String, Report<DiskError>> {
    fn _disk_id(path: &Path) -> Result<String, Report<DiskError>> {
        let disk_id = read_str!([SFDISK, "--disk-id", path])
            .whatever("unable to retrieve disk id")
            .with_info(|_| format!("disk: {path:?}"))?;
        if let Some(dos_id) = disk_id.strip_prefix("0x") {
            Ok(dos_id.to_owned())
        } else {
            Ok(disk_id)
        }
    }
    _disk_id(path.as_ref())
}

/// The `mkfs.ext4` executable.
const MKFS_ETX4: &str = "/usr/sbin/mkfs.ext4";
/// The `mkfs.vfat` executable.
const MKFS_VFAT: &str = "/usr/sbin/mkfs.vfat";

/// Formats a boot partition with FAT32.
pub fn mkfs_vfat(dev: impl AsRef<Path>, label: impl AsRef<str>) -> Result<(), Report<DiskError>> {
    run!([MKFS_VFAT, "-n", label.as_ref(), dev.as_ref(),])
        .whatever("unable to create FAT32 filesystem")?;
    Ok(())
}

/// Formats a system partition with EXT4.
pub fn mkfs_ext4(
    dev: impl AsRef<Path>,
    label: impl AsRef<str>,
    additional_options: &[String],
) -> Result<(), Report<DiskError>> {
    let mut cmd = cmd_os!(MKFS_ETX4, "-F", "-L", label.as_ref(), dev.as_ref());
    cmd.extend_args(additional_options);
    ParentEnv
        .run(cmd)
        .whatever("unable to create ETX4 filesystem")?;
    Ok(())
}
