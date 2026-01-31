use std::{path::PathBuf, str::FromStr};

use crate::scell_file::{docker::DockerImageDef, name::SCellName};

const SCELL_DEF_FROM_DELIMITER: char = '+';

#[derive(Debug, serde::Deserialize)]
pub struct SCellDef {
    pub from: FromStmt,
    #[serde(default)]
    pub run: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FromStmt {
    SCellDef {
        scell_path: Option<PathBuf>,
        scell_def_name: SCellName,
    },
    // TODO: add a separate types for `image` and `tag` fields, same as for `SCellName`
    DockerImage(DockerImageDef),
}

impl FromStr for FromStmt {
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
            None => Ok(Self::DockerImage(str.parse()?)),
        }
    }
}

impl<'de> serde::Deserialize<'de> for FromStmt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let str = String::deserialize(deserializer)?;
        str.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use test_case::test_case;

    use super::*;

    // We use a helper to make expected SCellNames in tests
    fn name(s: &str) -> SCellName {
        SCellName::from_str(s).unwrap()
    }

    #[test_case("+my-cell" => FromStmt::SCellDef { 
        scell_path: None,
        scell_def_name: name("my-cell") 
    } ; "local cell")]
    #[test_case("path/to/dir+my-cell" => FromStmt::SCellDef { 
        scell_path: Some(PathBuf::from("path/to/dir")), 
        scell_def_name: name("my-cell") 
    } ; "path and cell")]
    #[test_case("debian:12" => FromStmt::DockerImage(DockerImageDef { 
        image: "debian".to_string(), 
        tag: Some("12".to_string()) 
    }) ; "docker with tag")]
    #[test_case("scratch" => FromStmt::DockerImage(DockerImageDef { 
        image: "scratch".to_string(), 
        tag: None
    }) ; "docker image only")]
    fn test_from_parsing(input: &str) -> FromStmt {
        FromStmt::from_str(input).expect("Should be a valid input")
    }
}
