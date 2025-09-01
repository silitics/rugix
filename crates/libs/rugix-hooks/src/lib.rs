use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use tracing::{info, warn};

use xscript::Vars;

use reportify::{bail, ErrorExt, Report, ResultExt};

use crate::error::{HooksLoadError, HooksRunError};

pub mod error;

/// Collection of hooks for some operation.
pub struct Hooks {
    /// Name of the operation.
    operation: &'static str,
    /// Sorted hooks grouped by stage.
    stages: HashMap<String, Vec<Hook>>,
}

#[derive(Debug, Clone, Default)]
pub struct RunOptions {
    pub silent: bool,
}

impl RunOptions {
    pub fn with_silent(&mut self, silent: bool) -> &mut Self {
        self.silent = silent;
        self
    }
}

impl Hooks {
    /// Iterator over the hooks for the given stage.
    pub fn hooks(&self, stage: &str) -> &[Hook] {
        self.stages.get(stage).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Run the hooks for the given stage.
    pub fn run_hooks(
        &self,
        stage: &str,
        vars: Vars,
        opts: &RunOptions,
    ) -> Result<(), Report<HooksRunError>> {
        if !opts.silent {
            info!("running hooks for \"{}/{}\"", self.operation, stage);
        }
        for hook in self.hooks(stage) {
            if !opts.silent {
                info!("running hook {}", hook.name);
            }
            let mut cmd = Command::new(&hook.path);
            cmd.arg(self.operation).arg(stage);
            for (var, value) in vars.values() {
                let Some(value) = value else {
                    continue;
                };
                cmd.env(var, value);
            }
            if opts.silent {
                cmd.stderr(Stdio::null());
                cmd.stdout(Stdio::null());
            }
            if !cmd.status().map(|s| s.success()).unwrap_or(false) {
                bail!("hook \"{}/{}/{}\" failed", self.operation, stage, hook.name)
            }
        }
        Ok(())
    }
}

/// Hook.
#[derive(Debug, Clone)]
pub struct Hook {
    /// Name of the hook.
    name: String,
    /// Rank of the hook.
    rank: u64,
    /// Path of the hook.
    path: PathBuf,
}

/// Loader for loading hooks.
#[derive(Debug, Clone)]
pub struct HooksLoader {
    /// Directory to load hooks from.
    directory: Cow<'static, Path>,
}

impl HooksLoader {
    /// Create a new loader for the given directory.
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory: Cow::Owned(directory),
        }
    }

    /// Load and return the hooks for a given operation.
    pub fn load_hooks(&self, operation: &'static str) -> Result<Hooks, Report<HooksLoadError>> {
        let mut stages = HashMap::new();
        match std::fs::read_dir(&self.directory.join(operation)) {
            Ok(mut read_dir) => {
                while let Some(entry) = read_dir.next() {
                    rugix_tasks::check_canceled();
                    let entry = entry.whatever("unable to read stage")?;
                    if entry.file_type().map(|t| !t.is_dir()).unwrap_or(true) {
                        // Skip any entries that are not directories.
                        continue;
                    }
                    let stage_name = entry.file_name().to_string_lossy().into_owned();
                    let stage_dir = entry.path();
                    let mut stage_hooks = Vec::new();
                    let mut read_dir = std::fs::read_dir(&stage_dir).whatever_with(|_| {
                        format!("unable to read stage hooks for \"{operation}/{stage_name}\"")
                    })?;
                    while let Some(entry) = read_dir.next() {
                        let entry = entry.whatever("unable to read hook")?;
                        if entry.file_type().map(|t| !t.is_file()).unwrap_or(true) {
                            // Skip any entries that are not files.
                            continue;
                        }
                        let hook_filename = entry.file_name();
                        let hook_filename = hook_filename.to_string_lossy();
                        let Some((hook_rank, hook_name)) = hook_filename.split_once('-') else {
                            warn!("invalid hook filename {hook_filename:?}, missing `-`");
                            continue;
                        };
                        let Ok(hook_rank) = hook_rank.parse() else {
                            warn!("invalid hook filename {hook_filename:?}, invalid rank");
                            continue;
                        };
                        let hook_path = entry.path();
                        stage_hooks.push(Hook {
                            name: hook_name.to_owned(),
                            rank: hook_rank,
                            path: hook_path,
                        });
                    }
                    stage_hooks.sort_by_key(|hook| hook.rank);
                    stages.insert(stage_name, stage_hooks);
                }
            }
            Err(error) => {
                if !matches!(error.kind(), io::ErrorKind::NotFound) {
                    return Err(error.whatever_with(|_| {
                        format!("unable to read hooks for operation {operation:?}")
                    }));
                }
                // If the directory does not exist, then there are simply no hooks.
            }
        }
        return Ok(Hooks { operation, stages });
    }
}

impl Default for HooksLoader {
    fn default() -> Self {
        Self {
            directory: Cow::Borrowed(Path::new("/etc/rugix/hooks")),
        }
    }
}
