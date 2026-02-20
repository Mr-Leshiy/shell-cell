//! daemon client implementation for orchestrating containers and images.

mod docker;

use std::collections::HashMap;

use bollard::{
    Docker,
    secret::{ContainerCreateBody, HostConfig, PortBinding},
};

use self::docker::{build_image, start_container};
use crate::{
    buildkit::docker::{
        container_iteractive_exec, container_resize_exec, list_all_containers, pull_image,
        remove_container, remove_image, stop_container,
    },
    error::WrapUserError,
    pty::Pty,
    scell::{SCell, container_info::SCellContainerInfo},
};

pub type ImageId = String;

#[derive(Clone)]
pub struct BuildKitD {
    docker: Docker,
}

impl BuildKitD {
    /// Runs the `BuildKit` daemon as a container.
    pub async fn start() -> color_eyre::Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        docker
            .ping()
            .await
            .map_err(|_| {
                color_eyre::eyre::eyre!(
                    "Cannot connect to the Docker daemon. Is the docker daemon running?"
                )
            })
            .mark_as_user_err()?;
        Ok(Self { docker })
    }

    pub async fn build_image(
        &self,
        scell: &SCell,
        log_fn: impl Fn(String),
    ) -> color_eyre::Result<ImageId> {
        let image = scell.image()?;
        let (tar, dockerfile_path) = image.image_tar_artifact_bytes()?;
        let scell_name = scell.name()?.to_string();

        let image_id = build_image(
            &self.docker,
            &scell_name,
            "latest",
            dockerfile_path,
            tar,
            |info| {
                log_fn(info);
            },
        )
        .await
        .mark_as_user_err()?;

        Ok(image_id)
    }

    pub async fn start_container(
        &self,
        scell: &SCell,
    ) -> color_eyre::Result<()> {
        let binds: Vec<String> = scell
            .mounts()
            .0
            .iter()
            .map(|m| format!("{}:{}", m.host.display(), m.container.display()))
            .collect();

        let ports = scell.ports();

        let exposed_ports: Vec<String> = ports
            .0
            .iter()
            .map(|p| format!("{}/{}", p.container_port, p.protocol.as_str()))
            .collect();

        let port_bindings: HashMap<String, Option<Vec<PortBinding>>> = ports
            .0
            .into_iter()
            .map(|p| {
                let key = format!("{}/{}", p.container_port, p.protocol.as_str());
                let binding = PortBinding {
                    host_ip: p.host_ip,
                    host_port: Some(p.host_port),
                };
                (key, Some(vec![binding]))
            })
            .collect();

        let config = ContainerCreateBody {
            host_config: Some(HostConfig {
                binds: (!binds.is_empty()).then_some(binds),
                port_bindings: (!port_bindings.is_empty()).then_some(port_bindings),
                ..Default::default()
            }),
            exposed_ports: (!exposed_ports.is_empty()).then_some(exposed_ports),
            ..Default::default()
        };
        let scell_name = scell.name()?.to_string();
        start_container(&self.docker, &scell_name, "latest", &scell_name, config)
            .await
            .mark_as_user_err()?;
        Ok(())
    }

    pub async fn stop_container(
        &self,
        container: &SCellContainerInfo,
    ) -> color_eyre::Result<()> {
        stop_container(&self.docker, container.name.as_str()).await?;
        Ok(())
    }

    pub async fn cleanup_container(
        &self,
        container: &SCellContainerInfo,
    ) -> color_eyre::Result<()> {
        remove_container(&self.docker, container.name.as_str()).await?;
        remove_image(&self.docker, &container.image_id).await?;
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
    ) -> color_eyre::Result<Pty> {
        let (session_id, output, input) =
            container_iteractive_exec(&self.docker, &scell.name()?.to_string(), true, vec![
                scell.shell().to_string(),
            ])
            .await?;
        Ok(Pty::new(session_id, output, input))
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
