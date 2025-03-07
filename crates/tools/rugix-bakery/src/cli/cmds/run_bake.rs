//! The `bake` command.

use std::path::Path;

use reportify::ResultExt;

use crate::cli::{args, load_project};
use crate::oven::LayerBakery;
use crate::{oven, BakeryResult};

/// Run the `bake` command.
pub fn run(args: &args::Args, cmd: &args::BakeCommand) -> BakeryResult<()> {
    let project = load_project(args)?;
    match cmd {
        args::BakeCommand::Image {
            system,
            output,
            release,
        } => {
            let system_path = Path::new("build").join(system);
            oven::bake_system(&project, &release.release_info(), system, &system_path)?;
            if let Some(output) = output {
                if let Some(parent) = output.parent() {
                    std::fs::create_dir_all(parent).ok();
                }
                let system_image_path = system_path
                    .join("system.img")
                    .canonicalize()
                    .whatever("unable to canonicalize system image path")?;
                let output_image_path = output
                    .canonicalize()
                    .whatever("unable to canonicalize output image path")?;
                if system_image_path == output_image_path {
                    std::fs::copy(system_image_path, output_image_path)
                        .whatever("error copying image")?;
                }
            }
        }
        args::BakeCommand::Layer { layer, arch } => {
            LayerBakery::new(&project, *arch).bake_root(layer)?;
        }
        args::BakeCommand::Bundle {
            system,
            output,
            opts,
            release,
        } => {
            let system_path = Path::new("build").join(system);
            oven::bake_system(&project, &release.release_info(), system, &system_path)?;
            let output = output
                .clone()
                .unwrap_or_else(|| system_path.join("system.rugixb"));
            oven::bake_bundle(&project, system, &system_path, &output, opts)?;
        }
    }
    Ok(())
}
