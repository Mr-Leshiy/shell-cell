//! BuildKit daemon client implementation.

use bollard::{
    Docker,
    query_parameters::{CreateContainerOptions, CreateImageOptions, ListContainersOptionsBuilder},
    secret::ContainerCreateBody,
};
use futures::StreamExt;

const BUILDKIT_CONTAINER_NAME: &str = "shell-cell-buildkitd";
const BUILDKIT_CONTAINER_PORT: &str = "8372/tcp";

pub struct BuildKitD {
    docker: Docker,
}

impl BuildKitD {
    /// Runs the `BuildKit` daemon as a container.
    pub async fn start() -> anyhow::Result<Self> {
        let mut docker = Docker::connect_with_local_defaults()?;
        docker.ping().await.map_err(|_| {
            anyhow::anyhow!("Cannot connect to the Docker daemon. Is the docker daemon running?")
        })?;
        create_and_start_buildkitd_container(&mut docker).await?;
        Ok(Self { docker })
    }
}

async fn create_and_start_buildkitd_container(docker: &mut Docker) -> anyhow::Result<()> {
    const BUILDKIT_IMAGE: &str = "moby/buildkit";
    const BUILDKIT_VERSION: &str = "v0.27.1";

    // pulling 'moby/buildkit' image from the registry
    let mut stream = docker.create_image(
        Some(CreateImageOptions {
            from_image: Some(BUILDKIT_IMAGE.to_string()),
            tag: Some(BUILDKIT_VERSION.to_string()),
            ..Default::default()
        }),
        None,
        None,
    );
    while let Some(pulling_info) = stream.next().await {
        let info = pulling_info?;
        // TODO: improove logging
        println!("{info:?}");
    }

    let buildkit_image = format!("{BUILDKIT_IMAGE}:{BUILDKIT_VERSION}");

    let res = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::default()
                .filters(
                    &[
                        ("name", vec![BUILDKIT_CONTAINER_NAME.to_string()]),
                        ("ancestor", vec![buildkit_image.clone()]),
                    ]
                    .into_iter()
                    .collect(),
                )
                .build(),
        ))
        .await?;

    // if the container already exists, skip creating step
    if res.is_empty() {
        docker
            .create_container(
                Some(CreateContainerOptions {
                    name: Some(BUILDKIT_CONTAINER_NAME.to_string()),
                    ..Default::default()
                }),
                ContainerCreateBody {
                    image: Some(buildkit_image),
                    exposed_ports: Some(vec![BUILDKIT_CONTAINER_PORT.to_string()]),
                    ..Default::default()
                },
            )
            .await?;
    }

    docker
        .start_container(BUILDKIT_CONTAINER_NAME, None)
        .await?;

    Ok(())
}
