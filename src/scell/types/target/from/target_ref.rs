use std::{path::PathBuf, str::FromStr};

use crate::scell::types::name::TargetName;

const TARGET_REF_DELIMITER: char = '+';

#[derive(Debug, thiserror::Error)]
#[error("Target reference must be in the format '[<path_to_the_blueprint>]+<target_name>', provided: {0}")]
pub struct TargetRefParsingError(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetRef {
    location: Option<PathBuf>,
    name: TargetName,
}

impl FromStr for TargetRef {
    type Err = color_eyre::eyre::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.split_once(TARGET_REF_DELIMITER) {
            Some(("", suffix)) => {
                Ok(Self {
                    location: None,
                    name: suffix.parse()?,
                })
            },
            Some((prefix, suffix)) => {
                Ok(Self {
                    location: PathBuf::from_str(prefix).map(Some)?,
                    name: suffix.parse()?,
                })
            },
            _ => color_eyre::eyre::bail!(TargetRefParsingError(str.to_string())),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TargetRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let str = String::deserialize(deserializer)?;
        str.parse().map_err(serde::de::Error::custom)
    }
}
