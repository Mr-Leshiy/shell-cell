//! `BuildKit` daemon client implementation.

mod docker;

use bollard::Docker;

use self::docker::{build_image, create_and_start_container};

const IMAGE_PREFIX: &str = "scell";

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
        dockerfile_str: &str,
        image_name: &str,
        tag: Option<&str>,
    ) -> anyhow::Result<()> {
        build_image(
            &self.docker,
            dockerfile_str,
            &format!("{IMAGE_PREFIX}-{image_name}"),
            tag,
        )
        .await?;
        Ok(())
    }
}

async fn create_and_start_buildkit_container(docker: &mut Docker) -> anyhow::Result<()> {
    const BUILDKIT_IMAGE: &str = "moby/buildkit";
    const BUILDKIT_TAG: &str = "v0.27.1";
    const BUILDKIT_CONTAINER_NAME: &str = "shell-cell-buildkitd";
    const BUILDKIT_CONTAINER_PORT: &str = "8372/tcp";
    create_and_start_container(
        docker,
        BUILDKIT_IMAGE,
        BUILDKIT_TAG,
        BUILDKIT_CONTAINER_NAME,
        vec![BUILDKIT_CONTAINER_PORT.to_string()],
    )
    .await?;
    Ok(())
}
