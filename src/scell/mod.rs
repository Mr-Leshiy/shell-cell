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

use std::hash::{Hash, Hasher};

use hex::ToHex;

use crate::scell::{
    image::SCellImage,
    link::Link,
    name::SCellName,
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

/// Serializes `value` to YAML and encodes the result as a Rust debug-formatted
/// string so it can be stored as a single-line Docker label value.
///
/// The inverse operation is [`decode_yaml_label`].
pub fn encode_yaml_to_label(value: impl serde::Serialize) -> color_eyre::Result<String> {
    let yaml = yaml_serde::to_string(&value).map_err(|e| color_eyre::eyre::eyre!("{e}"))?;
    Ok(format!("{yaml:#?}"))
}

/// Decodes a Docker label value produced by [`encode_yaml_to_label`] back into
/// a [`yaml_serde::Value`].
///
/// The label is stored as `format!("{yaml:#?}")` — Rust's debug representation
/// of a `String` — which uses the same escape sequences as JSON strings
/// (`\n`, `\\`, `\"`, …). [`serde_json`] decodes that wrapper, then
/// [`yaml_serde`] parses the recovered YAML text.
pub fn decode_yaml_label(s: &str) -> color_eyre::Result<yaml_serde::Value> {
    let yaml: String =
        serde_json::from_str(s).map_err(|e| color_eyre::eyre::eyre!("{e}"))?;
    yaml_serde::from_str(&yaml).map_err(|e| color_eyre::eyre::eyre!("{e}"))
}

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

    /// Heavy operation, calculates name based on the `hex_hash` value
    pub fn name(&self) -> color_eyre::Result<SCellName> {
        SCellName::new(self)
    }

    pub fn image(&self) -> color_eyre::Result<SCellImage> {
        SCellImage::new(self)
    }

    /// Calculates a fast, non-cryptographic 'metrohash' hash value.
    /// Returns a hex string value.
    fn hex_hash(&self) -> color_eyre::Result<String> {
        let mut hasher = metrohash::MetroHash64::new();
        self.0.hash(&mut hasher);
        self.image()?.dump_to_string()?.hash(&mut hasher);
        Ok(hasher.finish().to_be_bytes().encode_hex())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::{decode_yaml_label, encode_yaml_to_label};

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
    fn round_trip(value: yaml_serde::Value) {
        let encoded = encode_yaml_to_label(&value).expect("encode should not fail");
        let decoded = decode_yaml_label(&encoded).expect("decode should not fail");
        assert_eq!(value, decoded);
    }
}
