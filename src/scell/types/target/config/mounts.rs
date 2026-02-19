use std::{hash::Hash, path::PathBuf, str::FromStr};

const MOUNT_DELIMETER: char = ':';

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize)]
pub struct MountsStmt(pub Vec<MountItem>);

impl Hash for MountsStmt {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        if !self.0.is_empty() {
            self.0.hash(state);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MountItem {
    pub host: PathBuf,
    pub container: PathBuf,
}

#[derive(Debug, thiserror::Error)]
#[error(
    "mount item must be in the following format '<host_path>:<container_absolute_path>', provided: {0}"
)]
pub struct MountItemParsingEror(String);

impl FromStr for MountItem {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((host, container)) = s.split_once(MOUNT_DELIMETER) {
            let host = PathBuf::from(host);
            let container = PathBuf::from(container);
            color_eyre::eyre::ensure!(container.is_absolute(), MountItemParsingEror(s.to_string()));
            Ok(Self { host, container })
        } else {
            color_eyre::eyre::bail!(MountItemParsingEror(s.to_string()));
        }
    }
}

impl<'de> serde::Deserialize<'de> for MountItem {
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
    #[test_case("/host/path:/container/path" => MountItem { 
        host: PathBuf::from("/host/path"), 
        container: PathBuf::from("/container/path") 
    } ; "valid absolute paths")]
    #[test_case("/data:/app/data" => MountItem { 
        host: PathBuf::from("/data"), 
        container: PathBuf::from("/app/data") 
    } ; "simple root paths")]
    #[test_case("relative/path:/container/path" => MountItem { 
        host: PathBuf::from("relative/path"), 
        container: PathBuf::from("/container/path") 
    } ; "host path is relative")]
    #[test_case(".:/app/data" => MountItem { 
        host: PathBuf::from("."), 
        container: PathBuf::from("/app/data") 
    } ; "empty host path")]
    fn test_mount_item_parsing_success(input: &str) -> MountItem {
        MountItem::from_str(input).expect("Should parse successfully")
    }

    // Failure cases
    #[test_case("host/path:relative/path" ; "host path is relative")]
    #[test_case("/host/path" ; "missing delimiter")]
    #[test_case("/host/path:" ; "empty container path")]
    fn test_mount_item_parsing_failure(input: &str) {
        let result = MountItem::from_str(input);
        assert!(
            result.is_err(),
            "Input '{input}' should have failed parsing"
        );
    }
}
