use std::path::Path;

use tracing::error;

use crate::slot_db;
use crate::system::paths::MOUNT_POINT_DATA;
use crate::system::System;

use crate::config::output::{
    BootGroupStateOutput, BootStateOutput, SlotStateOutput, StateManagementStatus, StateOutput,
    SystemStateOutput,
};

pub fn state_from_system(system: &System) -> SystemStateOutput {
    let boot_flow = system.boot_flow().name().to_owned();
    let slots = system
        .slots()
        .iter()
        .map(|(_, slot)| {
            let slot_state = match slot_db::get_stored_state(slot.name()) {
                Ok(state) => state,
                Err(error) => {
                    error!("unable to get state for slot {}: {:?}", slot.name(), error);
                    None
                }
            };
            (
                slot.name().to_owned(),
                SlotStateOutput {
                    active: Some(slot.active()),
                    hashes: slot_state.as_ref().map(|s| {
                        s.hashes
                            .iter()
                            .map(|(a, h)| (a.name().to_owned(), h.to_string()))
                            .collect()
                    }),
                    size: slot_state.as_ref().and_then(|s| s.size.map(|s| s.raw)),
                    updated_at: slot_state
                        .as_ref()
                        .and_then(|s| s.updated_at.map(|t| t.to_string())),
                },
            )
        })
        .collect();
    let active_boot_group = system
        .active_boot_entry()
        .map(|idx| system.boot_entries()[idx].name().to_owned());
    let default_boot_group = Some(
        system.boot_entries()[system.boot_flow().get_default(system).unwrap()]
            .name()
            .to_owned(),
    );
    let boot_groups = system
        .boot_entries()
        .iter()
        .map(|(_, group)| (group.name().to_owned(), BootGroupStateOutput {}))
        .collect();
    let state_management_status = if !Path::new("/run/rugix/state").exists() {
        StateManagementStatus::Disabled
    } else {
        if Path::new(MOUNT_POINT_DATA)
            .join(".rugix/data-mount-error.log")
            .exists()
        {
            StateManagementStatus::Error
        } else {
            StateManagementStatus::Active
        }
    };
    SystemStateOutput::new(slots, StateOutput::new(state_management_status)).with_boot(Some(
        BootStateOutput {
            boot_flow,
            active_group: active_boot_group,
            default_group: default_boot_group,
            groups: boot_groups,
        },
    ))
}
