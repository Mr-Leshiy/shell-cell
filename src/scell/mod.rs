//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

mod compile;
pub mod container_info;
mod image;
mod link;
mod name;
pub mod types;

use std::hash::{Hash, Hasher};

use hex::ToHex;

use crate::scell::{
    link::Link,
    name::SCellName,
    types::target::{
        config::{ConfigStmt, mounts::MountsStmt, ports::PortsStmt},
        shell::ShellStmt,
    },
};

const NAME_PREFIX: &str = "scell-";
const METADATA_TARGET_KEY: &str = "scell-target";
const METADATA_LOCATION_KEY: &str = "scell-location";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SCell(SCellInner);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SCellInner {
    links: Vec<Link>,
    shell: ShellStmt,
    hang: String,
    config: Option<ConfigStmt>,
}

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.0.shell.0
    }

    pub fn mounts(&self) -> MountsStmt {
        self.0
            .config
            .as_ref()
            .map(|c| c.mounts.clone())
            .unwrap_or_default()
    }

    pub fn ports(&self) -> PortsStmt {
        self.0
            .config
            .as_ref()
            .map(|c| c.ports.clone())
            .unwrap_or_default()
    }

    /// Heavy operation, calculates name based on the `hex_hash` value
    pub fn name(&self) -> color_eyre::Result<SCellName> {
        SCellName::new(self)
    }

    /// Calculates a fast, non-cryptographic 'metrohash' hash value.
    /// Returns a hex string value.
    fn hex_hash(&self) -> color_eyre::Result<String> {
        let mut hasher = metrohash::MetroHash64::new();
        self.0.hash(&mut hasher);
        // self.prepare_image_tar_artifact_bytes()?.hash(&mut hasher);
        Ok(hasher.finish().to_be_bytes().encode_hex())
    }
}
