//! Definition of the command line arguments.

use std::path::PathBuf;

use clap::Parser;

use crate::config::systems::Architecture;
use crate::oven::system::ReleaseInfo;
use crate::oven::BundleOpts;

/// Command line arguments.
#[derive(Debug, Parser)]
#[command(author, version = rugix_version::RUGIX_GIT_VERSION, about = None, long_about = None)]
pub struct Args {
    /// Path to the `rugix-bakery.toml` configuration file.
    #[clap(long)]
    pub config: Option<PathBuf>,
    /// The command to execute.
    #[clap(subcommand)]
    pub cmd: Command,
}

/// Commands of the CLI.
#[derive(Debug, Parser)]
pub enum Command {
    /// Build an image, layer, or update bundle.
    #[clap(subcommand)]
    Bake(BakeCommand),
    /// Run system tests.
    Test(TestCommand),
    /// Run a system in a VM.
    Run(RunCommand),
    /// List systems, recipes, and layers.
    #[clap(subcommand)]
    List(ListCommand),
    /// Pull in external repositories.
    Pull,
    /// Initialize the project from a template.
    Init(InitCommand),
    /// Spawn a shell in the Rugix Bakery Docker container.
    Shell,
    /// Control the cache of Rugix Bakery.
    #[clap(subcommand)]
    Cache(CacheCommand),
    /// Run Rugix Bundler.
    Bundler(BundlerCommand),
}

#[derive(Debug, clap::Args)]
pub struct ReleaseInfoArgs {
    #[clap(long)]
    pub release_id: Option<String>,
    #[clap(long)]
    pub release_version: Option<String>,
}

impl ReleaseInfoArgs {
    pub fn release_info(&self) -> ReleaseInfo {
        ReleaseInfo {
            system_id: self.release_id.clone(),
            system_version: self.release_version.clone(),
        }
    }
}

/// The `list` command.
#[derive(Debug, Parser)]
pub enum ListCommand {
    /// List available images.
    Systems,
}

/// The `bake` command.
#[derive(Debug, Parser)]
pub enum BakeCommand {
    /// Bake a system
    Image {
        /// The name of the system to bake.
        system: String,
        /// The output path for the resulting files.
        output: Option<PathBuf>,
        #[clap(flatten)]
        release: ReleaseInfoArgs,
        #[clap(long)]
        source_date: Option<jiff::Timestamp>,
    },
    /// Bake a layer.
    Layer {
        /// The architecture to bake the layer for.
        #[clap(long)]
        arch: Architecture,
        /// The name of the layer to bake.
        layer: String,
        #[clap(long)]
        source_date: Option<jiff::Timestamp>,
    },
    /// Bake a bundle.
    Bundle {
        system: String,
        output: Option<PathBuf>,
        /// Disable compression of the bundle.
        #[clap(flatten)]
        opts: BundleOpts,
        #[clap(flatten)]
        release: ReleaseInfoArgs,
    },
}

/// The `test` command.
#[derive(Debug, Parser)]
pub struct TestCommand {
    pub workflows: Vec<String>,
}

/// The `cache` command.
#[derive(Debug, Parser)]
pub enum CacheCommand {
    /// Clean the cache.
    Clean,
}

/// The `run` command.
#[derive(Debug, Parser)]
pub struct RunCommand {
    #[clap(flatten)]
    pub release: ReleaseInfoArgs,
    pub system: String,
}

/// The `bake` command.
#[derive(Debug, Parser)]
pub enum InternalCommand {
    MakeImage {
        config: PathBuf,
        source: PathBuf,
        image: PathBuf,
    },
}

/// The `init` command.
#[derive(Debug, Parser)]
pub struct InitCommand {
    /// Template to use.
    pub template: Option<String>,
}

/// The `bundler` command.
#[derive(Debug, Parser)]
pub struct BundlerCommand {
    #[clap(allow_hyphen_values(true))]
    pub args: Vec<String>,
}
