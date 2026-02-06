//! Command Line Interface implementation

mod ls;
mod progress;
mod run;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::Section;

use crate::error::UserError;

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
    // TODO: Implement
    // /// Clean up all orphaned containers and their corresponding images (those no longer
    // /// associated with any existing Shell-Cell source files).
    // Cleanup,
}

impl Cli {
    pub async fn exec(self) -> color_eyre::Result<()> {
        let verbose = self.verbose;
        self.exec_inner().await.map_err(|e| {
            if verbose {
                e.suggestion("To enable verbose output use -v, --verbose flags")
            } else {
                match e.downcast::<UserError>() {
                    Ok(e) => e.inner(),
                    Err(e) =>  {
                        e.note(
                            format!(
                                "Internal bug, please report it `{}/issues/new`",
                                built_info::PKG_REPOSITORY
                            )
                        )
                        .suggestion("If you've got a second, please toss a full backtrace into your ticket—it helps us squash the bug way faster! You can grab it by running the app with `RUST_BACKTRACE=1`.")
                    }
                }
            }
        })?;

        Ok(())
    }

    pub async fn exec_inner(self) -> color_eyre::Result<()> {
        match self.command {
            None => self.run().await?,
            Some(Commands::Ls) => self.ls().await?,
        }
        Ok(())
    }
}
