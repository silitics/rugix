use std::{path::Path, time::Duration};

use reportify::{bail, Report, ResultExt};
use tokio::{fs, task::spawn_blocking};
use tracing::info;
use workflow::{TestStep, TestWorkflow};

use crate::{bake, project::Project};

pub mod qemu;
pub mod workflow;

reportify::new_whatever_type! {
    RugpiTestError
}

pub type RugpiTestResult<T> = Result<T, Report<RugpiTestError>>;

pub async fn main(project: &Project, workflow: &Path) -> RugpiTestResult<()> {
    let workflow = toml::from_str::<TestWorkflow>(
        &fs::read_to_string(&workflow)
            .await
            .whatever("unable to read test workflow")?,
    )
    .whatever("unable to parse test workflow")?;

    for system in &workflow.systems {
        let output = Path::new("build/images")
            .join(&system.disk_image)
            .with_extension("img");
        let project = project.clone();
        let disk_image = system.disk_image.clone();
        {
            let output = output.clone();
            spawn_blocking(move || bake::bake_image(&project, &disk_image, &output))
                .await
                .whatever("error baking image")?
                .whatever("error baking image")?;
        }

        let vm = qemu::start(&output.to_string_lossy(), system).await?;

        info!("VM started");

        for step in &workflow.steps {
            match step {
                workflow::TestStep::Run {
                    script,
                    stdin,
                    may_fail,
                } => {
                    info!("running script");
                    vm.wait_for_ssh()
                        .await
                        .whatever("unable to connect to VM via SSH")?;
                    if let Err(report) = vm
                        .run_script(script, stdin.as_ref().map(|p| p.as_ref()))
                        .await
                        .whatever::<RugpiTestError, _>("unable to run script")
                    {
                        if may_fail.unwrap_or(false) {
                            eprintln!("ignoring error while executing script:\n{report:?}");
                        } else {
                            bail!("error during test")
                        }
                    }
                }
                TestStep::Wait { duration } => {
                    info!("waiting for {duration} seconds");
                    tokio::time::sleep(Duration::from_secs_f64(*duration)).await;
                }
            }
        }
    }

    Ok(())
}
