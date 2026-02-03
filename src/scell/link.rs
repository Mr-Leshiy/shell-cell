use std::path::PathBuf;

use crate::scell_file::{build::BuildStmt, copy::CopyStmt, image::ImageDef, name::SCellName};

#[derive(Debug, Hash)]
pub enum Link {
    Root(ImageDef),
    Node {
        name: SCellName,
        path: PathBuf,
        copy: CopyStmt,
        build: BuildStmt,
    },
}
