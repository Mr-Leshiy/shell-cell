#![allow(dead_code)]

use std::path::{Path, PathBuf};

use super::{
    CUE_CTX,
    errors::{FileOpenFailed, FilePathNotResolved},
};
use crate::error::WrapUserError;

pub const SCELL_ARGS_FILE_NAME: &str = ".scell_args.cue";

#[derive(Debug)]
pub struct SCellExtraArguments {
    file: Option<SCellExtraArgumentsFile>,
}

/// The parsed contents of a `.scell_args` CUE file.
#[derive(Debug)]
struct SCellExtraArgumentsFile {
    value: cue_rs::Value,
    location: PathBuf,
}

impl SCellExtraArguments {
    /// Creates a [`SCellExtraArguments`] with no extra arguments loaded.
    pub fn new_emtpy() -> Self {
        Self { file: None }
    }

    /// Reads and compiles the `.scell_args` CUE file found in the given directory
    /// into a [`SCellExtraArguments`].
    pub fn from_path<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
        let location = std::fs::canonicalize(&path)
            .wrap_user_err(FilePathNotResolved(path.as_ref().to_path_buf()))?;
        let location = location.join(SCELL_ARGS_FILE_NAME);
        let file = match std::fs::read(&location) {
            Ok(b) => {
                let value = cue_rs::Value::compile_bytes(&CUE_CTX, &b).mark_as_user_err()?;
                value.is_valid().mark_as_user_err()?;
                Some(SCellExtraArgumentsFile { value, location })
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e).wrap_user_err(FileOpenFailed(location))?,
        };
        Ok(Self { file })
    }

    /// Returns the compiled CUE value.
    pub fn cue_value(&self) -> Option<&cue_rs::Value> {
        self.file.as_ref().map(|f| &f.value)
    }
}
