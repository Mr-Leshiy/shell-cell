use crate::scell::types::target::config::{ConfigStmt, mounts::MountsStmt, ports::PortsStmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct SCellContainer {
    config: Option<ConfigStmt>,
}

impl SCellContainer {
    pub fn new(config: Option<ConfigStmt>) -> Self {
        Self { config }
    }

    pub fn mounts(&self) -> MountsStmt {
        self.config
            .as_ref()
            .map(|c| c.mounts.clone())
            .unwrap_or_default()
    }

    pub fn ports(&self) -> PortsStmt {
        self.config
            .as_ref()
            .map(|c| c.ports.clone())
            .unwrap_or_default()
    }
}
