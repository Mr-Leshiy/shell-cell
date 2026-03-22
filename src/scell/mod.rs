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
pub mod service;
pub mod types;

use std::hash::Hash;

use crate::scell::{
    container::SCellContainer,
    image::SCellImage,
    link::Link,
    name::SCellId,
    service::Service,
    types::target::{config::services::ServiceName, shell::ShellStmt},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SCell {
    image: SCellImage,
    container: SCellContainer,
    shell: ShellStmt,
    services: Vec<(ServiceName, Service)>,
}

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.shell.0
    }

    pub fn container_id(&self) -> color_eyre::Result<SCellId> {
        SCellId::new(|hasher| {
            self.image.hash(hasher)?;
            self.container.hash(hasher);
            for (name, service) in &self.services {
                name.hash(hasher);
                service.image.hash(hasher)?;
                service.container.hash(hasher);
            }
            Ok(())
        })
    }

    pub fn image(&self) -> &SCellImage {
        &self.image
    }

    pub fn container(&self) -> &SCellContainer {
        &self.container
    }

    pub fn services(&self) -> impl Iterator<Item = &(ServiceName, Service)> {
        self.services.iter()
    }
}
