use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use crate::scell::{
    image::SCellImage,
    types::target::config::{
        ConfigStmt, mounts::MountsStmt, ports::PortsStmt, services::ServiceName,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SCellContainer {
    config: Option<ConfigStmt>,
    #[serde(skip)]
    services: BTreeMap<ServiceName, Service>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    pub image: SCellImage,
    pub container: SCellContainer,
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
            "Nested services does not allowed"
        );

        Ok(Self {
            config,
            services: BTreeMap::new(),
        })
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

    pub fn services(&self) -> &BTreeMap<ServiceName, Service> {
        &self.services
    }

    pub fn hash<H: Hasher>(
        &self,
        hasher: &mut H,
    ) -> color_eyre::Result<()> {
        self.config.hash(hasher);
        for (name, service) in &self.services {
            name.hash(hasher);
            service.image.hash(hasher)?;
            service.container.hash(hasher)?;
        }
        Ok(())
    }
}
