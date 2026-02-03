#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod buildkit;
mod cli;
mod pty;
mod scell;
mod scell_file;

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use crate::cli::Cli;

fn scell_home_dir() -> anyhow::Result<PathBuf> {
    const SCELL_HOME_DIR: &str = ".scell";
    let scell_home = dirs::home_dir()
        .context("Current platform does not have a home directory")?
        .join(SCELL_HOME_DIR);
    std::fs::create_dir_all(&scell_home)?;
    Ok(scell_home)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;
    cli.exec().await?;
    Ok(())
}
