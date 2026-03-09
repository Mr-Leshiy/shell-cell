mod ui;

use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::{
        MIN_FPS,
        ls::app::{AppInner, AppItemSuperTrait, error_window::ErrorWindowState, ls::LsState},
    },
};

/// Holds the state while a item is being removed in the background.
///
/// The spawned task removes the item and then re-fetches
/// the full item's list, sending the result back over the channel.
pub struct RemovingState<Item> {
    pub for_removal: Item,
    pub ls_state: LsState<Item>,
    pub rx: Receiver<color_eyre::Result<Vec<Item>>>,
}

impl RemovingState<SCellContainerInfo> {
    /// Spawns a background task that removes `container` and re-fetches the list,
    /// returning a [`RemovingState`] to track progress.
    pub fn new(
        ls_state: LsState<SCellContainerInfo>,
        for_removal: SCellContainerInfo,
    ) -> Self {
        let buildkit = ls_state.buildkit.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn({
            let for_removal = for_removal.clone();
            async move {
                let res = buildkit.cleanup_container(&for_removal).await;
                let res = match res {
                    Ok(()) => buildkit.list_containers().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(res));
            }
        });
        Self {
            for_removal,
            ls_state,
            rx,
        }
    }
}

impl RemovingState<SCellImageInfo> {
    /// Spawns a background task that removes `image` and re-fetches the list,
    /// returning a [`RemovingState`] to track progress.
    pub fn new(
        ls_state: LsState<SCellImageInfo>,
        for_removal: SCellImageInfo,
    ) -> Self {
        let buildkit = ls_state.buildkit.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn({
            let for_removal = for_removal.clone();
            async move {
                let res = buildkit.cleanup_image(&for_removal).await;
                let res = match res {
                    Ok(()) => buildkit.list_images().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(res));
            }
        });
        Self {
            for_removal,
            ls_state,
            rx,
        }
    }
}

impl<Item: Clone + AppItemSuperTrait> RemovingState<Item> {
    /// Polls the background remove task for completion and returns the next app state.
    ///
    /// - [`AppInner::Removing`] — still waiting (self is returned back)
    /// - [`AppInner::Ls`] — removal succeeded; contains the refreshed item list
    pub fn try_recv(
        self,
        buildkit: &BuildKitD,
    ) -> color_eyre::Result<AppInner<Item>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(Ok(items)) => Ok(AppInner::Ls(LsState::new(items, buildkit.clone()))),
            Ok(Err(e)) => {
                Ok(AppInner::ErrorWindow(ErrorWindowState {
                    ls_state: self.ls_state,
                    message: e.to_string(),
                }))
            },
            Err(RecvTimeoutError::Timeout) => Ok(AppInner::Removing(self)),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "RemovingState channel cannot be disconnected without returning a result"
                )
            },
        }
    }
}
