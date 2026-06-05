mod ui;

use std::{
    sync::mpsc::{Receiver, RecvError, RecvTimeoutError},
    time::Duration,
};

use super::{App, StoppingState};
use crate::buildkit::{BuildKitD, container_info::SCellContainerInfo};

pub struct LoadingState {
    rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
    buildkit: BuildKitD,
}

impl LoadingState {
    pub fn load(buildkit: BuildKitD) -> App {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to fetch containers for stop
        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let result = async {
                    let res = buildkit.list_containers().await;
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    res
                }
                .await;
                drop(tx.send(result));
            }
        });

        App::Loading(LoadingState { rx, buildkit })
    }

    pub fn try_recv(
        self,
        timeout: Option<Duration>,
    ) -> color_eyre::Result<App> {
        if let Some(timeout) = timeout {
            match self.rx.recv_timeout(timeout) {
                Ok(Ok(items)) => Ok(StoppingState::stop(items, self.buildkit)),
                Ok(Err(e)) => {
                    color_eyre::eyre::bail!(e)
                },
                Err(RecvTimeoutError::Timeout) => Ok(App::Loading(self)),
                Err(RecvTimeoutError::Disconnected) => {
                    color_eyre::eyre::bail!(
                        "LoadingState channel cannot be disconnected without returning a result"
                    )
                },
            }
        } else {
            match self.rx.recv() {
                Ok(Ok(items)) => Ok(StoppingState::stop(items, self.buildkit)),
                Ok(Err(e)) => {
                    color_eyre::eyre::bail!(e)
                },
                Err(RecvError) => {
                    color_eyre::eyre::bail!(
                        "LoadingState channel cannot be disconnected without returning a result"
                    )
                },
            }
        }
    }
}
