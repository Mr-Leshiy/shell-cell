use std::path::PathBuf;

use crate::scell_file::{
    build::BuildStmt, copy::CopyStmt, image::ImageDef, name::SCellName, workspace::WorkspaceStmt,
};

#[derive(Debug, Hash)]
pub enum Link {
    Root(ImageDef),
    Node {
        name: SCellName,
        path: PathBuf,
        workspace: WorkspaceStmt,
        copy: CopyStmt,
        build: BuildStmt,
    },
}
