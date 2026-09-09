//! Common functionality shared between Rugix Bakery and Rugix Ctrl.
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use boot::grub::grub_envblk_decode;
use boot::grub::grub_envblk_encode;
use reportify::Report;
use reportify::ResultExt;

use crate::boot::uboot::UBootEnv;

pub mod boot;
pub mod devices;
pub mod disk;
#[cfg(target_os = "linux")]
pub mod fsutils;
#[cfg(target_os = "linux")]
pub mod img_extract;
pub mod maybe_compressed;
#[cfg(target_os = "linux")]
pub mod mount;
pub mod partitions;
pub mod path;
pub mod pipe;
pub mod slots;
pub mod stream_hasher;
pub mod utils;

reportify::new_whatever_type! {
    pub BootPatchError
}

pub fn grub_patch_env(
    boot_dir: impl AsRef<Path>,
    root: impl AsRef<str>,
) -> Result<(), Report<BootPatchError>> {
    grub_patch_env_with_init_overwrite(boot_dir, root, true)
}

/// Patches `boot.grubenv` to use the given root device and optionally overwrite its init
/// process with Rugix Ctrl.
pub fn grub_patch_env_with_init_overwrite(
    boot_dir: impl AsRef<Path>,
    root: impl AsRef<str>,
    overwrite_init: bool,
) -> Result<(), Report<BootPatchError>> {
    const RUGIX_BOOTARGS: &str = "rugpi_bootargs";
    let boot_env_path = boot_dir.as_ref().join("boot.grubenv");
    let mut init_arguments = Vec::new();
    if overwrite_init {
        init_arguments.push(format!("init={RUGIX_CTRL_INIT}"));
    } else if boot_env_path.exists() {
        let existing_env = grub_envblk_decode(
            &fs::read_to_string(&boot_env_path)
                .whatever("failed to read existing Grub environment")?,
        )
        .whatever("failed to decode existing Grub environment")?;
        if let Some(existing_bootargs) = existing_env.get(RUGIX_BOOTARGS) {
            init_arguments.extend(
                existing_bootargs
                    .split_ascii_whitespace()
                    .filter(|argument| argument.starts_with("init="))
                    .map(str::to_owned),
            );
        }
    }
    let mut bootargs = vec!["ro".to_owned()];
    bootargs.extend(init_arguments);
    bootargs.push(format!("root=PARTUUID={}", root.as_ref()));
    let mut env = HashMap::new();
    env.insert(RUGIX_BOOTARGS.to_owned(), bootargs.join(" "));
    let encoded = grub_envblk_encode(&env).whatever("failed to encode boot environment")?;
    std::fs::write(boot_env_path, encoded.as_bytes())
        .whatever("failed to write Grub environment file")?;
    Ok(())
}

/// Patches `cmdline.txt` to use the given root device and `rugix-ctrl` as init process.
pub fn rpi_patch_boot(
    path: impl AsRef<Path>,
    root: impl AsRef<str>,
) -> Result<(), Report<BootPatchError>> {
    rpi_patch_boot_with_init_overwrite(path, root, true)
}

/// Patches `cmdline.txt` to use the given root device and optionally overwrite its init
/// process with Rugix Ctrl.
pub fn rpi_patch_boot_with_init_overwrite(
    path: impl AsRef<Path>,
    root: impl AsRef<str>,
    overwrite_init: bool,
) -> Result<(), Report<BootPatchError>> {
    fn _patch_cmdline(
        path: &Path,
        root: &str,
        overwrite_init: bool,
    ) -> Result<(), Report<BootPatchError>> {
        let cmdline_path = path.join("cmdline.txt");
        let cmdline = fs::read_to_string(&cmdline_path)
            .whatever("unable to read `cmdline.txt` from boot partition")?;
        let mut parts = cmdline
            .split_ascii_whitespace()
            .filter(|part| {
                !part.starts_with("root=")
                    && (!overwrite_init || !part.starts_with("init="))
                    && !part.starts_with("panic")
                    && *part != "quiet"
            })
            .map(str::to_owned)
            .collect::<Vec<_>>();
        parts.push("panic=60".to_owned());
        parts.push(format!("root={root}"));
        if overwrite_init {
            parts.push(format!("init={RUGIX_CTRL_INIT}"));
        }
        let cmdline_value = parts.join(" ");
        fs::write(&cmdline_path, &cmdline_value)
            .whatever("unable to write `cmdline.txt` to boot partition")?;
        let boot_env_path = path.join("boot.env");
        let mut env = if boot_env_path.exists() {
            UBootEnv::load(&boot_env_path).whatever("unable to load U-Boot environment")?
        } else {
            UBootEnv::new()
        };
        env.set("bootargs", &cmdline_value);
        env.save(boot_env_path)
            .whatever("unable to save U-Boot environment")?;
        Ok(())
    }
    _patch_cmdline(path.as_ref(), root.as_ref(), overwrite_init)
}

/// Runs a closure on drop.
pub struct DropGuard<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> DropGuard<F> {
    /// Construct a new [`DropGuard`] with the given closure.
    pub fn new(closure: F) -> Self {
        Self(Some(closure))
    }

    /// Do not run the closure on drop.
    pub fn disable(&mut self) {
        self.0.take();
    }
}

impl<F: FnOnce()> Drop for DropGuard<F> {
    fn drop(&mut self) {
        if let Some(closure) = self.0.take() {
            closure()
        }
    }
}

const RUGIX_CTRL_INIT: &str = "/usr/bin/rugix-ctrl";

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tempfile::tempdir;

    use super::grub_patch_env;
    use super::grub_patch_env_with_init_overwrite;
    use super::rpi_patch_boot;
    use super::rpi_patch_boot_with_init_overwrite;
    use crate::boot::grub::grub_envblk_decode;
    use crate::boot::grub::grub_envblk_encode;
    use crate::boot::uboot::UBootEnv;

    /// Verifies that the default GRUB patch retains its legacy output.
    #[test]
    fn grub_boot_patch_overwrites_init_by_default() {
        let directory = tempdir().unwrap();
        let mut incoming_env = HashMap::new();
        incoming_env.insert(
            "rugpi_bootargs".to_owned(),
            "ro init=/sbin/init root=PARTUUID=old console=ttyS0".to_owned(),
        );
        std::fs::write(
            directory.path().join("boot.grubenv"),
            grub_envblk_encode(&incoming_env).unwrap(),
        )
        .unwrap();

        grub_patch_env(directory.path(), "new").unwrap();

        let updated_env = grub_envblk_decode(
            &std::fs::read_to_string(directory.path().join("boot.grubenv")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            updated_env.get("rugpi_bootargs").map(String::as_str),
            Some("ro init=/usr/bin/rugix-ctrl root=PARTUUID=new")
        );
    }

    /// Verifies that the GRUB opt-out retains the incoming `init`.
    #[test]
    fn grub_boot_patch_preserves_init_when_no_replacement_is_requested() {
        let directory = tempdir().unwrap();
        let mut incoming_env = HashMap::new();
        incoming_env.insert(
            "rugpi_bootargs".to_owned(),
            "rw root=PARTUUID=old init=/sbin/init splash".to_owned(),
        );
        std::fs::write(
            directory.path().join("boot.grubenv"),
            grub_envblk_encode(&incoming_env).unwrap(),
        )
        .unwrap();

        grub_patch_env_with_init_overwrite(directory.path(), "new", false).unwrap();

        let updated_env = grub_envblk_decode(
            &std::fs::read_to_string(directory.path().join("boot.grubenv")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            updated_env.get("rugpi_bootargs").map(String::as_str),
            Some("ro init=/sbin/init root=PARTUUID=new")
        );
    }

    /// Verifies that the legacy Raspberry Pi patch API continues to replace `init`.
    #[test]
    fn rpi_boot_patch_overwrites_init_by_default() {
        let directory = tempdir().unwrap();
        std::fs::write(
            directory.path().join("cmdline.txt"),
            "console=tty1 root=/dev/old init=/sbin/init quiet panic=10",
        )
        .unwrap();

        rpi_patch_boot(directory.path(), "PARTUUID=new").unwrap();

        let expected = "console=tty1 panic=60 root=PARTUUID=new init=/usr/bin/rugix-ctrl";
        assert_eq!(
            std::fs::read_to_string(directory.path().join("cmdline.txt")).unwrap(),
            expected
        );
        assert_eq!(
            UBootEnv::load(directory.path().join("boot.env"))
                .unwrap()
                .get("bootargs"),
            Some(expected)
        );
    }

    /// Verifies that the configurable Raspberry Pi patch API retains the incoming `init`.
    #[test]
    fn rpi_boot_patch_preserves_init_when_no_replacement_is_requested() {
        let directory = tempdir().unwrap();
        std::fs::write(
            directory.path().join("cmdline.txt"),
            "console=tty1 root=/dev/old init=/sbin/init quiet panic=10",
        )
        .unwrap();

        rpi_patch_boot_with_init_overwrite(directory.path(), "PARTUUID=new", false).unwrap();

        let expected = "console=tty1 init=/sbin/init panic=60 root=PARTUUID=new";
        assert_eq!(
            std::fs::read_to_string(directory.path().join("cmdline.txt")).unwrap(),
            expected
        );
        assert_eq!(
            UBootEnv::load(directory.path().join("boot.env"))
                .unwrap()
                .get("bootargs"),
            Some(expected)
        );
    }
}
