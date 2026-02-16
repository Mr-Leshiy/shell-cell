use crate::scell::types::target::config::mounts::MountsStmt;

pub mod mounts;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct ConfigStmt {
    pub mounts: MountsStmt,
}
