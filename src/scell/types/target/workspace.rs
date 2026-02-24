#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Deserialize, serde::Serialize)]
pub struct WorkspaceStmt(pub Option<String>);

impl WorkspaceStmt {
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}
