use std::path::PathBuf;

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Cannot resolve 'Shell-Cell' file path at '{0}'")]
pub struct FilePathNotResolved(pub PathBuf);

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Cannot open 'Shell-Cell' file at '{0}'")]
pub struct FileOpenFailed(pub PathBuf);
