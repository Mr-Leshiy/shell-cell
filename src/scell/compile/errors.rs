use std::path::PathBuf;

use crate::scell::parser::name::TargetName;

#[derive(Debug, PartialEq, thiserror::Error)]
#[error(
    "Cannot resolve a directory location at {0} while processing 'from' statement for target '{1}' at '{2}'"
)]
pub struct DirNotFoundFromStmt(pub PathBuf, pub TargetName, pub PathBuf);

#[derive(Debug, PartialEq, thiserror::Error)]
#[error(
    "Cannot load Shell-Cell file at '{0}' while processing 'from' statement for target '{1}' at '{2}'"
)]
pub struct FileLoadFromStmt(pub PathBuf, pub TargetName, pub PathBuf);

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Shell-Cell file '{0}' does not contain a target '{1}'")]
pub struct MissingTarget(pub TargetName, pub PathBuf);

#[derive(Debug, PartialEq, thiserror::Error)]
#[error(
    "A circular dependency was identified within the target graph. While processing 'from' statement for '{0}' at '{1}'"
)]
pub struct CircularTargets(pub TargetName, pub PathBuf);

#[derive(Debug, PartialEq, thiserror::Error)]
#[error(
    "Cannot resolve a 'mount' host path location at {0} while processing 'config' statement for target '{1}' at '{2}'"
)]
pub struct MountHostDirNotFound(pub PathBuf, pub TargetName, pub PathBuf);

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Shell-Cell file '{0}' does not contain an entrypoint target '{1}'")]
pub struct MissingEntrypoint(pub PathBuf, pub TargetName);

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Shell-Cell must have 'shell' statement in some target")]
pub struct MissingShellStmt;

#[derive(Debug, PartialEq, thiserror::Error)]
#[error("Shell-Cell must have 'hang' statement in some target")]
pub struct MissingHangStmt;
