use std::{path::PathBuf, str::FromStr};

use crate::scell::types::name::TargetName;

const TARGET_REF_DELIMITER: char = '+';

#[derive(Debug, thiserror::Error)]
#[error(
    "Target reference must be in the format '[<path_to_the_blueprint>]+<target_name>', provided: {0}\n(maybe you've meant 'from_image')\n"
)]
pub struct TargetRefParsingError(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetRef {
    pub location: Option<PathBuf>,
    pub name: TargetName,
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
            _ => Err(color_eyre::eyre::eyre!(TargetRefParsingError(str.to_string()))),
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

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use test_case::test_case;

    use super::*;
    use crate::scell::types::name::TargetName;

    fn name(s: &str) -> TargetName {
        TargetName::from_str(s).expect("valid target name in test helper")
    }

    #[test_case("+simple" => TargetRef { location: None, name: name("simple") } ; "local target simple name")]
    #[test_case("path/to/dir+target" => TargetRef { location: Some(PathBuf::from("path/to/dir")), name: name("target") } ; "relative path and target")]
    fn parse_ok(input: &str) -> TargetRef {
        TargetRef::from_str(input).expect("Should be a valid TargetRef")
    }

    // Failure: missing '+' delimiter entirely
    #[test_case("no-plus" ; "plain string without delimiter")]
    #[test_case("debian:12" ; "docker image with tag without delimiter")]
    #[test_case("" ; "empty string")]
    fn parse_err_no_delimiter(input: &str) {
        assert!(
            TargetRef::from_str(input).is_err(),
            "Input '{input}' should fail: missing '+' delimiter"
        );
    }

    // Failure: delimiter present but target name is invalid
    #[test_case("+" ; "only delimiter with empty name")]
    #[test_case("path+" ; "path with empty name")]
    fn parse_err_invalid_name(input: &str) {
        assert!(
            TargetRef::from_str(input).is_err(),
            "Input '{input}' should fail: invalid target name after '+'"
        );
    }
}
