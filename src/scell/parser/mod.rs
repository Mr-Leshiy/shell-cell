//! Implements a parsing and processing of Shell-Cell '.yaml' files

pub mod errors;
pub mod name;
pub mod target;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use self::{
    errors::{FileOpenFailed, FilePathNotResolved},
    name::TargetName,
    target::TargetStmt,
};
use crate::error::WrapUserError;

const SCELL_FILE_NAME: &str = "scell.yml";

#[derive(Debug)]
pub struct SCellFile {
    pub cells: HashMap<TargetName, TargetStmt>,
    pub location: PathBuf,
}

impl SCellFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
        let file_path = path.as_ref().join(SCELL_FILE_NAME);
        // Canonicalize to verify the file exists and get proper error context
        let file_path = std::fs::canonicalize(&file_path)
            .wrap_user_err(FilePathNotResolved(file_path.clone()))?;

        let file: std::fs::File =
            std::fs::File::open(&file_path).wrap_user_err(FileOpenFailed(file_path.clone()))?;
        let cells: HashMap<TargetName, TargetStmt> =
            yaml_serde::from_reader(&file).mark_as_user_err()?;

        Ok(Self {
            cells,
            location: path.as_ref().to_path_buf(),
        })
    }
}
