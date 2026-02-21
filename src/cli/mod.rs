//! Command Line Interface implementation

mod cleanup;
mod init;
mod ls;
mod progress;
mod run;
mod stop;

use std::{path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use color_eyre::Section;

use crate::{crate_info, error::UserError, scell::types::name::TargetName};

// 60 frames per second
const MIN_FPS: Duration = Duration::from_millis(1000 / 60);

#[derive(Parser)]
#[clap(version = crate_info::version())]
#[clap(about = crate_info::description())]
pub struct Cli {
    /// Path to the directory with 'scell.yml' file (optional),
    #[clap(value_name = "FILE", default_value = ".")]
    scell_path: PathBuf,

    /// Entry point target name to execute, instead of 'main' (optional)
    #[clap(short, long)]
    target: Option<TargetName>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a minimal `scell.yml` blueprint in the target directory
    Init {
        /// Directory to create the blueprint in (defaults to current directory)
        #[clap(value_name = "PATH", default_value = ".")]
        path: PathBuf,
    },
    /// List all existing Shell-Cell containers
    Ls,
    /// Stop all running Shell-Cell containers
    Stop,
    /// Clean up all orphan Shell-Cell containers with their corresponding images and just single images (those no
    /// longer associated with any existing Shell-Cell blueprint files).
    Cleanup,
}

impl Cli {
    pub async fn exec(self) -> color_eyre::Result<()> {
        const SUGGESTION: &str = "If you've got a second, please toss a full backtrace into your ticket—it helps us squash the bug way faster! You can grab it by running the app with `RUST_BACKTRACE=1`.";

        self.exec_inner().await.map_err(|e| {
            if e.is::<UserError>() {
                e
            } else {
                e.note(format!(
                    "Internal bug, please report it `{}/issues/new`",
                    crate_info::repository()
                ))
                .suggestion(SUGGESTION)
            }
        })?;

        Ok(())
    }

    pub async fn exec_inner(self) -> color_eyre::Result<()> {
        match self.command {
            None => run::run(self.scell_path, self.target).await?,
            Some(Commands::Init { path }) => init::init(path)?,
            Some(Commands::Ls) => ls::ls().await?,
            Some(Commands::Stop) => stop::stop().await?,
            Some(Commands::Cleanup) => cleanup::cleanup().await?,
        }
        Ok(())
    }
}
