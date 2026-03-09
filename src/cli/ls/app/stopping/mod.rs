mod ui;

use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::{
    buildkit::container_info::SCellContainerInfo,
    cli::{
        MIN_FPS,
        ls::app::{AppInner, AppItemSuperTrait, error_window::ErrorWindowState, ls::LsState},
    },
};

/// Holds the state while a item is being stopped in the background.
///
/// The spawned task stops the item and then re-fetches the full
/// items list, sending the result back over the channel.
pub struct StoppingState<Item> {
    pub for_stop: Item,
    pub ls_state: LsState<Item>,
    pub rx: Receiver<color_eyre::Result<Vec<Item>>>,
}

impl StoppingState<SCellContainerInfo> {
    /// Spawns a background task that stops `container` and re-fetches the list,
    /// returning a [`StoppingState`] to track progress.
    pub fn new(
        ls_state: LsState<SCellContainerInfo>,
        for_stop: SCellContainerInfo,
    ) -> Self {
        let buildkit = ls_state.buildkit.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn({
            let container = for_stop.clone();
            async move {
                let res = buildkit.stop_container(&container).await;
                let res = match res {
                    Ok(()) => buildkit.list_containers().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(res));
            }
        });
        Self {
            for_stop,
            ls_state,
            rx,
        }
    }
}

impl<Item: Clone + AppItemSuperTrait> StoppingState<Item> {
    /// Polls the background stop task for completion and returns the next app state.
    ///
    /// - [`AppInner::Stopping`] — still waiting (self is returned back)
    /// - [`AppInner::Ls`] — stop succeeded; contains the refreshed item list
    pub fn try_recv(self) -> color_eyre::Result<AppInner<Item>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(Ok(items)) => Ok(AppInner::Ls(LsState::new(items, self.ls_state.buildkit))),
            Ok(Err(e)) => {
                Ok(AppInner::ErrorWindow(ErrorWindowState {
                    ls_state: self.ls_state,
                    message: e.to_string(),
                }))
            },
            Err(RecvTimeoutError::Timeout) => Ok(AppInner::Stopping(self)),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "StoppingState channel cannot be disconnected without returning a result"
                )
            },
        }
    }
}
