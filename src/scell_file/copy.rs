use std::path::PathBuf;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CopyStmt(pub Vec<Vec<PathBuf>>);

impl<'de> serde::Deserialize<'de> for CopyStmt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let copy = Vec::<String>::deserialize(deserializer)?;

        Ok(Self(
            copy.into_iter()
                .map(|s| s.split_whitespace().map(PathBuf::from).collect())
                .collect(),
        ))
    }
}
