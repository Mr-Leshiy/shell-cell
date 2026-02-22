use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::{
    buildkit::container_info::SCellContainerInfo,
    cli::{MIN_FPS, ls::app::ls::LsState},
};

/// Holds the state while a container is being removed in the background.
///
/// The spawned task removes the container (and its image) and then re-fetches
/// the full container list, sending the result back over the channel.
pub struct RemovingState {
    /// Name of the container being removed (used for UI display).
    pub container_name: String,
    pub ls_state: LsState<SCellContainerInfo>,
    pub rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
}

impl RemovingState {
    /// Polls the background remove task for completion.
    ///
    /// Returns `Some((containers, buildkit))` with the refreshed container
    /// list when the operation is done, or `None` if still in progress.
    pub fn try_recv(&mut self) -> color_eyre::Result<Option<Vec<SCellContainerInfo>>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(result) => result.map(Some),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "RemovingState channel cannot be disconnected without returning a result"
                )
            },
        }
    }
}
