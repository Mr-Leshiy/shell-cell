#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod buildkit;
mod cli;
mod pty;
mod scell;
mod scell_file;

use std::path::Path;

use crate::{buildkit::BuildKitD, scell::SCell, scell_file::SCellFile};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = Path::new("scell.yml");
    let scell_f = SCellFile::from_path(path)?;
    let scell = SCell::build(scell_f, path.to_path_buf(), None)?;

    let buildkit = BuildKitD::start().await?;
    buildkit.build_image(&scell).await?;
    buildkit.start_container(&scell).await?;
    let pty_streams = buildkit.run_shell(&scell).await?;

    pty::run(&pty_streams).await?;

    Ok(())
}
