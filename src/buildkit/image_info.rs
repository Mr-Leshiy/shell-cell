use std::{path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};
use color_eyre::eyre::ContextCompat;

use crate::scell::{
    METADATA_LOCATION_KEY, METADATA_TARGET_KEY, SCell, name::SCellName, types::name::TargetName,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellImageInfo {
    pub name: SCellName,
    pub orphan: bool,
    pub in_use: bool,
    pub location: Option<PathBuf>,
    pub target: Option<TargetName>,
    pub created_at: Option<DateTime<Utc>>,
    // An image id, not a 'scell-*' name
    pub image_id: String,
}

impl SCellImageInfo {
    pub fn new(
        name: SCellName,
        in_use: bool,
        target: Option<TargetName>,
        location: Option<PathBuf>,
        created_at: Option<DateTime<Utc>>,
        image_id: String,
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
            true
        };

        Self {
            name,
            in_use,
            orphan,
            location,
            target,
            created_at,
            image_id,
        }
    }
}

impl TryFrom<bollard::secret::ImageSummary> for SCellImageInfo {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: bollard::secret::ImageSummary) -> Result<Self, Self::Error> {
        let [image_tag] = value.repo_tags.as_slice() else {
            color_eyre::eyre::bail!("'Shell-Cell' image must have only one tag");
        };
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
            .get(METADATA_TARGET_KEY)
            .map(|s| TargetName::from_str(s.as_str()))
            .transpose()?;

        let location = value.labels.get(METADATA_LOCATION_KEY).map(PathBuf::from);

        let image_id = value.id;

        let name = image_name.parse()?;

        let orphan = if let Some(ref location) = location
            && let Some(ref target) = target
        {
            // Determine if the container is orphaned by comparing the container name
            // with the expected SCell name
            SCell::compile(location, Some(target.clone()))
                .and_then(|scell| Ok(scell.name()? != name))
                // If compilation fails, consider it orphaned
                .unwrap_or(true)
        } else {
            true
        };

        Ok(Self {
            name,
            in_use,
            orphan,
            location,
            target,
            created_at,
            image_id,
        })
    }
}
