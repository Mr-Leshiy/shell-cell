use std::{fmt::Display, str::FromStr};

const ENV_DELIMETER: char = '=';

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct EnvStmt(pub Vec<EnvStmtItem>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvStmtItem {
    pub key: String,
    pub value: String,
}

#[derive(Debug, thiserror::Error)]
#[error(
    "env item must be in the following format '<KEY>=<VALUE>', provided: {0}"
)]
pub struct EnvStmtItemParsingError(String);

impl FromStr for EnvStmtItem {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((key, value)) = s.split_once(ENV_DELIMETER) {
            color_eyre::eyre::ensure!(
                !key.is_empty(),
                EnvStmtItemParsingError(s.to_string())
            );
            Ok(Self {
                key: key.to_string(),
                value: value.to_string(),
            })
        } else {
            color_eyre::eyre::bail!(EnvStmtItemParsingError(s.to_string()));
        }
    }
}

impl Display for EnvStmtItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

impl<'de> serde::Deserialize<'de> for EnvStmtItem {
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
    #[test_case("DB_HOST=localhost" => EnvStmtItem {
        key: "DB_HOST".to_string(),
        value: "localhost".to_string()
    } ; "simple key value")]
    #[test_case("DB_PORT=5432" => EnvStmtItem {
        key: "DB_PORT".to_string(),
        value: "5432".to_string()
    } ; "numeric value")]
    #[test_case("DB_NAME=CatalystEventDev" => EnvStmtItem {
        key: "DB_NAME".to_string(),
        value: "CatalystEventDev".to_string()
    } ; "camel case value")]
    #[test_case("DB_DESCRIPTION=\"Catalyst Event DB\"" => EnvStmtItem {
        key: "DB_DESCRIPTION".to_string(),
        value: "\"Catalyst Event DB\"".to_string()
    } ; "quoted value with spaces")]
    #[test_case("PATH=/usr/local/bin:/usr/bin" => EnvStmtItem {
        key: "PATH".to_string(),
        value: "/usr/local/bin:/usr/bin".to_string()
    } ; "value containing colons")]
    #[test_case("KEY=" => EnvStmtItem {
        key: "KEY".to_string(),
        value: String::new()
    } ; "empty value")]
    #[test_case("CONNECTION=host=localhost port=5432" => EnvStmtItem {
        key: "CONNECTION".to_string(),
        value: "host=localhost port=5432".to_string()
    } ; "value containing equals signs")]
    fn test_env_stmt_item_parsing_success(input: &str) -> EnvStmtItem {
        EnvStmtItem::from_str(input).expect("Should parse successfully")
    }

    // Failure cases
    #[test_case("" ; "empty string")]
    #[test_case("NO_EQUALS" ; "missing delimiter")]
    #[test_case("=value" ; "empty key")]
    fn test_env_stmt_item_parsing_failure(input: &str) {
        let result = EnvStmtItem::from_str(input);
        assert!(
            result.is_err(),
            "Input '{input}' should have failed parsing"
        );
    }

    // Display roundtrip
    #[test_case("DB_HOST=localhost" ; "simple roundtrip")]
    #[test_case("DB_PORT=5432" ; "numeric roundtrip")]
    #[test_case("KEY=" ; "empty value roundtrip")]
    #[test_case("PATH=/usr/local/bin:/usr/bin" ; "colons in value roundtrip")]
    fn test_env_stmt_item_display_roundtrip(input: &str) {
        let parsed = EnvStmtItem::from_str(input).expect("Should parse successfully");
        assert_eq!(parsed.to_string(), input);
    }
}
