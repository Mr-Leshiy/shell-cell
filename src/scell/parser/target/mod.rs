pub mod build;
pub mod config;
pub mod copy;
pub mod image;
pub mod shell;
pub mod workspace;

use std::{path::PathBuf, str::FromStr};

use super::{
    name::TargetName,
    target::{
        build::BuildStmt, copy::CopyStmt, image::ImageDef, shell::ShellStmt,
        workspace::WorkspaceStmt,
    },
};

const SCELL_DEF_FROM_DELIMITER: char = '+';

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct TargetStmt {
    pub from: FromStmt,
    #[serde(default)]
    pub workspace: WorkspaceStmt,
    #[serde(default)]
    pub build: BuildStmt,
    #[serde(default)]
    pub copy: CopyStmt,
    #[serde(default)]
    pub shell: Option<ShellStmt>,
    pub hang: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FromStmt {
    TargetRef {
        location: Option<PathBuf>,
        name: TargetName,
    },
    Image(ImageDef),
}

impl FromStr for FromStmt {
    type Err = color_eyre::eyre::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.split_once(SCELL_DEF_FROM_DELIMITER) {
            Some(("", suffix)) => {
                Ok(Self::TargetRef {
                    location: None,
                    name: suffix.parse()?,
                })
            },
            Some((prefix, suffix)) => {
                Ok(Self::TargetRef {
                    location: PathBuf::from_str(prefix).map(Some)?,
                    name: suffix.parse()?,
                })
            },
            None => Ok(Self::Image(str.parse()?)),
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
    fn name(s: &str) -> TargetName {
        TargetName::from_str(s).unwrap()
    }

    #[test_case("+my-cell" => FromStmt::TargetRef { 
        location: None,
        name: name("my-cell") 
    } ; "local cell")]
    #[test_case("path/to/dir+my-cell" => FromStmt::TargetRef { 
        location: Some(PathBuf::from("path/to/dir")), 
        name: name("my-cell") 
    } ; "path and cell")]
    #[test_case("debian:12" => FromStmt::Image(ImageDef { 
        image: "debian".to_string(), 
        tag: Some("12".to_string()) 
    }) ; "docker with tag")]
    #[test_case("scratch" => FromStmt::Image(ImageDef { 
        image: "scratch".to_string(), 
        tag: None
    }) ; "docker image only")]
    fn test_from_parsing(input: &str) -> FromStmt {
        FromStmt::from_str(input).expect("Should be a valid input")
    }
}
