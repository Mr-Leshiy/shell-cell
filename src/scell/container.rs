use crate::scell::types::target::{config::ConfigStmt, shell::ShellStmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct SCellContainer {
    shell: ShellStmt,
    config: Option<ConfigStmt>,
}
