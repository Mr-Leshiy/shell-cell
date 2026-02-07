#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct ConfigStmt {
    pub mounts: Vec<String>,
}
