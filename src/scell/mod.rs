//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

mod compile;
pub mod container;
pub mod image;
mod link;
pub mod name;
pub mod types;

use std::hash::Hash;

use crate::scell::{
    image::SCellImage,
    link::Link,
    name::SCellId,
    types::target::{
        config::{ConfigStmt, mounts::MountsStmt, ports::PortsStmt},
        shell::ShellStmt,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SCell {
    image: SCellImage,
    container: SCellContainer,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellContainer {
    shell: ShellStmt,
    config: Option<ConfigStmt>,
}

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.container.shell.0
    }

    pub fn mounts(&self) -> MountsStmt {
        self.container
            .config
            .as_ref()
            .map(|c| c.mounts.clone())
            .unwrap_or_default()
    }

    pub fn ports(&self) -> PortsStmt {
        self.container
            .config
            .as_ref()
            .map(|c| c.ports.clone())
            .unwrap_or_default()
    }

    pub fn image_id(&self) -> color_eyre::Result<SCellId> {
        SCellId::new(|hasher| {
            self.image.hash(hasher)?;
            Ok(())
        })
    }

    pub fn container_id(&self) -> color_eyre::Result<SCellId> {
        SCellId::new(|hasher| {
            self.image.hash(hasher)?;
            self.container.hash(hasher);
            Ok(())
        })
    }

    pub fn image(&self) -> &SCellImage {
        &self.image
    }

    pub fn container(&self) -> &SCellContainer {
        &self.container
    }
}
