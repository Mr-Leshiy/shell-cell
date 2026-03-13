use std::path::Path;

use super::errors::{FileOpenFailed, FilePathNotResolved};
use crate::error::WrapUserError;

pub const SCELL_ARGS_FILE_NAME: &str = ".scell_args";

/// A type wrapper over a compiled CUE value representing extra arguments
/// that can be passed to a Shell-Cell session.
#[derive(Debug)]
pub struct SCellExtraArguments(cue_rs::Value);

impl SCellExtraArguments {
    /// Reads and compiles the `.scell_args` CUE file found in the given directory
    /// into a [`SCellExtraArguments`].
    pub fn from_path<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
        let ctx = cue_rs::Ctx::new()?;

        let location = std::fs::canonicalize(&path)
            .wrap_user_err(FilePathNotResolved(path.as_ref().to_path_buf()))?;
        let file_path = location.join(SCELL_ARGS_FILE_NAME);

        let bytes = std::fs::read(&file_path).wrap_user_err(FileOpenFailed(file_path.clone()))?;

        let value = cue_rs::Value::compile_bytes(&ctx, &bytes).mark_as_user_err()?;
        value.is_valid().mark_as_user_err()?;

        Ok(Self(value))
    }
}
