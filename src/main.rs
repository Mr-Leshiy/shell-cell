#![doc = include_str!("../README.md")]
#![allow(dead_code)]

use crate::scell_file::SCellFile;

mod scell;
mod scell_file;

fn main() -> anyhow::Result<()> {
    let _scell_f = SCellFile::from_path("scell.yml")?;
    Ok(())
}
