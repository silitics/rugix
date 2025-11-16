use std::path::Path;

use reportify::{bail, ResultExt};

use rugix_common::boot::uboot::UBootEnv;
use rugix_common::fsutils::copy_recursive;

use crate::BakeryResult;

pub fn initialize_armbian_uboot(layer_path: &Path, config_dir: &Path) -> BakeryResult<()> {
    // Copy first-stage boot script from layer
    let boot_dir = layer_path.join("roots/boot");
    let first_stage_script = boot_dir.join("first-stage.boot.scr");
    
    if !first_stage_script.exists() {
        bail!("first-stage boot script not found at {}", first_stage_script.display());
    }
    
    copy_recursive(&first_stage_script, config_dir.join("boot.scr"))
        .whatever("unable to copy first-stage boot script to boot.scr")?;
    
    // Create default U-Boot environment
    let mut env = UBootEnv::new();
    env.set("bootpart", "2");
    env.save(config_dir.join("bootpart.default.env"))
        .whatever("unable to create default U-Boot environment")?;

    let mut env = UBootEnv::new();
    env.set("boot_spare", "0");
    env.save(config_dir.join("boot_spare.disabled.env"))
        .whatever("unable to write U-Boot environment")?;
    env.save(config_dir.join("boot_spare.env"))
        .whatever("unable to write U-Boot environment")?;

    let mut env = UBootEnv::new();
    env.set("boot_spare", "1");
    env.save(config_dir.join("boot_spare.enabled.env"))
        .whatever("unable to write U-Boot environment")?;

    Ok(())
}
