mod ui;
use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::{
        MIN_FPS,
        ls::app::{AppInner, AppItemSuperTrait, ls::LsState},
    },
};

/// Holds the state when the user is viewing the inspect overlay.
pub struct LoadingState<Item> {
    buildkit: BuildKitD,
    rx: Receiver<color_eyre::Result<Vec<Item>>>,
}

impl LoadingState<SCellContainerInfo> {
    /// Creates a new [`LoadingState<SCellContainerInfo>`], spawning an async task
    /// that fetches the current Shell-Cell image list.
    pub fn new(buildkit: BuildKitD) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
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

        Self { buildkit, rx }
    }
}

impl LoadingState<SCellImageInfo> {
    /// Creates a new [`LoadingState<SCellImageInfo>`], spawning an async task
    /// that fetches the current Shell-Cell image list.
    pub fn new(buildkit: BuildKitD) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let result = async {
                    let res = buildkit.list_images().await;
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    res
                }
                .await;
                drop(tx.send(result));
            }
        });

        Self { buildkit, rx }
    }
}

impl<Item: Clone + AppItemSuperTrait> LoadingState<Item> {
    /// Polls the background stop task for completion and returns the next app state.
    ///
    /// - [`AppInner::Loading`] — still loading (self is returned back)
    /// - [`AppInner::Ls`] — stop succeeded; contains the refreshed item list
    pub fn try_recv(self) -> color_eyre::Result<AppInner<Item>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(Ok(items)) => Ok(AppInner::Ls(LsState::new(items, self.buildkit))),
            Ok(Err(e)) => {
                color_eyre::eyre::bail!(e)
            },
            Err(RecvTimeoutError::Timeout) => Ok(AppInner::Loading(self)),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "StoppingState channel cannot be disconnected without returning a result"
                )
            },
        }
    }
}
