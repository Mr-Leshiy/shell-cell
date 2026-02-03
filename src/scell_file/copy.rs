use std::{fmt::Write, path::PathBuf};

use itertools::Itertools;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CopyStmt(pub Vec<Vec<PathBuf>>);

impl CopyStmt {
    pub fn to_dockerfile(
        &self,
        dockerfile: &mut String,
    ) {
        for e in &self.0 {
            let _ = writeln!(
                dockerfile,
                "COPY {}",
                e.iter().map(|p| format!("{}", p.display())).join(" ")
            );
        }
    }
}

impl<'de> serde::Deserialize<'de> for CopyStmt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let copy = Vec::<String>::deserialize(deserializer)?;

        Ok(Self(
            copy.into_iter()
                .map(|s| s.split(' ').map(PathBuf::from).collect())
                .collect(),
        ))
    }
}
