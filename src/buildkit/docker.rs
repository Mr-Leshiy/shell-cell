use std::pin::Pin;

use bollard::{
    Docker, body_full,
    container::LogOutput,
    exec::{ResizeExecOptions, StartExecOptions, StartExecResults},
    query_parameters::{
        BuildImageOptionsBuilder, CreateContainerOptions, CreateImageOptions,
        ListContainersOptionsBuilder, ListImagesOptionsBuilder, RemoveContainerOptionsBuilder,
        RemoveImageOptionsBuilder,
    },
    secret::{ContainerCreateBody, ContainerSummary, ExecConfig, ImageSummary},
};
use bytes::Bytes;
use color_eyre::eyre::ContextCompat;
use futures::{Stream, StreamExt};
use tokio::io::AsyncWrite;

pub async fn build_image(
    docker: &Docker,
    image_name: &str,
    tag: &str,
    dockerfile_path: &str,
    tar_bytes: Bytes,
    log_fn: impl Fn(String),
) -> color_eyre::Result<String> {
    let options = BuildImageOptionsBuilder::new()
        .dockerfile(dockerfile_path)
        .t(&format!("{image_name}:{tag}"))
        .rm(true)
        .forcerm(true)
        .build();

    let mut stream = docker.build_image(options, None, Some(body_full(tar_bytes)));

    let mut image_id = None;
    while let Some(build_info) = stream.next().await {
        let build_info = build_info?;
        if let Some(status) = build_info.status {
            log_fn(status);
        }
        if let Some(stream) = build_info.stream {
            log_fn(stream);
        }
        if let Some(aux) = build_info.aux
            && let Some(id) = aux.id
        {
            image_id = Some(id);
        }
    }
    image_id.context("If image was built sucessfully, it must has an ID")
}

pub async fn pull_image(
    docker: &Docker,
    image_name: &str,
    tag: &str,
) -> color_eyre::Result<()> {
    let mut stream = docker.create_image(
        Some(CreateImageOptions {
            from_image: Some(image_name.to_string()),
            tag: Some(tag.to_string()),
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

    Ok(())
}

pub async fn start_container(
    docker: &Docker,
    image_name: &str,
    tag: &str,
    container_name: &str,
    mut config: ContainerCreateBody,
) -> color_eyre::Result<()> {
    let buildkit_image = format!("{image_name}:{tag}");
    let res = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::new()
                .filters(
                    &[
                        ("name", vec![container_name]),
                        ("ancestor", vec![&buildkit_image]),
                    ]
                    .into_iter()
                    .collect(),
                )
                .all(true)
                .build(),
        ))
        .await?;

    // if the container already exists, skip creating step
    config.image = Some(buildkit_image);
    if res.is_empty() {
        docker
            .create_container(
                Some(CreateContainerOptions {
                    name: Some(container_name.to_string()),
                    ..Default::default()
                }),
                config,
            )
            .await?;
    }
    docker.start_container(container_name, None).await?;

    Ok(())
}

pub async fn stop_container(
    docker: &Docker,
    container_name: &str,
) -> color_eyre::Result<()> {
    docker.stop_container(container_name, None).await?;
    Ok(())
}

pub async fn remove_container(
    docker: &Docker,
    container_name: &str,
) -> color_eyre::Result<()> {
    let config = RemoveContainerOptionsBuilder::default().force(true).build();
    docker
        .remove_container(container_name, Some(config))
        .await?;
    Ok(())
}

pub async fn remove_image(
    docker: &Docker,
    image_name: &str,
) -> color_eyre::Result<()> {
    let conifg = RemoveImageOptionsBuilder::default().force(true).build();
    docker.remove_image(image_name, Some(conifg), None).await?;
    Ok(())
}

pub async fn list_all_containers(docker: &Docker) -> color_eyre::Result<Vec<ContainerSummary>> {
    let res = docker
        .list_containers(Some(ListContainersOptionsBuilder::new().all(true).build()))
        .await?;
    Ok(res)
}

pub async fn list_all_images(docker: &Docker) -> color_eyre::Result<Vec<ImageSummary>> {
    let res = docker
        .list_images(Some(ListImagesOptionsBuilder::new().all(true).build()))
        .await?;
    Ok(res)
}

type Output = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;
type Input = Pin<Box<dyn AsyncWrite + Send>>;
type PtySessionId = String;

pub async fn container_iteractive_exec(
    docker: &Docker,
    container_name: &str,
    priveleged: bool,
    cmd: Vec<String>,
) -> color_eyre::Result<(PtySessionId, Output, Input)> {
    let config = ExecConfig {
        cmd: Some(cmd),
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        tty: Some(true),
        privileged: Some(priveleged),
        ..Default::default()
    };
    let exec_id = docker.create_exec(container_name, config).await?.id;

    let config = StartExecOptions {
        detach: false,
        tty: true,
        output_capacity: None,
    };

    let StartExecResults::Attached { output, input } =
        docker.start_exec(&exec_id, Some(config)).await?
    else {
        color_eyre::eyre::bail!(
            "it must be attached session, as `detach` flag was passed to `false"
        );
    };

    Ok((exec_id, output, input))
}

pub async fn container_resize_exec(
    docker: &Docker,
    exec_id: &str,
    height: u16,
    width: u16,
) -> color_eyre::Result<()> {
    docker
        .resize_exec(exec_id, ResizeExecOptions { height, width })
        .await?;
    Ok(())
}
