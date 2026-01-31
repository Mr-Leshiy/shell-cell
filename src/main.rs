#![doc = include_str!("../README.md")]

use crate::scell_file::SCellFile;

mod scell_file;

fn main() -> anyhow::Result<()> {
    let scell_f = SCellFile::from_path("scell.yml")?;

    println!("{scell_f:?}");
    Ok(())
}
