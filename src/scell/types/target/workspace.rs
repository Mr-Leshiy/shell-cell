#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Deserialize, serde::Serialize)]
pub struct WorkspaceStmt(pub Option<String>);
