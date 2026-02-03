use std::{fmt::Write, path::PathBuf};

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

impl Link {
    pub fn to_dockerfile(
        &self,
        dockerfile: &mut String,
    ) {
        match self {
            Link::Root(root) => {
                let _ = writeln!(dockerfile, "FROM {root}");
            },
            Link::Node {
                build: run, copy, ..
            } => {
                copy.to_dockerfile(dockerfile);
                run.to_dockerfile(dockerfile);
            },
        }
    }
}
