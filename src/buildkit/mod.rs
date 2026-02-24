//! daemon client implementation for orchestrating containers and images.

pub mod container_info;
mod docker;
pub mod image_info;

use std::collections::HashMap;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use bollard::{
    Docker,
    secret::{ContainerCreateBody, HostConfig, PortBinding},
};

use crate::{
    buildkit::{
        container_info::{CONTAINER_METADATA_IMAGE_ID_KEY, SCellContainerInfo},
        docker::{
            build_image, container_iteractive_exec, container_resize_exec, list_all_containers,
            list_all_images, pull_image, remove_container, remove_image, start_container,
            stop_container,
        },
        image_info::{
            IMAGE_METADATA_DESCRIPTION_KEY, IMAGE_METADATA_ENTRY_POINT_KEY,
            IMAGE_METADATA_LOCATION_KEY, SCellImageInfo,
        },
    },
    error::WrapUserError,
    pty::Pty,
    scell::SCell,
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
        let (tar, dockerfile_path) = scell.image().image_tar_artifact_bytes()?;
        let labels = image_metadata(scell)?;

        let image_id = build_image(
            &self.docker,
            &scell.image_id()?.to_string(),
            "latest",
            dockerfile_path,
            tar,
            labels,
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
        start_container(
            &self.docker,
            &scell.image_id()?.to_string(),
            "latest",
            &scell.container_id()?.to_string(),
            container_config(scell)?,
        )
        .await
        .mark_as_user_err()?;
        Ok(())
    }

    pub async fn stop_container(
        &self,
        container: &SCellContainerInfo,
    ) -> color_eyre::Result<()> {
        stop_container(&self.docker, container.id.as_str()).await?;
        Ok(())
    }

    pub async fn cleanup_container(
        &self,
        container: &SCellContainerInfo,
    ) -> color_eyre::Result<()> {
        remove_container(&self.docker, container.id.as_str()).await?;
        remove_image(&self.docker, &container.docker_image_id).await?;
        Ok(())
    }

    pub async fn cleanup_image(
        &self,
        image: &SCellImageInfo,
    ) -> color_eyre::Result<()> {
        remove_image(&self.docker, &image.docker_image_id).await?;
        Ok(())
    }

    pub async fn list_containers(&self) -> color_eyre::Result<Vec<SCellContainerInfo>> {
        Ok(list_all_containers(&self.docker)
            .await?
            .into_iter()
            .filter_map(|v| SCellContainerInfo::try_from(v).ok())
            .collect())
    }

    pub async fn list_images(&self) -> color_eyre::Result<Vec<SCellImageInfo>> {
        Ok(list_all_images(&self.docker)
            .await?
            .into_iter()
            .filter_map(|v| SCellImageInfo::try_from(v).ok())
            .collect())
    }

    pub async fn attach_to_shell(
        &self,
        scell: &SCell,
    ) -> color_eyre::Result<Pty> {
        let (session_id, output, input) = container_iteractive_exec(
            &self.docker,
            &scell.container_id()?.to_string(),
            true,
            vec![scell.shell().to_string()],
        )
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

fn container_config(scell: &SCell) -> color_eyre::Result<ContainerCreateBody> {
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

    Ok(ContainerCreateBody {
        host_config: Some(HostConfig {
            binds: (!binds.is_empty()).then_some(binds),
            port_bindings: (!port_bindings.is_empty()).then_some(port_bindings),
            annotations: Some(container_metadata(scell)?),
            ..Default::default()
        }),
        exposed_ports: (!exposed_ports.is_empty()).then_some(exposed_ports),
        ..Default::default()
    })
}

fn image_metadata(scell: &SCell) -> color_eyre::Result<HashMap<String, String>> {
    Ok([
        (
            IMAGE_METADATA_LOCATION_KEY.to_string(),
            format!("{}", scell.image().location().display()),
        ),
        (
            IMAGE_METADATA_ENTRY_POINT_KEY.to_string(),
            scell.image().entry_point().to_string(),
        ),
        (
            IMAGE_METADATA_DESCRIPTION_KEY.to_string(),
            encode_object_to_metadata(scell.image())?,
        ),
    ]
    .into_iter()
    .collect())
}

fn container_metadata(scell: &SCell) -> color_eyre::Result<HashMap<String, String>> {
    Ok([(
        CONTAINER_METADATA_IMAGE_ID_KEY.to_string(),
        scell.image_id()?.to_string(),
    )]
    .into_iter()
    .collect())
}

/// Serializes `value` JSON string and which is `BASE64_URL_SAFE_NO_PAD` encoded,
/// so it can be stored as a single-line Docker label value or container annotation value.
///
/// The inverse operation is [`decode_object_from_label`].
fn encode_object_to_metadata<T: serde::Serialize>(value: T) -> color_eyre::Result<String> {
    let json = serde_json::to_string(&value)?;
    Ok(BASE64_URL_SAFE_NO_PAD.encode(json))
}

/// Decodes a Docker label value produced by [`encode_object_to_label`] back into
/// a [`T`].
fn decode_object_from_metadata<T: serde::de::DeserializeOwned>(s: &str) -> color_eyre::Result<T> {
    let json_str_bytes = BASE64_URL_SAFE_NO_PAD.decode(s)?;
    let json_str = String::from_utf8_lossy(&json_str_bytes);
    let json: serde_json::Value = serde_json::from_str(&json_str)?;
    Ok(serde_json::from_value(json)?)
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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{decode_object_from_metadata, encode_object_to_metadata};

    #[test_case(yaml_serde::Value::String("hello".into()) ; "string")]
    #[test_case(yaml_serde::Value::Bool(true)              ; "bool true")]
    #[test_case(yaml_serde::Value::Bool(false)             ; "bool false")]
    #[test_case(yaml_serde::Value::Number(yaml_serde::Number::from(42u64)) ; "integer")]
    #[test_case(yaml_serde::Value::Sequence(vec![
        yaml_serde::Value::String("a".into()),
        yaml_serde::Value::String("b".into()),
    ]) ; "sequence")]
    #[test_case({
        let mut m = yaml_serde::Mapping::new();
        m.insert(
            yaml_serde::Value::String("shell".into()),
            yaml_serde::Value::String("/bin/bash".into()),
        );
        yaml_serde::Value::Mapping(m)
    } ; "mapping")]
    #[allow(clippy::needless_pass_by_value)]
    fn round_trip(value: yaml_serde::Value) {
        let encoded = encode_object_to_metadata(&value).expect("encode should not fail");
        let decoded: yaml_serde::Value =
            decode_object_from_metadata(&encoded).expect("decode should not fail");
        assert_eq!(value, decoded);
    }
}
