use std::{hash::Hash, path::PathBuf};

use crate::scell::types::{
    name::TargetName,
    target::{
        build::BuildStmt, copy::CopyStmt, env::EnvStmt, from::image::ImageDef,
        workspace::WorkspaceStmt,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Link {
    Root(ImageDef),
    Node {
        name: TargetName,
        location: PathBuf,
        workspace: WorkspaceStmt,
        env: EnvStmt,
        copy: CopyStmt,
        build: BuildStmt,
    },
}
