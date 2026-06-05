mod ui;

use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, RecvError, RecvTimeoutError},
    time::Duration,
};

use super::App;
use crate::buildkit::{BuildKitD, container_info::SCellContainerInfo};

pub struct StoppingState {
    // TODO: make it private after moving ui functionality under this 'mod' scope
    pub(super) containers: HashMap<SCellContainerInfo, Option<color_eyre::Result<()>>>,
    rx: Receiver<(SCellContainerInfo, color_eyre::Result<()>)>,
}

impl StoppingState {
    pub fn stop(
        containers: Vec<SCellContainerInfo>,
        buildkit: BuildKitD,
    ) -> App {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to stop containers
        tokio::spawn({
            let containers = containers.clone();
            async move {
                for c in containers {
                    let res = buildkit.stop_container(&c).await;
                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                    drop(tx.send((c, res)));
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        App::Stopping(Self {
            containers: containers.into_iter().map(|c| (c, None)).collect(),
            rx,
        })
    }

    /// Returns true when the underlying channel has closed (all containers processed).
    pub fn try_update(
        &mut self,
        timeout: Option<Duration>,
    ) -> bool {
        if let Some(timeout) = timeout {
            match self.rx.recv_timeout(timeout) {
                Ok(update) => {
                    self.containers.insert(update.0, Some(update.1));
                    false
                },
                Err(RecvTimeoutError::Timeout) => false,
                Err(RecvTimeoutError::Disconnected) => true,
            }
        } else {
            match self.rx.recv() {
                Ok(update) => {
                    self.containers.insert(update.0, Some(update.1));
                    false
                },
                Err(RecvError) => true,
            }
        }
    }
}
