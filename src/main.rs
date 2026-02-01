#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod buildkit;
mod scell;
mod scell_file;

use std::path::Path;

use crate::{buildkit::BuildKitD, scell::SCell, scell_file::SCellFile};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = Path::new("scell.yml");
    let scell_f = SCellFile::from_path(&path)?;
    let scell = SCell::build(scell_f, path.to_path_buf(), None)?;
    println!("{scell:?}");

    let _buildkit = BuildKitD::start().await?;
    Ok(())
}
