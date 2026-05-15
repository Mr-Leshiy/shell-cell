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
    pub location: Option<PathBuf>,
    pub target: Option<TargetName>,
    pub desc: Option<yaml_serde::Value>,
    pub created_at: Option<DateTime<Utc>>,
    // A Docker image id, not a [`SCellId`]
    pub docker_image_id: String,
}

impl SCellImageInfo {
    pub fn image_name(id: &SCellId) -> String {
        format!("{id}:latest")
    }
}

impl TryFrom<(String, bollard::models::ImageSummary)> for SCellImageInfo {
    type Error = color_eyre::eyre::Error;

    fn try_from(v: (String, bollard::models::ImageSummary)) -> Result<Self, Self::Error> {
        let (image_tag, value) = v;
        let Some((image_name, "latest")) = image_tag.split_once(':') else {
            color_eyre::eyre::bail!("'Shell-Cell' image tag must be '<scell_name>:latest'");
        };
        // Podman qualifies names with a registry prefix (e.g. docker.io/library/scell-…);
        // strip everything up to and including the last '/' so we get a bare image name.
        let image_name = image_name
            .rsplit_once('/')
            .map_or(image_name, |(_, name)| name);

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
                .and_then(|scell| Ok(scell.image().id()? != id))
                // If compilation fails, consider it orphaned
                .unwrap_or(true)
        } else {
            true
        };

        Ok(Self {
            id,
            orphan,
            location,
            target,
            desc,
            created_at,
            docker_image_id,
        })
    }
}
