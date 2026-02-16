#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct BuildStmt(pub Vec<String>);
