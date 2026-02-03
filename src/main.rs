#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod buildkit;
mod cli;
mod pty;
mod scell;
mod scell_file;

use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;
    cli.exec().await?;
    Ok(())
}
