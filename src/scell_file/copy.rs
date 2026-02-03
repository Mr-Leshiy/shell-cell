use std::{ops::Deref, path::PathBuf};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct CopyStmt(pub Vec<Vec<PathBuf>>);

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CopyStmtEntry(Vec<PathBuf>);

impl Deref for CopyStmtEntry {
    type Target = Vec<PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for CopyStmtEntry {
    fn from(s: &str) -> Self {
        Self(s.split_whitespace().map(PathBuf::from).collect())
    }
}

impl<'de> serde::Deserialize<'de> for CopyStmtEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let str = String::deserialize(deserializer)?;
        Ok(str.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use test_case::test_case;

    use super::*;

    #[test_case("entry" => CopyStmtEntry(
        vec![
            PathBuf::from("entry")
        ]
    )
    ; "one entry")]
    #[test_case("entry1 entry2" => CopyStmtEntry(
        vec![
            PathBuf::from("entry1"),
            PathBuf::from("entry2"),
        ]
    )
    ; "two entries")]
    #[test_case("       entry1     entry2      " => CopyStmtEntry(
        vec![
            PathBuf::from("entry1"),
            PathBuf::from("entry2"),
        ]
    )
    ; "two entries more spaces")]
    fn test_from_parsing(input: &str) -> CopyStmtEntry {
        input.into()
    }
}
