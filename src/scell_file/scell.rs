use std::{path::PathBuf, str::FromStr};

use crate::scell_file::{
    copy::CopyStmt, image::ImageDef, name::SCellName, build::BuildStmt, shell::ShellStmt,
};

const SCELL_DEF_FROM_DELIMITER: char = '+';

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct SCellStmt {
    pub from: FromStmt,
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
    SCellRef {
        scell_path: Option<PathBuf>,
        scell_def_name: SCellName,
    },
    Image(ImageDef),
}

impl FromStr for FromStmt {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.split_once(SCELL_DEF_FROM_DELIMITER) {
            Some(("", suffix)) => {
                Ok(Self::SCellRef {
                    scell_path: None,
                    scell_def_name: suffix.parse()?,
                })
            },
            Some((prefix, suffix)) => {
                Ok(Self::SCellRef {
                    scell_path: PathBuf::from_str(prefix).map(Some)?,
                    scell_def_name: suffix.parse()?,
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
    fn name(s: &str) -> SCellName {
        SCellName::from_str(s).unwrap()
    }

    #[test_case("+my-cell" => FromStmt::SCellRef { 
        scell_path: None,
        scell_def_name: name("my-cell") 
    } ; "local cell")]
    #[test_case("path/to/dir+my-cell" => FromStmt::SCellRef { 
        scell_path: Some(PathBuf::from("path/to/dir")), 
        scell_def_name: name("my-cell") 
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
