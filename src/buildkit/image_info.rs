use std::{path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};
use color_eyre::eyre::ContextCompat;

use crate::{
    buildkit::decode_object_from_metadata,
    scell::{SCell, name::SCellId, types::name::TargetName},
};

pub const IMAGE_METADATA_ENTRY_POINT_KEY: &str = "scell-target";
pub const IMAGE_METADATA_LOCATION_KEY: &str = "scell-location";
pub const IMAGE_METADATA_DESCRIPTION_KEY: &str = "scell-image-description";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellImageInfo {
    pub id: SCellId,
    pub orphan: bool,
    pub in_use: bool,
    pub location: Option<PathBuf>,
    pub target: Option<TargetName>,
    pub desc: Option<yaml_serde::Value>,
    pub created_at: Option<DateTime<Utc>>,
    // A Docker image id, not a [`SCellId`]
    pub docker_image_id: String,
}

impl TryFrom<(String, bollard::secret::ImageSummary)> for SCellImageInfo {
    type Error = color_eyre::eyre::Error;

    fn try_from(v: (String, bollard::secret::ImageSummary)) -> Result<Self, Self::Error> {
        let (image_tag, value) = v;
        let Some((image_name, "latest")) = image_tag.split_once(':') else {
            color_eyre::eyre::bail!("'Shell-Cell' image tag must be '<scell_name>:latest'");
        };

        let in_use = value.containers > 0;

        let created_at = Some(
            DateTime::from_timestamp_secs(value.created)
                .context("'Shell-Cell' image must have a valid 'created_at' timestamp")?,
        );

        let target = value
            .labels
            .get(IMAGE_METADATA_ENTRY_POINT_KEY)
            .map(|s| TargetName::from_str(s.as_str()))
            .transpose()?;

        let location = value
            .labels
            .get(IMAGE_METADATA_LOCATION_KEY)
            .map(PathBuf::from);

        let desc = value
            .labels
            .get(IMAGE_METADATA_DESCRIPTION_KEY)
            .map(|s| decode_object_from_metadata(s))
            .transpose()?;

        let docker_image_id = value.id;

        let id = image_name.parse()?;

        let orphan = if let Some(ref location) = location
            && let Some(ref target) = target
        {
            // Determine if the container is orphaned by comparing the container name
            // with the expected SCellId
            SCell::compile(location, Some(target.clone()))
                .and_then(|scell| Ok(scell.image_id()? != id))
                // If compilation fails, consider it orphaned
                .unwrap_or(true)
        } else {
            true
        };

        Ok(Self {
            id,
            orphan,
            in_use,
            location,
            target,
            desc,
            created_at,
            docker_image_id,
        })
    }
}
