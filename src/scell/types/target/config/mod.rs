use crate::scell::types::target::config::{
    mounts::MountsStmt, ports::PortsStmt, services::ServicesStmt,
};

pub mod mounts;
pub mod ports;
pub mod services;

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct ConfigStmt {
    #[serde(default)]
    pub mounts: MountsStmt,
    #[serde(default)]
    pub ports: PortsStmt,
    #[serde(default)]
    pub services: ServicesStmt,
}
