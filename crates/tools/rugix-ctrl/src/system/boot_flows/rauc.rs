//! RAUC-compatible boot flows.

use hashbrown::HashMap;
use reportify::bail;

use crate::boot::fwenv::{load_vars, set_vars};
use crate::config::system::RaucBootFlowConfig;
use crate::system::boot_flows::{BootFlow, BootFlowResult};
use crate::system::boot_groups::{BootGroupIdx, BootGroups};

#[derive(Debug, Clone)]
struct RaucBootGroup {
    idx: BootGroupIdx,
    name: String,
}

#[derive(Debug)]
struct RaucBootFlow {
    groups: HashMap<BootGroupIdx, RaucBootGroup>,
}

fn rauc_boot_fow(
    boot_entries: &BootGroups,
    config: &RaucBootFlowConfig,
) -> BootFlowResult<RaucBootFlow> {
    let groups = boot_entries
        .iter()
        .enumerate()
        .map(|(no, (idx, group))| {
            (
                idx,
                RaucBootGroup {
                    idx,
                    name: config
                        .group_names
                        .as_ref()
                        .and_then(|n| n.get(no))
                        .map(|n| n.to_owned())
                        .unwrap_or_else(|| group.name().to_uppercase()),
                },
            )
        })
        .collect::<HashMap<_, _>>();
    if groups.len() < 2 {
        bail!("at least two boot groups are required");
    }
    Ok(RaucBootFlow { groups })
}

#[derive(Debug)]
pub struct RaucUboot {
    inner: RaucBootFlow,
}

impl RaucUboot {
    pub fn new(boot_entries: &BootGroups, config: &RaucBootFlowConfig) -> BootFlowResult<Self> {
        let inner = rauc_boot_fow(boot_entries, config)?;
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
            let boot_env = load_vars()?;
            let Some(rauc_group) = self.inner.groups.get(&group) else {
                bail!("invalid boot group");
            };
            let Some(mut boot_order) = boot_env.get("BOOT_ORDER").map(|v| v.trim()).map(|v| {
                v.split_whitespace()
                    .map(|e| e.to_owned())
                    .collect::<Vec<_>>()
            }) else {
                bail!("unable to determine the boot order");
            };
            boot_order.retain(|e| e != &rauc_group.name);
            boot_order.insert(0, rauc_group.name.clone());
            let mut env = HashMap::new();
            // Allow booting into the selected slot once.
            env.insert(format!("BOOT_{}_LEFT", rauc_group.name), "1".to_owned());
            env.insert("BOOT_ORDER".to_owned(), boot_order.join(" "));
            set_vars(&env)?;
        }
        Ok(())
    }

    fn get_default(&self, _: &crate::system::System) -> super::BootFlowResult<BootGroupIdx> {
        let boot_env = load_vars()?;
        let Some(boot_order) = boot_env
            .get("BOOT_ORDER")
            .map(|v| v.trim())
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
        else {
            bail!("unable to determine the boot order");
        };
        for group in boot_order {
            let left = boot_env
                .get(&format!("BOOT_{group}_LEFT"))
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(0);
            for rauc_group in self.inner.groups.values() {
                if group == rauc_group.name && left > 0 {
                    return Ok(rauc_group.idx);
                }
            }
        }
        bail!("unable to determine the default boot group");
    }

    fn commit(&self, system: &crate::system::System) -> super::BootFlowResult<()> {
        let boot_env = load_vars()?;
        let group = system.active_boot_entry().unwrap();
        let Some(rauc_group) = self.inner.groups.get(&group) else {
            bail!("invalid boot group");
        };
        let Some(mut boot_order) = boot_env.get("BOOT_ORDER").map(|v| v.trim()).map(|v| {
            v.split_whitespace()
                .map(|e| e.to_owned())
                .collect::<Vec<_>>()
        }) else {
            bail!("unable to determine the boot order");
        };
        boot_order.retain(|e| e != &rauc_group.name);
        boot_order.insert(0, rauc_group.name.clone());
        let mut env = HashMap::new();
        // Allow booting into the selected slot once.
        env.insert(format!("BOOT_{}_LEFT", rauc_group.name), "3".to_owned());
        env.insert("BOOT_ORDER".to_owned(), boot_order.join(" "));
        set_vars(&env)?;
        Ok(())
    }

    fn mark_good(&self, _: &crate::system::System, group: BootGroupIdx) -> BootFlowResult<()> {
        let mut env = HashMap::new();
        let rauc_group = self.inner.groups.get(&group).unwrap();
        env.insert(format!("BOOT_{}_LEFT", rauc_group.name), "3".to_owned());
        set_vars(&env)?;
        Ok(())
    }

    fn mark_bad(&self, system: &crate::system::System, group: BootGroupIdx) -> BootFlowResult<()> {
        let mut env = HashMap::new();
        if group == system.active_boot_entry().unwrap() {
            bail!("cannot mark the active boot group as bad");
        }
        let rauc_group = self.inner.groups.get(&group).unwrap();
        env.insert(format!("BOOT_{}_LEFT", rauc_group.name), "0".to_owned());
        set_vars(&env)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RaucGrub {
    inner: RaucBootFlow,
}

impl RaucGrub {
    pub fn new(boot_entries: &BootGroups, config: &RaucBootFlowConfig) -> BootFlowResult<Self> {
        let inner = rauc_boot_fow(boot_entries, config)?;
        Ok(Self { inner })
    }
}

impl BootFlow for RaucGrub {
    fn name(&self) -> &str {
        "rauc-grub"
    }

    fn set_try_next(
        &self,
        system: &crate::system::System,
        group: BootGroupIdx,
    ) -> super::BootFlowResult<()> {
        if group != self.get_default(system)? {
            let boot_env = load_vars()?;
            let Some(rauc_group) = self.inner.groups.get(&group) else {
                bail!("invalid boot group");
            };
            let Some(mut boot_order) = boot_env.get("BOOT_ORDER").map(|v| v.trim()).map(|v| {
                v.split_whitespace()
                    .map(|e| e.to_owned())
                    .collect::<Vec<_>>()
            }) else {
                bail!("unable to determine the boot order");
            };
            boot_order.retain(|e| e != &rauc_group.name);
            boot_order.insert(0, rauc_group.name.clone());
            let mut env = HashMap::new();
            env.insert(format!("{}_OK", rauc_group.name), "1".to_owned());
            env.insert(format!("{}_TRY", rauc_group.name), "0".to_owned());
            env.insert("BOOT_ORDER".to_owned(), boot_order.join(" "));
            set_vars(&env)?;
        }
        Ok(())
    }

    fn get_default(&self, _: &crate::system::System) -> super::BootFlowResult<BootGroupIdx> {
        let boot_env = load_vars()?;
        let Some(boot_order) = boot_env
            .get("BOOT_ORDER")
            .map(|v| v.trim())
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
        else {
            bail!("unable to determine the boot order");
        };
        for group in boot_order {
            let group_ok = boot_env
                .get(&format!("{group}_OK"))
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(0);
            let group_try = boot_env
                .get(&format!("{group}_TRY"))
                .and_then(|v| v.trim().parse::<u32>().ok())
                .unwrap_or(1);
            for rauc_group in self.inner.groups.values() {
                if group_ok > 0 && group_try < 1 {
                    return Ok(rauc_group.idx);
                }
            }
        }
        bail!("unable to determine the default boot group");
    }

    fn commit(&self, system: &crate::system::System) -> super::BootFlowResult<()> {
        let boot_env = load_vars()?;
        let group = system.active_boot_entry().unwrap();
        let Some(rauc_group) = self.inner.groups.get(&group) else {
            bail!("invalid boot group");
        };
        let Some(mut boot_order) = boot_env.get("BOOT_ORDER").map(|v| v.trim()).map(|v| {
            v.split_whitespace()
                .map(|e| e.to_owned())
                .collect::<Vec<_>>()
        }) else {
            bail!("unable to determine the boot order");
        };
        boot_order.retain(|e| e != &rauc_group.name);
        boot_order.insert(0, rauc_group.name.clone());
        let mut env = HashMap::new();
        // Allow booting into the selected slot once.
        env.insert(format!("{}_OK", rauc_group.name), "1".to_owned());
        env.insert(format!("{}_TRY", rauc_group.name), "0".to_owned());
        env.insert("BOOT_ORDER".to_owned(), boot_order.join(" "));
        set_vars(&env)?;
        Ok(())
    }

    fn mark_good(&self, _: &crate::system::System, group: BootGroupIdx) -> BootFlowResult<()> {
        let mut env = HashMap::new();
        let rauc_group = self.inner.groups.get(&group).unwrap();
        env.insert(format!("{}_OK", rauc_group.name), "1".to_owned());
        env.insert(format!("{}_TRY", rauc_group.name), "0".to_owned());
        set_vars(&env)?;
        Ok(())
    }

    fn mark_bad(&self, system: &crate::system::System, group: BootGroupIdx) -> BootFlowResult<()> {
        let mut env = HashMap::new();
        if group == system.active_boot_entry().unwrap() {
            bail!("cannot mark the active boot group as bad");
        }
        let rauc_group = self.inner.groups.get(&group).unwrap();
        env.insert(format!("{}_OK", rauc_group.name), "0".to_owned());
        env.insert(format!("{}_TRY", rauc_group.name), "0".to_owned());
        set_vars(&env)?;
        Ok(())
    }
}
