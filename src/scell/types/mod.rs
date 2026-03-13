//! Implements a parsing and processing of Shell-Cell '.cue' files

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

pub const SCELL_YML_FILE_NAME: &str = "scell.cue";

pub const SCELL_SCHEMA: &[u8] = include_bytes!("scell_schema.cue");

#[derive(Debug)]
pub struct SCellFile {
    pub cells: HashMap<TargetName, TargetStmt>,
    pub location: PathBuf,
}

impl SCellFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
        let ctx = cue_rs::Ctx::new()?;
        let schema = cue_rs::Value::compile_bytes(&ctx, SCELL_SCHEMA)?;
        schema.is_valid()?;

        let location = std::fs::canonicalize(&path)
            .wrap_user_err(FilePathNotResolved(path.as_ref().to_path_buf()))?;
        let file_path = location.join(SCELL_YML_FILE_NAME);

        let scell_yaml_bytes =
            std::fs::read(&file_path).wrap_user_err(FileOpenFailed(file_path.clone()))?;

        let scell_cue = cue_rs::Value::compile_bytes(&ctx, &scell_yaml_bytes)?;
        let scell_cue = cue_rs::Value::unify(&schema, &scell_cue);
        scell_cue.is_valid().mark_as_user_err()?;

        let scell_json_bytes = scell_cue.to_json_bytes()?;
        let scell_json = serde_json::from_slice(&scell_json_bytes)?;
        let cells: HashMap<TargetName, TargetStmt> = serde_json::from_value(scell_json)?;

        Ok(Self { cells, location })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_validity_test() {
        let ctx = cue_rs::Ctx::new().unwrap();
        let schema = cue_rs::Value::compile_bytes(&ctx, SCELL_SCHEMA).unwrap();
        schema.is_valid().unwrap();
    }
}
