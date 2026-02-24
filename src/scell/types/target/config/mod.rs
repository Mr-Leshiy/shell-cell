use crate::scell::types::target::config::{mounts::MountsStmt, ports::PortsStmt};

pub mod mounts;
pub mod ports;

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct ConfigStmt {
    #[serde(default)]
    pub mounts: MountsStmt,
    #[serde(default)]
    pub ports: PortsStmt,
}
