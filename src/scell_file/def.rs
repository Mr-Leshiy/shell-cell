use std::{path::PathBuf, str::FromStr};

use crate::scell_file::name::SCellName;

const SCELL_DEF_FROM_DELIMITER: char = '+';
const DOCKER_IMAGE_TAG_DELIMETER: char = ':';

#[derive(Debug, serde::Deserialize)]
pub struct SCellDef {
    pub from: From,
    pub run: Vec<String>,
}

#[derive(Debug)]
pub enum From {
    SCellDef {
        scell_path: Option<PathBuf>,
        scell_def_name: SCellName,
    },
    DockerImage {
        image: String,
        tag: Option<String>,
    },
}

impl FromStr for From {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // TODO: properly verify and process all possible string inputs
        match str.split_once(SCELL_DEF_FROM_DELIMITER) {
            Some(("", suffix)) => {
                Ok(Self::SCellDef {
                    scell_path: None,
                    scell_def_name: suffix.parse()?,
                })
            },
            Some((prefix, suffix)) => {
                Ok(Self::SCellDef {
                    scell_path: PathBuf::from_str(prefix).map(Some)?,
                    scell_def_name: suffix.parse()?,
                })
            },
            None => {
                match str.split_once(DOCKER_IMAGE_TAG_DELIMETER) {
                    Some((prefix, suffix)) => {
                        Ok(Self::DockerImage {
                            image: prefix.to_string(),
                            tag: Some(suffix.to_string()),
                        })
                    },
                    None => {
                        Ok(Self::DockerImage {
                            image: str.to_string(),
                            tag: None,
                        })
                    },
                }
            },
        }
    }
}

impl<'de> serde::Deserialize<'de> for From {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let str = String::deserialize(deserializer)?;
        str.parse().map_err(serde::de::Error::custom)
    }
}
