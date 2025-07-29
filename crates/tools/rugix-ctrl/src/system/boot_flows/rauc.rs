//! RAUC-compatible boot flows.

use hashbrown::HashMap;
use reportify::bail;

use crate::boot::fwenv::{load_vars, set_vars};
use crate::system::boot_flows::{BootFlow, BootFlowResult};
use crate::system::boot_groups::{BootGroupIdx, BootGroups};

#[derive(Debug)]
struct RaucBootFlow {
    entry_a: BootGroupIdx,
    entry_b: BootGroupIdx,
}

fn rauc_boot_fow(boot_entries: &BootGroups) -> BootFlowResult<RaucBootFlow> {
    let mut entries = boot_entries.iter();
    let Some((entry_a_idx, _)) = entries.next() else {
        bail!("invalid number of entries");
    };
    let Some((entry_b_idx, _)) = entries.next() else {
        bail!("invalid number of entries");
    };
    Ok(RaucBootFlow {
        entry_a: entry_a_idx,
        entry_b: entry_b_idx,
    })
}

#[derive(Debug)]
pub struct RaucUboot {
    inner: RaucBootFlow,
}

impl RaucUboot {
    pub fn new(boot_entries: &BootGroups) -> BootFlowResult<Self> {
        let inner = rauc_boot_fow(boot_entries)?;
        Ok(Self { inner })
    }
}

impl BootFlow for RaucUboot {
    fn name(&self) -> &str {
        "rauc-uboot"
    }

    fn set_try_next(
        &self,
        system: &crate::system::System,
        group: BootGroupIdx,
    ) -> super::BootFlowResult<()> {
        if group != self.get_default(system)? {
            let mut env = HashMap::new();
            if group == self.inner.entry_a {
                env.insert("BOOT_ORDER".to_owned(), "A B".to_owned());
                env.insert("BOOT_A_LEFT".to_owned(), "1".to_owned());
                env.insert("BOOT_B_LEFT".to_owned(), "3".to_owned());
            } else {
                env.insert("BOOT_ORDER".to_owned(), "B A".to_owned());
                env.insert("BOOT_A_LEFT".to_owned(), "3".to_owned());
                env.insert("BOOT_B_LEFT".to_owned(), "1".to_owned());
            }
            set_vars(&env)?;
        }
        Ok(())
    }

    fn get_default(&self, _: &crate::system::System) -> super::BootFlowResult<BootGroupIdx> {
        let boot_env = load_vars()?;
        let Some(boot_order) = boot_env.get("BOOT_ORDER").map(|v| v.trim()) else {
            bail!("unable to determine the boot order");
        };
        let Some(a_left) = boot_env
            .get("BOOT_A_LEFT")
            .and_then(|v| v.trim().parse::<u32>().ok())
        else {
            bail!("Update available flag is not set");
        };
        let Some(b_left) = boot_env
            .get("BOOT_B_LEFT")
            .and_then(|v| v.trim().parse::<u32>().ok())
        else {
            bail!("Update available flag is not set");
        };
        let a_first = boot_order == "A B";
        // Invert active `mender_boot_part` if an update is available.
        Ok(if a_first {
            if a_left > 0 {
                self.inner.entry_a
            } else {
                self.inner.entry_b
            }
        } else {
            if b_left > 0 {
                self.inner.entry_b
            } else {
                self.inner.entry_a
            }
        })
    }

    fn commit(&self, system: &crate::system::System) -> super::BootFlowResult<()> {
        let mut env = HashMap::new();
        if system.active_boot_entry().unwrap() == self.inner.entry_a {
            env.insert("BOOT_ORDER".to_owned(), "A B".to_owned());
            env.insert("BOOT_A_LEFT".to_owned(), "3".to_owned());
        } else {
            env.insert("BOOT_ORDER".to_owned(), "B A".to_owned());
            env.insert("BOOT_A_LEFT".to_owned(), "3".to_owned());
        };
        set_vars(&env)?;
        Ok(())
    }
}
