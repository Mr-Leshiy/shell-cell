#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod buildkit;
mod cli;
mod pty;
mod scell;
mod scell_file;

use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::ContextCompat;

use crate::cli::Cli;

fn scell_home_dir() -> color_eyre::Result<PathBuf> {
    const SCELL_HOME_DIR: &str = ".scell";
    let scell_home = dirs::home_dir()
        .context("Current platform does not have a home directory")?
        .join(SCELL_HOME_DIR);
    std::fs::create_dir_all(&scell_home)?;
    Ok(scell_home)
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_location_section(false)
        .capture_span_trace_by_default(false)
        .display_env_section(false)
        .install()?;
    let cli = Cli::try_parse()?;
    cli.exec().await?;
    Ok(())
}
