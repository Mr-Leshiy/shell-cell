#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Deserialize)]
pub struct WorkspaceStmt(pub Option<String>);
