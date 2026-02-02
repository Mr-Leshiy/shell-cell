//! `BuildKit` daemon client implementation.

mod docker;

use bollard::Docker;

use self::docker::{build_image, start_container};
use crate::{
    buildkit::docker::{container_iteractive_exec, pull_image},
    pty::PtyStdStreams,
    scell::SCell,
};

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
        log_fn: impl Fn(String),
    ) -> anyhow::Result<()> {
        build_image(
            &self.docker,
            &scell.to_dockerfile(),
            &name(scell),
            "latest",
            |info| {
                if let Some(stream) = info.stream {
                    log_fn(stream);
                }
                if let Some(status) = info.status {
                    log_fn(status);
                }
            },
        )
        .await?;
        Ok(())
    }

    pub async fn start_container(
        &self,
        scell: &SCell,
    ) -> anyhow::Result<()> {
        start_container(&self.docker, &name(scell), "latest", &name(scell), vec![]).await?;
        Ok(())
    }

    pub async fn attach_to_shell(
        &self,
        scell: &SCell,
    ) -> anyhow::Result<PtyStdStreams> {
        let (output, input) = container_iteractive_exec(&self.docker, &name(scell), true, vec![
            scell.shell().to_string(),
        ])
        .await?;
        Ok(PtyStdStreams::new(output, input))
    }
}

fn name(scell: &SCell) -> String {
    const PREFIX: &str = "scell";
    format!("{PREFIX}-{}", scell.hex_hash())
}

async fn create_and_start_buildkit_container(docker: &Docker) -> anyhow::Result<()> {
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
