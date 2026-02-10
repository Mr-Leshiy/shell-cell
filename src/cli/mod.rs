//! Command Line Interface implementation

mod ls;
mod progress;
mod run;
mod stop;

use std::{path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use color_eyre::Section;

use crate::crate_info;

// 60 frames per second
const MIN_FPS: Duration = Duration::from_millis(1000 / 60);

#[derive(Parser)]
#[clap(version = crate_info::version())]
#[clap(about = crate_info::description())]
pub struct Cli {
    /// Path to the directory with 'scell.yml' file (Optional),
    #[clap(value_name = "FILE", default_value = ".")]
    pub scell_path: PathBuf,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all existing Shell-Cell containers
    Ls,
    /// Stop all running Shell-Cell containers
    Stop,
    // TODO: Implement
    // /// Clean up all orphaned containers and their corresponding images (those no longer
    // /// associated with any existing Shell-Cell source files).
    // Cleanup,
}

impl Cli {
    pub async fn exec(self) -> color_eyre::Result<()> {
        const SUGGESTION: &str = "If you've got a second, please toss a full backtrace into your ticket—it helps us squash the bug way faster! You can grab it by running the app with `RUST_BACKTRACE=1`.";

        self.exec_inner().await.map_err(|e| {
            e.note(format!(
                "Internal bug, please report it `{}/issues/new`",
                crate_info::repository()
            ))
            .suggestion(SUGGESTION)
        })?;

        Ok(())
    }

    pub async fn exec_inner(self) -> color_eyre::Result<()> {
        match self.command {
            None => self.run().await?,
            Some(Commands::Ls) => self.ls().await?,
            Some(Commands::Stop) => self.stop().await?,
        }
        Ok(())
    }
}
