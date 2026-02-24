//! Shell-Cell definition.
//! Its consists of the `Link`s, smallest abstraction of the whole Shell-Cell unit.
//! Each Shell-Cell would **always** contain at least two links, the root and the node.
//! The root link is always must be a some "base" image e.g. <https://docs.docker.com/build/building/base-images/>.
//! Not necessarily the docker base image, but it must be some image which would be a
//! "base" for entire Shell-Cell.

mod compile;
pub mod image;
mod link;
pub mod name;
pub mod types;

use std::hash::Hash;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};

use crate::scell::{
    image::SCellImage,
    link::Link,
    name::SCellId,
    types::target::{
        config::{ConfigStmt, mounts::MountsStmt, ports::PortsStmt},
        hang::HangStmt,
        shell::ShellStmt,
    },
};

pub const NAME_PREFIX: &str = "scell-";
pub const METADATA_TARGET_KEY: &str = "scell-target";
pub const METADATA_LOCATION_KEY: &str = "scell-location";
pub const METADATA_DEFINITION_KEY: &str = "scell-definition";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SCell(SCellInner);

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
struct SCellInner {
    links: Vec<Link>,
    shell: ShellStmt,
    hang: HangStmt,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ConfigStmt>,
}

impl SCell {
    /// Returns an underlying shell's binary path
    pub fn shell(&self) -> &str {
        &self.0.shell.0
    }

    pub fn mounts(&self) -> MountsStmt {
        self.0
            .config
            .as_ref()
            .map(|c| c.mounts.clone())
            .unwrap_or_default()
    }

    pub fn ports(&self) -> PortsStmt {
        self.0
            .config
            .as_ref()
            .map(|c| c.ports.clone())
            .unwrap_or_default()
    }

    pub fn image_id(&self) -> color_eyre::Result<SCellId> {
        SCellId::new(|hasher| {
            self.0.hash(hasher);
            self.image()?.dump_to_string()?.hash(hasher);
            Ok(())
        })
    }

    pub fn container_id(&self) -> color_eyre::Result<SCellId> {
        SCellId::new(|hasher| {
            self.0.hash(hasher);
            self.image()?.dump_to_string()?.hash(hasher);
            Ok(())
        })
    }

    pub fn image(&self) -> color_eyre::Result<SCellImage> {
        SCellImage::new(self)
    }
}

/// Serializes `value` JSON string and which is `BASE64_URL_SAFE_NO_PAD` encoded,
/// so it can be stored as a single-line Docker label value.
///
/// The inverse operation is [`decode_object_from_label`].
pub fn encode_object_to_label<T: serde::Serialize>(value: T) -> color_eyre::Result<String> {
    let json = serde_json::to_string(&value)?;
    Ok(BASE64_URL_SAFE_NO_PAD.encode(json))
}

/// Decodes a Docker label value produced by [`encode_object_to_label`] back into
/// a [`T`].
pub fn decode_object_from_label<T: serde::de::DeserializeOwned>(s: &str) -> color_eyre::Result<T> {
    let json_str_bytes = BASE64_URL_SAFE_NO_PAD.decode(s)?;
    let json_str = String::from_utf8_lossy(&json_str_bytes);
    let json: serde_json::Value = serde_json::from_str(&json_str)?;
    Ok(serde_json::from_value(json)?)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{decode_object_from_label, encode_object_to_label};

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
        let encoded = encode_object_to_label(&value).expect("encode should not fail");
        let decoded: yaml_serde::Value =
            decode_object_from_label(&encoded).expect("decode should not fail");
        assert_eq!(value, decoded);
    }
}
