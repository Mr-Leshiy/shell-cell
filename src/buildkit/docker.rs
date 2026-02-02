use std::pin::Pin;

use bollard::{
    Docker, body_full,
    container::LogOutput,
    exec::{StartExecOptions, StartExecResults},
    query_parameters::{
        BuildImageOptionsBuilder, CreateContainerOptions, CreateImageOptions,
        ListContainersOptionsBuilder,
    },
    secret::{ContainerCreateBody, ExecConfig},
};
use futures::{Stream, StreamExt};
use tokio::io::AsyncWrite;

pub async fn build_image(
    docker: &Docker,
    dockerfile_str: &str,
    image_name: &str,
    tag: &str,
) -> anyhow::Result<()> {
    const DOCKERFILE_NAME: &str = "Dockerfile";
    // Unix file mode,
    // 6 (Owner): Read (4) + Write (2) = Read & Write.
    const FILE_MODE: u32 = 0o600;

    // Create an in-memory tar archive containing the Dockerfile
    let mut header = tar::Header::new_gnu();
    header.set_path(DOCKERFILE_NAME)?;
    header.set_size(dockerfile_str.len() as u64);
    header.set_mode(FILE_MODE);
    header.set_cksum();

    let mut tar = tar::Builder::new(Vec::new());
    tar.append(&header, dockerfile_str.as_bytes())?;
    let uncompressed_tar = tar.into_inner()?;

    let options = BuildImageOptionsBuilder::new()
        .dockerfile(DOCKERFILE_NAME)
        .t(&format!("{image_name}:{tag}"))
        .rm(true)
        .forcerm(true)
        .build();

    let mut stream = docker.build_image(options, None, Some(body_full(uncompressed_tar.into())));

    while let Some(build_info) = stream.next().await {
        let info = build_info?;
        // TODO: improove logging
        println!("{info:?}");
    }

    Ok(())
}

pub async fn pull_image(
    docker: &Docker,
    image_name: &str,
    tag: &str,
) -> anyhow::Result<()> {
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
    exposed_ports: Vec<String>,
) -> anyhow::Result<()> {
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
                .build(),
        ))
        .await?;

    // if the container already exists, skip creating step
    if res.is_empty() {
        docker
            .create_container(
                Some(CreateContainerOptions {
                    name: Some(container_name.to_string()),
                    ..Default::default()
                }),
                ContainerCreateBody {
                    image: Some(buildkit_image),
                    exposed_ports: Some(exposed_ports),
                    ..Default::default()
                },
            )
            .await?;
    }

    docker.start_container(container_name, None).await?;

    Ok(())
}

type Output = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;
type Input = Pin<Box<dyn AsyncWrite + Send>>;

pub async fn container_iteractive_exec(
    docker: &Docker,
    container_name: &str,
    priveleged: bool,
    cmd: Vec<String>,
) -> anyhow::Result<(Output, Input)> {
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
        anyhow::bail!("it must be attached session, as `detach` flag was passed to `false");
    };

    // // 3. Set local terminal to raw mode
    // // This ensures keystrokes are passed directly to the container
    // let _raw = crossterm::terminal::enable_raw_mode()?;

    // // Task for Container Output -> Local Stdout
    // let output_task = tokio::spawn(async move {
    //     while let Some(Ok(msg)) = output.next().await {
    //         print!("{}", msg);
    //         std::io::stdout().flush().unwrap();
    //     }
    // });

    // // Task for Local Stdin -> Container Input
    // let input_task = tokio::spawn(async move {
    //     let mut stdin = std::io::stdin();
    //     let mut buf = [0u8; 1024];
    //     loop {
    //         let n = stdin.read(&mut buf).unwrap();
    //         if n == 0 {
    //             break;
    //         }
    //         input.write_all(&buf[..n]).await.unwrap();
    //         input.flush().await.unwrap();
    //     }
    // });

    // // Wait for the output to finish (shell exit)
    // tokio::select! {
    //     _ = output_task => (),
    //     _ = input_task => (),
    // };

    // crossterm::terminal::disable_raw_mode()?;

    Ok((output, input))
}
