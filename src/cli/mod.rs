//! Command Line Interface implementation

mod ls;
mod progress;
mod run;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[allow(clippy::doc_markdown)]
/// Binary build info
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser)]
#[clap(version = built_info::PKG_VERSION)]
#[clap(about = built_info::PKG_DESCRIPTION)]
pub struct Cli {
    /// Path to the 'scell.yml' file (Optional),
    #[clap(value_name = "FILE", default_value = "./scell.yml")]
    pub scell_path: PathBuf,

    /// Show detailed logs
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all existing Shell-Cell containers
    Ls,
}

impl Cli {
    pub async fn exec(self) -> anyhow::Result<()> {
        let verbose = self.verbose;
        self.exec_inner().await.map_err(|e| {
            if verbose {
                e
            } else {
                anyhow::anyhow!("{e}\n To enable verbose output use -v, --verbose flags")
            }
        })?;

        Ok(())
    }

    pub async fn exec_inner(self) -> anyhow::Result<()> {
        match self.command {
            None => self.run().await?,
            Some(Commands::Ls) => self.ls().await?,
        }
        Ok(())
    }
}
