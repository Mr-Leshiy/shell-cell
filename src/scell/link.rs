use std::{hash::Hash, path::PathBuf};

use crate::scell::types::{
    name::TargetName,
    target::{
        build::BuildStmt, copy::CopyStmt, env::EnvStmt, from::image::ImageDef,
        workspace::WorkspaceStmt,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(untagged)]
pub enum Link {
    Root(RootNode),
    Node {
        name: TargetName,
        location: PathBuf,
        #[serde(skip_serializing_if = "WorkspaceStmt::is_none")]
        workspace: WorkspaceStmt,
        env: EnvStmt,
        copy: CopyStmt,
        build: BuildStmt,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(untagged)]
pub enum RootNode {
    Image(ImageDef),
    Dockerfile(PathBuf),
}
