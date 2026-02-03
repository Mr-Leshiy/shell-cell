//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

mod compile;
mod image;

use std::path::PathBuf;

use crate::scell_file::{
    build::BuildStmt, copy::CopyStmt, image::ImageDef, name::SCellName, shell::ShellStmt,
    workspace::WorkspaceStmt,
};

#[derive(Debug)]
pub struct SCell {
    links: Vec<Link>,
    shell: ShellStmt,
    hang: String,
}

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

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.shell.bin_path
    }
}
