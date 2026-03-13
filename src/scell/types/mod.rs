//! Implements a parsing and processing of Shell-Cell '.cue' files

pub mod errors;
pub mod extra_arguments;
pub mod name;
pub mod target;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use self::{
    errors::{FileOpenFailed, FilePathNotResolved},
    name::TargetName,
    target::TargetStmt,
};
use crate::{error::WrapUserError, scell::types::extra_arguments::SCellExtraArguments};

pub const SCELL_CUE_FILE_NAME: &str = "scell.cue";

pub const SCELL_SCHEMA: &[u8] = include_bytes!("scell_schema.cue");

#[allow(clippy::expect_used)]
static CUE_CTX: LazyLock<cue_rs::Ctx> =
    LazyLock::new(|| cue_rs::Ctx::new().expect("Cannot initialize `cue_rs::Ctx`"));

#[derive(Debug)]
pub struct SCellFile {
    pub cells: HashMap<TargetName, TargetStmt>,
    pub location: PathBuf,
}

impl SCellFile {
    pub fn from_path<P: AsRef<Path>>(
        path: P,
        extra_args: &SCellExtraArguments,
    ) -> color_eyre::Result<Self> {
        let schema = cue_rs::Value::compile_bytes(&CUE_CTX, SCELL_SCHEMA)?;
        schema.is_valid()?;

        let location = std::fs::canonicalize(&path)
            .wrap_user_err(FilePathNotResolved(path.as_ref().to_path_buf()))?;
        let file_path = location.join(SCELL_CUE_FILE_NAME);

        let scell_yaml_bytes =
            std::fs::read(&file_path).wrap_user_err(FileOpenFailed(file_path.clone()))?;

        let mut scell_cue = cue_rs::Value::compile_bytes(&CUE_CTX, &scell_yaml_bytes)?;
        if let Some(extra_args) = extra_args.cue_value() {
            scell_cue = cue_rs::Value::unify(&scell_cue, extra_args);
        }
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
        let schema = cue_rs::Value::compile_bytes(&CUE_CTX, SCELL_SCHEMA).unwrap();
        schema.is_valid().unwrap();
    }
}
