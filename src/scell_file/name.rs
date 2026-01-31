use std::{str::FromStr, sync::LazyLock};

use regex::Regex;

#[allow(clippy::expect_used)]
static SCELL_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[a-z][a-z0-9_-]*$").expect("Must be valid REGEX expression"));

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SCellName(String);

impl FromStr for SCellName {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        anyhow::ensure!(
            SCELL_NAME_REGEX.is_match(str),
            "Shell-Cell name '{str}' must matches with the REGEX pattern: {}",
            SCELL_NAME_REGEX.as_str()
        );
        Ok(Self(str.to_string()))
    }
}

impl<'de> serde::Deserialize<'de> for SCellName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let str = String::deserialize(deserializer)?;
        str.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    // Success cases
    #[test_case("simple" ; "lowercase letters only")]
    #[test_case("cell-name" ; "with hyphens")]
    #[test_case("cell_name" ; "with underscores")]
    #[test_case("v1-beta" ; "with numbers after start")]
    #[test_case("a-1-b_2" ; "complex mix")]
    fn test_scell_name_valid(input: &str) {
        assert_eq!(SCellName::from_str(input).unwrap().0, input);
    }

    // Failure cases
    #[test_case("1st-cell" ; "starts with number")]
    #[test_case("Cell" ; "contains uppercase")]
    #[test_case("cell.dot" ; "contains dot")]
    #[test_case("" ; "empty string")]
    #[test_case(" cell" ; "leading space")]
    #[test_case("cell\n" ; "contains new line")]
    fn test_scell_name_invalid(input: &str) {
        assert!(SCellName::from_str(input).is_err());
    }
}
