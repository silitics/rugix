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

use std::path::PathBuf;

use hashbrown::HashMap;
use reportify::{bail, ResultExt};
use rugix_common::boot::grub::{load_grub_env, save_grub_env};

use crate::boot::fwenv::{load_vars, set_vars};
use crate::system::boot_flows::{BootFlow, BootFlowResult};
use crate::system::boot_groups::{BootGroupIdx, BootGroups};

#[derive(Debug)]
struct MenderBootFlow {
    entry_a: BootGroupIdx,
    entry_b: BootGroupIdx,
}

fn mender_boot_flow(boot_entries: &BootGroups) -> BootFlowResult<MenderBootFlow> {
    let mut entries = boot_entries.iter();
    let Some((entry_a_idx, _)) = entries.next() else {
        bail!("invalid number of entries");
    };
    let Some((entry_b_idx, _)) = entries.next() else {
        bail!("invalid number of entries");
    };
    Ok(MenderBootFlow {
        entry_a: entry_a_idx,
        entry_b: entry_b_idx,
    })
}

const MENDER_GRUB_ENV1: &str = "grub-mender-grubenv/mender_grubenv1/env";
const MENDER_GRUB_ENV2: &str = "grub-mender-grubenv/mender_grubenv2/env";

#[derive(Debug)]
pub struct MenderGrub {
    inner: MenderBootFlow,
    boot_root: PathBuf,
    boot_part_a: String,
    boot_part_b: String,
}

impl MenderGrub {
    pub fn new(boot_entries: &BootGroups) -> BootFlowResult<Self> {
        let inner = mender_boot_flow(boot_entries)?;
        Ok(Self {
            inner,
            boot_root: PathBuf::from("/boot"),
            boot_part_a: "2".to_owned(),
            boot_part_b: "3".to_owned(),
        })
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
        let mut boot_env = load_grub_env(&self.boot_root.join(MENDER_GRUB_ENV1))
            .whatever("unable to load Grub environment")?;
        if group != self.get_default(system)? {
            boot_env.insert("bootcount".to_owned(), "0".to_owned());
            boot_env.insert("upgrade_available".to_owned(), "1".to_owned());
        } else {
            boot_env.insert("bootcount".to_owned(), "0".to_owned());
            boot_env.insert("upgrade_available".to_owned(), "0".to_owned());
        }
        if group == self.inner.entry_a {
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_a.clone());
        } else {
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_b.clone());
        }
        // TODO: Implement Mender's lock file mechanism.
        save_grub_env(&self.boot_root.join(MENDER_GRUB_ENV1), &boot_env)
            .whatever("unable to save Grub environment")?;
        save_grub_env(&self.boot_root.join(MENDER_GRUB_ENV2), &boot_env)
            .whatever("unable to save Grub environment")?;

        Ok(())
    }

    fn get_default(&self, _: &crate::system::System) -> super::BootFlowResult<BootGroupIdx> {
        let boot_env = load_grub_env(&self.boot_root.join(MENDER_GRUB_ENV1))
            .whatever("unable to load Grub environment")?;
        let Some(mender_boot_part) = boot_env.get("mender_boot_part").map(|v| v.trim()) else {
            bail!("Mender boot partition is not set");
        };
        let Some(update_avaliable) = boot_env.get("upgrade_available").map(|v| v.trim()) else {
            bail!("Update available flag is not set");
        };
        // Invert active `mender_boot_part` if an update is available.
        Ok(if update_avaliable == "1" {
            if mender_boot_part == self.boot_part_a {
                self.inner.entry_b
            } else {
                self.inner.entry_a
            }
        } else {
            if mender_boot_part == self.boot_part_a {
                self.inner.entry_a
            } else {
                self.inner.entry_b
            }
        })
    }

    fn commit(&self, system: &crate::system::System) -> super::BootFlowResult<()> {
        let mut boot_env = load_grub_env(&self.boot_root.join(MENDER_GRUB_ENV1))
            .whatever("unable to load Grub environment")?;
        boot_env.insert("bootcount".to_owned(), "0".to_owned());
        boot_env.insert("upgrade_available".to_owned(), "0".to_owned());
        if system.active_boot_entry().unwrap() == self.inner.entry_a {
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_a.clone());
        } else {
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_b.to_owned());
        };
        save_grub_env(&self.boot_root.join(MENDER_GRUB_ENV1), &boot_env)
            .whatever("unable to save Grub environment")?;
        save_grub_env(&self.boot_root.join(MENDER_GRUB_ENV2), &boot_env)
            .whatever("unable to save Grub environment")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MenderUboot {
    inner: MenderBootFlow,
    boot_part_a: String,
    boot_part_b: String,
}

impl MenderUboot {
    pub fn new(boot_entries: &BootGroups) -> BootFlowResult<Self> {
        let inner = mender_boot_flow(boot_entries)?;
        Ok(Self {
            inner,
            boot_part_a: "2".to_owned(),
            boot_part_b: "3".to_owned(),
        })
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
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_a.clone());
            boot_env.insert("mender_boot_part_hex".to_owned(), self.boot_part_a.clone());
        } else {
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_b.clone());
            boot_env.insert("mender_boot_part_hex".to_owned(), self.boot_part_b.clone());
        }
        set_vars(&boot_env)?;
        Ok(())
    }

    fn get_default(&self, _: &crate::system::System) -> super::BootFlowResult<BootGroupIdx> {
        let boot_env = load_vars()?;
        let Some(mender_boot_part) = boot_env.get("mender_boot_part").map(|v| v.trim()) else {
            bail!("Mender boot partition is not set");
        };
        let Some(update_avaliable) = boot_env.get("upgrade_available").map(|v| v.trim()) else {
            bail!("Update available flag is not set");
        };
        // Invert active `mender_boot_part` if an update is available.
        Ok(if update_avaliable == "1" {
            if mender_boot_part == self.boot_part_a {
                self.inner.entry_b
            } else {
                self.inner.entry_a
            }
        } else {
            if mender_boot_part == self.boot_part_a {
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
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_a.clone());
            boot_env.insert("mender_boot_part_hex".to_owned(), self.boot_part_a.clone());
        } else {
            boot_env.insert("mender_boot_part".to_owned(), self.boot_part_b.clone());
            boot_env.insert("mender_boot_part_hex".to_owned(), self.boot_part_b.clone());
        };
        set_vars(&boot_env)?;
        Ok(())
    }
}
