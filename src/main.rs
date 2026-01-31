#![doc = include_str!("../README.md")]
#![allow(dead_code)]

use std::path::{Path};

use crate::{scell::SCell, scell_file::SCellFile};

mod scell;
mod scell_file;

fn main() -> anyhow::Result<()> {
    let path = Path::new("scell.yml");
    let scell_f = SCellFile::from_path(&path)?;
    let scell = SCell::build(scell_f, path.to_path_buf(), None)?;
    println!("{scell:?}");
    Ok(())
}
