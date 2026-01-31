use std::{path::PathBuf, str::FromStr};

use crate::scell_file::name::SCellName;

const SCELL_DEF_FROM_DELIMITER: char = '+';
const DOCKER_IMAGE_TAG_DELIMETER: char = ':';

#[derive(Debug, serde::Deserialize)]
pub struct SCellDef {
    pub from: From,
    pub run: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum From {
    SCellDef {
        scell_path: Option<PathBuf>,
        scell_def_name: SCellName,
    },
    // TODO: add a separate types for `image` and `tag` fields, same as for `SCellName`
    DockerImage {
        image: String,
        tag: Option<String>,
    },
}

impl FromStr for From {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use std::path::PathBuf;

    // We use a helper to make expected SCellNames in tests
    fn name(s: &str) -> SCellName {
        SCellName::from_str(s).unwrap()
    }

    #[test_case("+my-cell" => From::SCellDef { 
        scell_path: None, 
        scell_def_name: name("my-cell") 
    } ; "local cell")]
    #[test_case("path/to/dir+my-cell" => From::SCellDef { 
        scell_path: Some(PathBuf::from("path/to/dir")), 
        scell_def_name: name("my-cell") 
    } ; "path and cell")]
    
    #[test_case("debian:12" => From::DockerImage { 
        image: "debian".to_string(), 
        tag: Some("12".to_string()) 
    } ; "docker with tag")]
    
    #[test_case("scratch" => From::DockerImage { 
        image: "scratch".to_string(), 
        tag: None 
    } ; "docker image only")]
    fn test_from_parsing(input: &str) -> From {
        From::from_str(input).expect("Should be a valid input")
    }
}