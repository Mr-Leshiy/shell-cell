use std::{hash::Hash, path::PathBuf, str::FromStr};

use color_eyre::eyre::ContextCompat;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize)]
pub struct CopyStmt(pub Vec<CopyStmtEntry>);

impl Hash for CopyStmt {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        if !self.0.is_empty() {
            self.0.hash(state);
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CopyStmtEntry {
    pub dest: PathBuf,
    pub src: Vec<PathBuf>,
}

impl FromStr for CopyStmtEntry {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = s.split_whitespace().map(PathBuf::from).collect::<Vec<_>>();
        let dest = res.pop().context(format!(
            "'from' statement entry must have desctination path, entry: {s}"
        ))?;
        color_eyre::eyre::ensure!(
            !res.is_empty(),
            "'from' statement entry must have at least one source path, entry: {s}"
        );

        Ok(Self { dest, src: res })
    }
}

impl<'de> serde::Deserialize<'de> for CopyStmtEntry {
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

    #[test_case(
        "src1 dst"
        => CopyStmtEntry {
            src: vec![PathBuf::from("src1")],
            dest: PathBuf::from("dst"),
        }
    ; "two entries")]
    #[test_case(
        "src1 src2 dst"
        => CopyStmtEntry {
            src: vec![
                PathBuf::from("src1"),
                PathBuf::from("src2"),
            ],
            dest: PathBuf::from("dst"),
        }
    ; "three entries")]
    #[test_case(
        "       src1        src2            dst"
        => CopyStmtEntry {
            src: vec![
                PathBuf::from("src1"),
                PathBuf::from("src2"),
            ],
            dest: PathBuf::from("dst"),
        }
    ; "three entries more space")]
    fn parsing_test(input: &str) -> CopyStmtEntry {
        input.parse().unwrap()
    }
}
