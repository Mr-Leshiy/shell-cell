//! `BuildKit` daemon client implementation.

mod docker;

use bollard::{
    Docker,
    secret::{ContainerCreateBody, HostConfig},
};

use self::docker::{build_image, start_container};
use crate::{
    buildkit::docker::{
        container_iteractive_exec, container_resize_exec, list_all_containers, pull_image,
        remove_container, remove_image, stop_container,
    },
    pty::PtySession,
    scell::{SCell, container_info::SCellContainerInfo},
};

#[derive(Clone)]
pub struct BuildKitD {
    docker: Docker,
}

impl BuildKitD {
    /// Runs the `BuildKit` daemon as a container.
    pub async fn start() -> color_eyre::Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        docker.ping().await.map_err(|_| {
            color_eyre::eyre::eyre!(
                "Cannot connect to the Docker daemon. Is the docker daemon running?"
            )
        })?;
        Ok(Self { docker })
    }

    pub async fn build_image(
        &self,
        scell: &SCell,
        log_fn: impl Fn(String),
    ) -> color_eyre::Result<()> {
        let (tar, dockerfile_path) = scell.prepare_image_tar_artifact_bytes()?;
        build_image(
            &self.docker,
            &scell.name()?.to_string(),
            "latest",
            dockerfile_path,
            tar,
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
    ) -> color_eyre::Result<()> {
        let config = ContainerCreateBody {
            host_config: Some(HostConfig {
                binds: Some(
                    scell
                        .mounts()
                        .0
                        .iter()
                        .map(|m| format!("{}:{}", m.host.display(), m.container.display()))
                        .collect(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        };
        let scell_name = scell.name()?.to_string();
        start_container(&self.docker, &scell_name, "latest", &scell_name, config).await?;
        Ok(())
    }

    pub async fn stop_container(
        &self,
        scell: &SCell,
    ) -> color_eyre::Result<()> {
        stop_container(&self.docker, &scell.name()?.to_string()).await?;
        Ok(())
    }

    pub async fn stop_container_by_name(
        &self,
        container_name: &str,
    ) -> color_eyre::Result<()> {
        stop_container(&self.docker, container_name).await?;
        Ok(())
    }

    pub async fn cleanup_container_by_name(
        &self,
        name: &str,
    ) -> color_eyre::Result<()> {
        remove_container(&self.docker, name).await?;
        remove_image(&self.docker, &format!("{name}:latest")).await?;
        Ok(())
    }

    pub async fn list_containers(&self) -> color_eyre::Result<Vec<SCellContainerInfo>> {
        Ok(list_all_containers(&self.docker)
            .await?
            .into_iter()
            .filter_map(|v| SCellContainerInfo::try_from(v).ok())
            .collect())
    }

    pub async fn attach_to_shell(
        &self,
        scell: &SCell,
    ) -> color_eyre::Result<PtySession> {
        let (session_id, output, input) =
            container_iteractive_exec(&self.docker, &scell.name()?.to_string(), true, vec![
                scell.shell().to_string(),
            ])
            .await?;
        Ok(PtySession::new(session_id, output, input))
    }

    pub async fn resize_shell(
        &self,
        session_id: &str,
        height: u16,
        width: u16,
    ) -> color_eyre::Result<()> {
        container_resize_exec(&self.docker, session_id, height, width).await
    }
}

async fn create_and_start_buildkit_container(docker: &Docker) -> color_eyre::Result<()> {
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
        ContainerCreateBody::default(),
    )
    .await?;
    Ok(())
}
