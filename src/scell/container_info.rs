use std::{fmt::Display, path::PathBuf, str::FromStr};

use bollard::secret::ContainerSummaryStateEnum;
use chrono::{DateTime, Utc};
use color_eyre::eyre::ContextCompat;

use super::{METADATA_LOCATION_KEY, METADATA_TARGET_KEY, SCell, parser::name::TargetName};
use crate::scell::name::SCellName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellContainerInfo {
    pub name: SCellName,
    pub orphan: bool,
    pub status: Status,
    pub location: Option<PathBuf>,
    pub target: Option<TargetName>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Status {
    #[default]
    Empty,
    Created,
    Running,
    Paused,
    Restarting,
    Exited,
    Removing,
    Dead,
}

impl Display for Status {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty"),
            Self::Created => write!(f, "created"),
            Self::Running => write!(f, "running"),
            Self::Paused => write!(f, "paused"),
            Self::Restarting => write!(f, "restarting"),
            Self::Exited => write!(f, "exited"),
            Self::Removing => write!(f, "removing"),
            Self::Dead => write!(f, "dead"),
        }
    }
}

impl From<&ContainerSummaryStateEnum> for Status {
    fn from(value: &ContainerSummaryStateEnum) -> Self {
        match value {
            ContainerSummaryStateEnum::EMPTY => Self::Empty,
            ContainerSummaryStateEnum::CREATED => Self::Created,
            ContainerSummaryStateEnum::RUNNING => Self::Running,
            ContainerSummaryStateEnum::PAUSED => Self::Paused,
            ContainerSummaryStateEnum::RESTARTING => Self::Restarting,
            ContainerSummaryStateEnum::EXITED => Self::Exited,
            ContainerSummaryStateEnum::REMOVING => Self::Removing,
            ContainerSummaryStateEnum::DEAD => Self::Dead,
        }
    }
}

impl SCellContainerInfo {
    pub fn new(
        name: SCellName,
        status: Status,
        target: Option<TargetName>,
        location: Option<PathBuf>,
        created_at: Option<DateTime<Utc>>,
    ) -> Self {
        let orphan = if let Some(ref location) = location
            && let Some(ref target) = target
            && created_at.is_some()
        {
            // Determine if the container is orphaned by comparing the container name
            // with the expected SCell name
            SCell::compile(location, Some(target.clone()))
                .and_then(|scell| Ok(scell.name()? != name))
                // If compilation fails, consider it orphaned
                .unwrap_or(true)
        } else {
            false
        };

        Self {
            name,
            orphan,
            status,
            location,
            target,
            created_at,
        }
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

        let created_at = value
            .created
            .map(|v| {
                DateTime::from_timestamp_secs(v)
                    .context("'Shell-Cell' container must have a valid 'created_at' timestamp")
            })
            .transpose()?;

        let status = value.state.as_ref().map(Into::into).unwrap_or_default();

        let target = value
            .labels
            .as_ref()
            .and_then(|v| {
                v.get(METADATA_TARGET_KEY)
                    .map(|s| TargetName::from_str(s.as_str()))
            })
            .transpose()?;

        let location = value
            .labels
            .as_ref()
            .and_then(|v| v.get(METADATA_LOCATION_KEY).map(PathBuf::from));

        let name = container_name.parse()?;

        Ok(Self::new(name, status, target, location, created_at))
    }
}
