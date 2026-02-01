//! `BuildKit` daemon client implementation.

mod docker;

use bollard::Docker;

use self::docker::{build_image, start_container};
use crate::{buildkit::docker::pull_image, scell::SCell};

pub struct BuildKitD {
    docker: Docker,
}

impl BuildKitD {
    /// Runs the `BuildKit` daemon as a container.
    pub async fn start() -> anyhow::Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        docker.ping().await.map_err(|_| {
            anyhow::anyhow!("Cannot connect to the Docker daemon. Is the docker daemon running?")
        })?;
        Ok(Self { docker })
    }

    pub async fn build_image(
        &self,
        scell: &SCell,
    ) -> anyhow::Result<()> {
        build_image(
            &self.docker,
            &scell.to_dockerfile(),
            &image_name(scell),
            "latest",
        )
        .await?;
        Ok(())
    }

    pub async fn start_container(
        &mut self,
        scell: &SCell,
    ) -> anyhow::Result<()> {
        start_container(
            &mut self.docker,
            &image_name(scell),
            "latest",
            &image_name(scell),
            vec![],
        )
        .await?;
        Ok(())
    }
}

fn image_name(scell: &SCell) -> String {
    const IMAGE_PREFIX: &str = "scell";

    format!("{IMAGE_PREFIX}-{}", scell.hex_hash())
}

async fn create_and_start_buildkit_container(docker: &mut Docker) -> anyhow::Result<()> {
    const BUILDKIT_IMAGE: &str = "moby/buildkit";
    const BUILDKIT_TAG: &str = "v0.27.1";
    const BUILDKIT_CONTAINER_NAME: &str = "shell-cell-buildkitd";
    const BUILDKIT_CONTAINER_PORT: &str = "8372/tcp";

    pull_image(docker, BUILDKIT_IMAGE, BUILDKIT_TAG).await?;
    start_container(
        docker,
        BUILDKIT_IMAGE,
        BUILDKIT_TAG,
        BUILDKIT_CONTAINER_NAME,
        vec![BUILDKIT_CONTAINER_PORT.to_string()],
    )
    .await?;
    Ok(())
}
