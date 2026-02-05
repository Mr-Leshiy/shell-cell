//! Implements a parsing and processing of Shell-Cell '.yaml' files

pub mod build;
pub mod copy;
pub mod image;
pub mod name;
pub mod scell;
pub mod shell;
pub mod workspace;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use self::scell::SCellStmt;
use crate::scell_file::name::SCellName;

const SUPPORTED_VERSION: &str = "0.1";

#[derive(Debug)]
pub struct SCellFile {
    pub cells: HashMap<SCellName, SCellStmt>,
    pub location: PathBuf,
}

impl SCellFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
        #[derive(serde::Deserialize)]
        struct SCellFileYml {
            version: String,
            #[serde(flatten)]
            cells: HashMap<SCellName, SCellStmt>,
        }

        let file: std::fs::File = std::fs::File::open(&path)?;
        let scell_f: SCellFileYml = yaml_serde::from_reader(&file)?;
        color_eyre::eyre::ensure!(
            scell_f.version == SUPPORTED_VERSION,
            "Currently supported version is {SUPPORTED_VERSION}, provided {}.",
            scell_f.version
        );

        Ok(Self {
            cells: scell_f.cells,
            location: path.as_ref().to_path_buf(),
        })
    }
}
