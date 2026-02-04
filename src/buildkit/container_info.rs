use anyhow::Context;
use chrono::{DateTime, Utc};

use crate::buildkit::NAME_PREFIX;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerInfo {
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<bollard::secret::ContainerSummary> for ContainerInfo {
    type Error = anyhow::Error;

    fn try_from(value: bollard::secret::ContainerSummary) -> Result<Self, Self::Error> {
        let c_names = value
            .names
            .context("'Shell-Cell' container must have a name")?;
        let [c_name] = c_names.as_slice() else {
            anyhow::bail!("'Shell-Cell' container must have only one name");
        };
        // For historic reasons, names are prefixed with a forward-slash (`/`).
        let c_name = c_name
            .strip_prefix("/")
            .context("Container name must have a '/' prefix")?;

        anyhow::ensure!(
            c_name.contains(NAME_PREFIX),
            "'Shell-Cell' container must have a prefix {NAME_PREFIX}"
        );

        anyhow::ensure!(
            value.image.is_some_and(|i_name| i_name.starts_with(c_name)),
            "'Shell-Cell' container must have an image name equals to the container's name {c_name}"
        );

        let Some(created_at) = value.created else {
            anyhow::bail!("'Shell-Cell' container must have creation timestamp");
        };

        let created_at = DateTime::from_timestamp_secs(created_at)
            .context("'Shell-Cell' container must have a valid 'created_at' timestamp")?;

        Ok(Self {
            name: c_name.to_string(),
            created_at,
        })
    }
}
