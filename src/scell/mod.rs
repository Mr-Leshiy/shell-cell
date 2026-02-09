//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

mod compile;
pub mod container_info;
mod image;
mod parser;

use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use hex::ToHex;

use self::parser::{
    name::TargetName,
    target::{
        build::BuildStmt, copy::CopyStmt, image::ImageDef, shell::ShellStmt,
        workspace::WorkspaceStmt,
    },
};
use crate::scell::parser::target::config::{ConfigStmt, mounts::MountsStmt};

const NAME_PREFIX: &str = "scell-";
const IMAGE_METADATA_NAME: &str = "scell-name";
const IMAGE_METADATA_LOCATION: &str = "scell-location";

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SCell {
    links: Vec<Link>,
    shell: ShellStmt,
    hang: String,
    config: Option<ConfigStmt>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Link {
    Root(ImageDef),
    Node {
        name: TargetName,
        location: PathBuf,
        workspace: WorkspaceStmt,
        copy: CopyStmt,
        build: BuildStmt,
    },
}

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.shell.0
    }

    pub fn mounts(&self) -> MountsStmt {
        self.config
            .as_ref()
            .map(|c| c.mounts.clone())
            .unwrap_or_default()
    }

    pub fn name(&self) -> String {
        format!("{NAME_PREFIX}{}", self.hex_hash())
    }

    /// Calculates a fast, non-cryptographic 'metrohash' hash value.
    /// Returns a hex string value.
    fn hex_hash(&self) -> String {
        let mut hasher = metrohash::MetroHash64::new();
        self.hash(&mut hasher);
        hasher.finish().to_be_bytes().encode_hex()
    }
}
