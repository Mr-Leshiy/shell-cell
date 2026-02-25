use std::{fmt::Display, path::PathBuf, str::FromStr};

use bollard::secret::ContainerSummaryStateEnum;
use chrono::{DateTime, Utc};
use color_eyre::eyre::ContextCompat;

use crate::{
    buildkit::{
        decode_object_from_metadata,
        image_info::{
            IMAGE_METADATA_DESCRIPTION_KEY, IMAGE_METADATA_ENTRY_POINT_KEY,
            IMAGE_METADATA_LOCATION_KEY,
        },
    },
    scell::{SCell, name::SCellId, types::name::TargetName},
};

pub const CONTAINER_METADATA_IMAGE_ID_KEY: &str = "scell-image-id";
pub const CONTAINER_METADATA_DESCRIPTION_KEY: &str = "scell-container-description";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellContainerInfo {
    pub id: SCellId,
    pub orphan: bool,
    pub status: Status,
    pub image_id: Option<SCellId>,
    pub location: Option<PathBuf>,
    pub target: Option<TargetName>,
    pub image_desc: Option<yaml_serde::Value>,
    pub container_desc: Option<yaml_serde::Value>,
    pub created_at: Option<DateTime<Utc>>,
    // A Docker image id, not a [`SCellId`]
    pub docker_image_id: String,
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
                v.get(IMAGE_METADATA_ENTRY_POINT_KEY)
                    .map(|s| TargetName::from_str(s.as_str()))
            })
            .transpose()?;

        let location = value
            .labels
            .as_ref()
            .and_then(|v| v.get(IMAGE_METADATA_LOCATION_KEY).map(PathBuf::from));

        let image_desc = value
            .labels
            .as_ref()
            .and_then(|v| {
                v.get(IMAGE_METADATA_DESCRIPTION_KEY)
                    .map(|s| decode_object_from_metadata(s))
            })
            .transpose()?;

        let container_desc = value
            .labels
            .as_ref()
            .and_then(|v| {
                v.get(CONTAINER_METADATA_DESCRIPTION_KEY)
                    .map(|s| decode_object_from_metadata(s))
            })
            .transpose()?;

        let image_id = value
            .labels
            .as_ref()
            .and_then(|v| v.get(CONTAINER_METADATA_IMAGE_ID_KEY).map(|s| s.parse()))
            .transpose()?;

        let docker_image_id = value
            .image_id
            .context("'Shell-Cell' container must have a corresponding Docker/Podman image ID")?;

        let id = container_name.parse()?;

        let orphan = if let Some(ref location) = location
            && let Some(ref target) = target
            && created_at.is_some()
        {
            // Determine if the container is orphaned by comparing the container name
            // with the expected SCellId
            SCell::compile(location, Some(target.clone()))
                .and_then(|scell| Ok(scell.container_id()? != id))
                // If compilation fails, consider it orphaned
                .unwrap_or(true)
        } else {
            true
        };

        Ok(Self {
            id,
            orphan,
            status,
            image_id,
            location,
            target,
            image_desc,
            container_desc,
            created_at,
            docker_image_id,
        })
    }
}
