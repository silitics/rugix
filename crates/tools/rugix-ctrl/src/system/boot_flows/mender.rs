//! Mender-compatible boot flows.

/*

# Grub

There are redundant Grub environment files:

- `grub-mender-grubenv/mender_grubenv1/env`
- `grub-mender-grubenv/mender_grubenv2/env`

In addition, there are two lock files:

- `grub-mender-grubenv/mender_grubenv1/lock`
- `grub-mender-grubenv/mender_grubenv2/lock`

The lock files are Grub environment files with a variable `editing`. This variable is set
to `1` prior to editing the respective environment file and to `0` afterwards.

Within the environment files, there are three variables:

- `bootcount`: Number of attempts of booting the update.
- `upgrade_available`: Indicates whether an update is available.
- `mender_boot_part`: Boot partition to boot from next.

*/

use std::path::Path;
use std::str::FromStr;

use hashbrown::HashMap;
use reportify::{bail, ResultExt};
use rugix_common::boot::grub::{load_grub_env, save_grub_env, GrubEnv};
use tracing::error;

use crate::boot::fwenv::{load_vars, set_vars};
use crate::config::system::MenderBootFlowConfig;
use crate::system::boot_flows::{BootFlow, BootFlowResult};
use crate::system::boot_groups::{BootGroupIdx, BootGroups};
use crate::system::System;

#[derive(Debug)]
struct MenderBootFlow {
    config: MenderBootFlowConfig,
    entry_a: BootGroupIdx,
    entry_b: BootGroupIdx,
}

impl MenderBootFlow {
    pub fn boot_root(&self) -> &Path {
        self.config
            .boot_dir
            .as_deref()
            .map(Path::new)
            .unwrap_or(Path::new("/boot/efi"))
    }

    pub fn boot_part_a(&self) -> u32 {
        self.config.boot_part_a.unwrap_or(2)
    }

    pub fn boot_part_b(&self) -> u32 {
        self.config.boot_part_b.unwrap_or(3)
    }
}

fn mender_boot_flow(
    boot_entries: &BootGroups,
    config: &MenderBootFlowConfig,
) -> BootFlowResult<MenderBootFlow> {
    let mut entries = boot_entries.iter();
    let Some((entry_a_idx, _)) = entries.next() else {
        bail!("invalid number of entries");
    };
    let Some((entry_b_idx, _)) = entries.next() else {
        bail!("invalid number of entries");
    };
    Ok(MenderBootFlow {
        config: config.clone(),
        entry_a: entry_a_idx,
        entry_b: entry_b_idx,
    })
}

const MENDER_GRUB_ENV1: &str = "grub-mender-grubenv/mender_grubenv1/env";
const MENDER_GRUB_LOCK1: &str = "grub-mender-grubenv/mender_grubenv1/lock";
const MENDER_GRUB_ENV2: &str = "grub-mender-grubenv/mender_grubenv2/env";
const MENDER_GRUB_LOCK2: &str = "grub-mender-grubenv/mender_grubenv2/lock";

fn mender_save_grub_env(boot_root: &Path, env: &GrubEnv) -> BootFlowResult<()> {
    let mut locked = std::collections::HashMap::new();
    locked.insert("editing".to_owned(), "1".to_owned());
    let mut unlocked = std::collections::HashMap::new();
    unlocked.insert("editing".to_owned(), "0".to_owned());
    // Primary environment file.
    save_grub_env(boot_root.join(MENDER_GRUB_LOCK1), &locked).ok();
    save_grub_env(boot_root.join(MENDER_GRUB_ENV1), env)
        .whatever("unable to save Grub environment")?;
    save_grub_env(boot_root.join(MENDER_GRUB_LOCK1), &unlocked).ok();
    // Secondary environment file.
    save_grub_env(boot_root.join(MENDER_GRUB_LOCK2), &locked).ok();
    save_grub_env(boot_root.join(MENDER_GRUB_ENV2), &env)
        .whatever("unable to save Grub environment")?;
    save_grub_env(&boot_root.join(MENDER_GRUB_LOCK2), &locked).ok();
    Ok(())
}

fn mender_load_grub_env(system: &System, boot_root: &Path) -> BootFlowResult<GrubEnv> {
    let primary_ok = load_grub_env(boot_root.join(MENDER_GRUB_ENV1))
        .map(|env| match env.get("editing").as_deref() {
            Some(value) if value == "0" => true,
            _ => false,
        })
        .unwrap_or(false);
    let secondary_ok = load_grub_env(boot_root.join(MENDER_GRUB_ENV1))
        .map(|env| match env.get("editing").as_deref() {
            Some(value) if value == "0" => true,
            _ => false,
        })
        .unwrap_or(false);
    let boot_env = if primary_ok {
        load_grub_env(boot_root.join(MENDER_GRUB_ENV1))
            .whatever("unable to load Grub environment")?
    } else if secondary_ok {
        load_grub_env(boot_root.join(MENDER_GRUB_ENV2))
            .whatever("unable to load Grub environment")?
    } else {
        error!("both primary and secondary environment files are corrupted");
        // Try to load the primary environment anyway.
        load_grub_env(boot_root.join(MENDER_GRUB_ENV1))
            .whatever("unable to load Grub environment")?
    };

    if !primary_ok || !secondary_ok {
        let _write_guard = system.config_partition().and_then(|c| {
            if boot_root.starts_with(c.path()) {
                Some(c.acquire_write_guard())
            } else {
                None
            }
        });
        let mut locked = std::collections::HashMap::new();
        locked.insert("editing".to_owned(), "1".to_owned());
        let mut unlocked = std::collections::HashMap::new();
        unlocked.insert("editing".to_owned(), "0".to_owned());
        if !primary_ok {
            save_grub_env(boot_root.join(MENDER_GRUB_LOCK1), &locked).ok();
            save_grub_env(boot_root.join(MENDER_GRUB_ENV1), &boot_env).ok();
            save_grub_env(boot_root.join(MENDER_GRUB_LOCK1), &unlocked).ok();
        }
        if !secondary_ok {
            save_grub_env(boot_root.join(MENDER_GRUB_LOCK2), &locked).ok();
            save_grub_env(boot_root.join(MENDER_GRUB_ENV2), &boot_env).ok();
            save_grub_env(&boot_root.join(MENDER_GRUB_LOCK2), &locked).ok();
        }
    }
    Ok(boot_env)
}

#[derive(Debug)]
pub struct MenderGrub {
    inner: MenderBootFlow,
}

impl MenderGrub {
    pub fn new(boot_entries: &BootGroups, config: &MenderBootFlowConfig) -> BootFlowResult<Self> {
        let inner = mender_boot_flow(boot_entries, config)?;
        Ok(Self { inner })
    }
}

impl BootFlow for MenderGrub {
    fn name(&self) -> &str {
        "mender-grub"
    }

    fn set_try_next(
        &self,
        system: &crate::system::System,
        group: BootGroupIdx,
    ) -> super::BootFlowResult<()> {
        let mut boot_env = load_grub_env(self.inner.boot_root().join(MENDER_GRUB_ENV1))
            .whatever("unable to load Grub environment")?;
        if group != self.get_default(system)? {
            boot_env.insert("bootcount".to_owned(), "0".to_owned());
            boot_env.insert("upgrade_available".to_owned(), "1".to_owned());
        } else {
            boot_env.insert("bootcount".to_owned(), "0".to_owned());
            boot_env.insert("upgrade_available".to_owned(), "0".to_owned());
        }
        if group == self.inner.entry_a {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_a().to_string(),
            );
        } else {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_b().to_string(),
            );
        }
        // Load the boot environment once and repair it if necessary.
        let _ = mender_load_grub_env(system, self.inner.boot_root());
        let _write_guard = system.config_partition().and_then(|c| {
            if self.inner.boot_root().starts_with(c.path()) {
                Some(c.acquire_write_guard())
            } else {
                None
            }
        });
        mender_save_grub_env(self.inner.boot_root(), &boot_env)?;
        Ok(())
    }

    fn get_default(&self, system: &crate::system::System) -> super::BootFlowResult<BootGroupIdx> {
        let boot_env = mender_load_grub_env(system, self.inner.boot_root())?;
        let Some(mender_boot_part) = boot_env
            .get("mender_boot_part")
            .map(|v| v.trim())
            .and_then(|v| u32::from_str(v).ok())
        else {
            bail!("Mender boot partition is not set");
        };
        let Some(upgrade_available) = boot_env.get("upgrade_available").map(|v| v.trim()) else {
            bail!("Update available flag is not set");
        };
        // Invert active `mender_boot_part` if an update is available.
        Ok(if upgrade_available == "1" {
            if mender_boot_part == self.inner.boot_part_a() {
                self.inner.entry_b
            } else {
                self.inner.entry_a
            }
        } else {
            if mender_boot_part == self.inner.boot_part_a() {
                self.inner.entry_a
            } else {
                self.inner.entry_b
            }
        })
    }

    fn commit(&self, system: &crate::system::System) -> super::BootFlowResult<()> {
        let mut boot_env = load_grub_env(self.inner.boot_root().join(MENDER_GRUB_ENV1))
            .whatever("unable to load Grub environment")?;
        boot_env.insert("bootcount".to_owned(), "0".to_owned());
        boot_env.insert("upgrade_available".to_owned(), "0".to_owned());
        if system.active_boot_entry().unwrap() == self.inner.entry_a {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_a().to_string(),
            );
        } else {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_b().to_string(),
            );
        };
        let _ = mender_load_grub_env(system, self.inner.boot_root());
        let _write_guard = system.config_partition().and_then(|c| {
            if self.inner.boot_root().starts_with(c.path()) {
                Some(c.acquire_write_guard())
            } else {
                None
            }
        });
        mender_save_grub_env(self.inner.boot_root(), &boot_env)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MenderUboot {
    inner: MenderBootFlow,
}

impl MenderUboot {
    pub fn new(boot_entries: &BootGroups, config: &MenderBootFlowConfig) -> BootFlowResult<Self> {
        let inner = mender_boot_flow(boot_entries, config)?;
        Ok(Self { inner })
    }
}

impl BootFlow for MenderUboot {
    fn name(&self) -> &str {
        "mender-uboot"
    }

    fn set_try_next(
        &self,
        system: &crate::system::System,
        group: BootGroupIdx,
    ) -> super::BootFlowResult<()> {
        let mut boot_env = HashMap::new();
        if group != self.get_default(system)? {
            boot_env.insert("bootcount".to_owned(), "0".to_owned());
            boot_env.insert("upgrade_available".to_owned(), "1".to_owned());
        } else {
            boot_env.insert("bootcount".to_owned(), "0".to_owned());
            boot_env.insert("upgrade_available".to_owned(), "0".to_owned());
        }
        if group == self.inner.entry_a {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_a().to_string(),
            );
            boot_env.insert(
                "mender_boot_part_hex".to_owned(),
                format!("{:#x}", self.inner.boot_part_a()),
            );
        } else {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_b().to_string(),
            );
            boot_env.insert(
                "mender_boot_part_hex".to_owned(),
                format!("{:#x}", self.inner.boot_part_b()),
            );
        }
        set_vars(&boot_env)?;
        Ok(())
    }

    fn get_default(&self, _: &crate::system::System) -> super::BootFlowResult<BootGroupIdx> {
        let boot_env = load_vars()?;
        let Some(mender_boot_part) = boot_env
            .get("mender_boot_part")
            .map(|v| v.trim())
            .and_then(|v| u32::from_str(v).ok())
        else {
            bail!("Mender boot partition is not set");
        };
        let Some(update_avaliable) = boot_env.get("upgrade_available").map(|v| v.trim()) else {
            bail!("Update available flag is not set");
        };
        // Invert active `mender_boot_part` if an update is available.
        Ok(if update_avaliable == "1" {
            if mender_boot_part == self.inner.boot_part_a() {
                self.inner.entry_b
            } else {
                self.inner.entry_a
            }
        } else {
            if mender_boot_part == self.inner.boot_part_b() {
                self.inner.entry_a
            } else {
                self.inner.entry_b
            }
        })
    }

    fn commit(&self, system: &crate::system::System) -> super::BootFlowResult<()> {
        let mut boot_env = HashMap::new();
        boot_env.insert("bootcount".to_owned(), "0".to_owned());
        boot_env.insert("upgrade_available".to_owned(), "0".to_owned());
        if system.active_boot_entry().unwrap() == self.inner.entry_a {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_a().to_string(),
            );
            boot_env.insert(
                "mender_boot_part_hex".to_owned(),
                format!("{:#x}", self.inner.boot_part_a()),
            );
        } else {
            boot_env.insert(
                "mender_boot_part".to_owned(),
                self.inner.boot_part_b().to_string(),
            );
            boot_env.insert(
                "mender_boot_part_hex".to_owned(),
                format!("{:#x}", self.inner.boot_part_b()),
            );
        };
        set_vars(&boot_env)?;
        Ok(())
    }
}
