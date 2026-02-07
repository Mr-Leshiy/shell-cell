use std::path::PathBuf;

use crate::scell::parser::name::TargetName;

#[derive(Debug, thiserror::Error)]
#[error(
    "Cannot resolve a directory location at {0} while processing 'from' statement for target '{1}' at '{2}'"
)]
pub struct DirNotFoundFromStmt(pub PathBuf, pub TargetName, pub PathBuf);

#[derive(Debug, thiserror::Error)]
#[error(
    "Cannot load Shell-Cell file at '{0}' while processing 'from' statement for target '{1}' at '{2}'"
)]
pub struct FileLoadFromStmt(pub PathBuf, pub TargetName, pub PathBuf);

#[derive(Debug, thiserror::Error)]
#[error("Shell-Cell file '{0}' does not contain a target '{1}'")]
pub struct MissingTarget(pub TargetName, pub PathBuf);

#[derive(Debug, thiserror::Error)]
#[error("Shell-Cell file '{0}' does not contain a target '{1}'")]
pub struct CircularTargets(pub TargetName, pub PathBuf);
