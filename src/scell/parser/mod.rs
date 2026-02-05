//! Implements a parsing and processing of Shell-Cell '.yaml' files

pub mod build;
pub mod copy;
pub mod image;
pub mod name;
pub mod shell;
pub mod target;
pub mod workspace;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use self::{name::TargetName, target::TargetStmt};

#[derive(Debug)]
pub struct SCellFile {
    pub cells: HashMap<TargetName, TargetStmt>,
    pub location: PathBuf,
}

impl SCellFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
        let file: std::fs::File = std::fs::File::open(&path)?;
        let cells: HashMap<TargetName, TargetStmt> = yaml_serde::from_reader(&file)?;

        Ok(Self {
            cells,
            location: path.as_ref().to_path_buf(),
        })
    }
}
