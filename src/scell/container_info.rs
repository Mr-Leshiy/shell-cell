use std::{fmt::Display, path::PathBuf, str::FromStr};

use bollard::secret::ContainerSummaryStateEnum;
use chrono::{DateTime, Utc};
use color_eyre::eyre::ContextCompat;

use super::{
    METADATA_LOCATION_KEY, METADATA_TARGET_KEY, NAME_PREFIX, SCell, parser::name::TargetName,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellContainerInfo {
    pub target: TargetName,
    pub orphan: bool,
    pub location: PathBuf,
    pub container_name: String,
    pub created_at: DateTime<Utc>,
    pub status: Status,
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
        target: TargetName,
        location: PathBuf,
        container_name: String,
        created_at: DateTime<Utc>,
        status: Status,
    ) -> color_eyre::Result<Self> {
        color_eyre::eyre::ensure!(
            container_name.contains(NAME_PREFIX),
            "'Shell-Cell' container must have a prefix {NAME_PREFIX}"
        );

        // Determine if the container is orphaned by comparing the container name
        // with the expected SCell name
        let scell = SCell::compile(&location, Some(target.clone()))?;
        let _name = scell.name();
        let orphan = SCell::compile(&location, Some(target.clone()))
            .map(|scell| scell.name() != container_name)
            .unwrap_or(true); // If compilation fails, consider it orphaned

        Ok(Self {
            target,
            orphan,
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

        let status = value.state.as_ref().map(Into::into).unwrap_or_default();

        let target = value
            .labels
            .as_ref()
            .and_then(|v| {
                v.get(METADATA_TARGET_KEY)
                    .map(|s| TargetName::from_str(s.as_str()))
            })
            .context(format!(
                "'Shell-Cell' container must have a metadata {METADATA_TARGET_KEY} item"
            ))??;

        let location = value
            .labels
            .as_ref()
            .and_then(|v| v.get(METADATA_LOCATION_KEY).map(PathBuf::from))
            .context(format!(
                "'Shell-Cell' container must have a metadata {METADATA_LOCATION_KEY} item"
            ))?;

        Self::new(target, location, container_name, created_at, status)
    }
}
