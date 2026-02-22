//! Implements a parsing and processing of Shell-Cell '.yaml' files

pub mod args;
pub mod errors;
pub mod name;
pub mod target;

use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
};

use self::{
    errors::{FileOpenFailed, FilePathNotResolved},
    name::TargetName,
    target::TargetStmt,
};
use crate::error::WrapUserError;

pub const SCELL_YML_FILE_NAME: &str = "scell.yml";

pub const SCELL_SCHEMA: &str = include_str!("scell_schema.cue");

#[derive(Debug)]
pub struct SCellFile {
    pub cells: HashMap<TargetName, TargetStmt>,
    pub location: PathBuf,
}

impl SCellFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
        let schema = cue_rs::Value::new(SCELL_SCHEMA)?;
        schema.validate()?;

        let location = std::fs::canonicalize(&path)
            .wrap_user_err(FilePathNotResolved(path.as_ref().to_path_buf()))?;
        let file_path = location.join(SCELL_YML_FILE_NAME);

        let mut file: std::fs::File =
            std::fs::File::open(&file_path).wrap_user_err(FileOpenFailed(file_path.clone()))?;

        let mut scell_yml_str = String::new();
        file.read_to_string(&mut scell_yml_str)?;
        schema.validate_yaml(&scell_yml_str).mark_as_user_err()?;

        let cells: HashMap<TargetName, TargetStmt> =
            yaml_serde::from_str(&scell_yml_str).mark_as_user_err()?;

        Ok(Self { cells, location })
    }
}
