//! Common functionality shared between Rugix Bakery and Rugix Ctrl.
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use boot::grub::grub_envblk_encode;
use reportify::{Report, ResultExt};

use crate::boot::uboot::UBootEnv;

pub mod boot;
pub mod devices;
pub mod disk;
#[cfg(target_os = "linux")]
pub mod fsutils;
pub mod loop_dev;
pub mod maybe_compressed;
#[cfg(target_os = "linux")]
pub mod mount;
pub mod partitions;
pub mod pipe;
pub mod slots;
pub mod stream_hasher;
pub mod utils;

reportify::new_whatever_type! {
    BootPatchError
}

pub fn grub_patch_env(
    boot_dir: impl AsRef<Path>,
    root: impl AsRef<str>,
) -> Result<(), Report<BootPatchError>> {
    const RUGIX_BOOTARGS: &str = "rugpi_bootargs";
    let mut env = HashMap::new();
    env.insert(
        RUGIX_BOOTARGS.to_owned(),
        format!(
            "ro init=/usr/bin/rugix-ctrl root=PARTUUID={}",
            root.as_ref()
        ),
    );
    let encoded = grub_envblk_encode(&env).whatever("unable to encode boot environment")?;
    std::fs::write(boot_dir.as_ref().join("boot.grubenv"), encoded.as_bytes())
        .whatever("unable to write grub environment file")?;
    Ok(())
}

/// Patches `cmdline.txt` to use the given root device and `rugix-ctrl` as init process.
pub fn rpi_patch_boot(
    path: impl AsRef<Path>,
    root: impl AsRef<str>,
) -> Result<(), Report<BootPatchError>> {
    fn _patch_cmdline(path: &Path, root: &str) -> Result<(), Report<BootPatchError>> {
        let cmdline_path = path.join("cmdline.txt");
        let cmdline = fs::read_to_string(&cmdline_path)
            .whatever("unable to read `cmdline.txt` from boot partition")?;
        let mut parts = cmdline
            .split_ascii_whitespace()
            .filter(|part| {
                !part.starts_with("root=")
                    && !part.starts_with("init=")
                    && !part.starts_with("panic")
                    && *part != "quiet"
            })
            .map(str::to_owned)
            .collect::<Vec<_>>();
        parts.push("panic=60".to_owned());
        parts.push(format!("root={root}"));
        parts.push("init=/usr/bin/rugix-ctrl".to_owned());
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
    _patch_cmdline(path.as_ref(), root.as_ref())
}

/// Patches Armbian boot configuration and fstab for split boot partition.
/// Updates `armbianEnv.txt` with the correct root device and modifies fstab to mount boot partition.
pub fn armbian_patch_boot(
    boot_dir: impl AsRef<Path>,
    system_dir: impl AsRef<Path>,
    root_partuuid: impl AsRef<str>,
    boot_partuuid: impl AsRef<str>,
) -> Result<(), Report<BootPatchError>> {
    fn _patch_armbian(
        boot_dir: &Path,
        system_dir: &Path,
        root_partuuid: &str,
        boot_partuuid: &str,
    ) -> Result<(), Report<BootPatchError>> {
        // Update armbianEnv.txt with correct rootDev and init
        let armbian_env_path = boot_dir.join("armbianEnv.txt");
        if armbian_env_path.exists() {
            let env_content = fs::read_to_string(&armbian_env_path)
                .whatever("unable to read `armbianEnv.txt` from boot partition")?;
            
            let mut new_lines = Vec::new();
            let mut found_rootdev = false;
            let mut found_extraargs = false;
            
            for line in env_content.lines() {
                if line.starts_with("rootdev=") {
                    new_lines.push(format!("rootdev={}", root_partuuid));
                    found_rootdev = true;
                } else if line.starts_with("extraargs=") {
                    // Append init to existing extraargs
                    let existing_args = line.strip_prefix("extraargs=").unwrap_or("");
                    let mut args_parts = existing_args
                        .split_ascii_whitespace()
                        .filter(|part| !part.starts_with("init="))
                        .map(str::to_owned)
                        .collect::<Vec<_>>();
                    args_parts.push("init=/usr/bin/rugix-ctrl".to_owned());
                    new_lines.push(format!("extraargs={}", args_parts.join(" ")));
                    found_extraargs = true;
                } else {
                    new_lines.push(line.to_owned());
                }
            }
            
            // If rootdev was not found, add it
            if !found_rootdev {
                new_lines.push(format!("rootdev={}", root_partuuid));
            }
            
            // If extraargs was not found, add it
            if !found_extraargs {
                new_lines.push("extraargs=init=/usr/bin/rugix-ctrl".to_owned());
            }
            
            let new_content = new_lines.join("\n") + "\n";
            fs::write(&armbian_env_path, &new_content)
                .whatever("unable to write `armbianEnv.txt` to boot partition")?;
        }
        
        // Create fstab with root and boot partitions
        let fstab_path = system_dir.join("etc/fstab");
        let fstab_content = format!(
            "{}\t/\text4\tdefaults,noatime\t0\t1\n{}\t/boot\tvfat\tdefaults\t0\t2\n",
            root_partuuid, boot_partuuid
        );
        fs::write(&fstab_path, fstab_content)
            .whatever("unable to write fstab")?;
        
        Ok(())
    }
    _patch_armbian(boot_dir.as_ref(), system_dir.as_ref(), root_partuuid.as_ref(), boot_partuuid.as_ref())
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
