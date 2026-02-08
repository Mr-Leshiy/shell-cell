//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

mod compile;
mod image;
mod parser;

use std::{path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};
use color_eyre::eyre::ContextCompat;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SCellContainerInfo {
    pub name: TargetName,
    pub location: PathBuf,
    pub container_name: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
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
}

impl SCellContainerInfo {
    pub fn new(
        name: &str,
        location: PathBuf,
        container_name: String,
        created_at: DateTime<Utc>,
        status: String,
    ) -> color_eyre::Result<Self> {
        color_eyre::eyre::ensure!(
            container_name.contains(NAME_PREFIX),
            "'Shell-Cell' container must have a prefix {NAME_PREFIX}"
        );

        Ok(Self {
            name: name.parse()?,
            location,
            container_name,
            created_at,
            status,
        })
    }
}

impl TryFrom<bollard::secret::ContainerSummary> for SCellContainerInfo {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: bollard::secret::ContainerSummary) -> Result<Self, Self::Error> {
        let c_names = value
            .names
            .context("'Shell-Cell' container must have a name")?;
        let [container_name] = c_names.as_slice() else {
            color_eyre::eyre::bail!("'Shell-Cell' container must have only one name");
        };

        color_eyre::eyre::ensure!(
            container_name.contains(NAME_PREFIX),
            "'Shell-Cell' container must have a prefix {NAME_PREFIX}"
        );
        // For historic reasons, names are prefixed with a forward-slash (`/`).
        let container_name = container_name
            .strip_prefix("/")
            .context("Container name must have a '/' prefix")?
            .to_string();

        color_eyre::eyre::ensure!(
            value
                .image
                .is_some_and(|i_name| i_name.starts_with(&container_name)),
            "'Shell-Cell' container must have an image name equals to the container's name {container_name}"
        );

        let Some(created_at) = value.created else {
            color_eyre::eyre::bail!("'Shell-Cell' container must have creation timestamp");
        };

        let created_at = DateTime::from_timestamp_secs(created_at)
            .context("'Shell-Cell' container must have a valid 'created_at' timestamp")?;

        let status = value
            .state
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();

        let name = value
            .labels
            .as_ref()
            .and_then(|v| {
                v.get(IMAGE_METADATA_NAME)
                    .map(|s| TargetName::from_str(s.as_str()))
            })
            .context(format!(
                "'Shell-Cell' container must have a metadata {IMAGE_METADATA_NAME} item"
            ))??;

        let location = value
            .labels
            .as_ref()
            .and_then(|v| v.get(IMAGE_METADATA_LOCATION).map(PathBuf::from))
            .context(format!(
                "'Shell-Cell' container must have a metadata {IMAGE_METADATA_LOCATION} item"
            ))?;

        Ok(Self {
            name,
            location,
            container_name,
            created_at,
            status,
        })
    }
}
