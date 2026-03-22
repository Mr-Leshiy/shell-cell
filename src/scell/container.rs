use std::hash::Hash;

use crate::scell::types::target::config::{ConfigStmt, mounts::MountsStmt, ports::PortsStmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct SCellContainer {
    config: Option<ConfigStmt>,
}

impl SCellContainer {
    pub fn new(config: Option<ConfigStmt>) -> color_eyre::Result<Self> {
        color_eyre::eyre::ensure!(
            config.as_ref().is_none_or(|config| {
                config.services.0.iter().all(|(_, service)| {
                    service
                        .config
                        .as_ref()
                        .is_none_or(|service_config| service_config.services.0.is_empty())
                })
            }),
            "Nested services are not allowed"
        );

        Ok(Self { config })
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
