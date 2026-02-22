use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::{
    buildkit::container_info::SCellContainerInfo,
    cli::{MIN_FPS, ls::app::ls::LsState},
};

/// Holds the state while a container is being stopped in the background.
///
/// The spawned task stops the container and then re-fetches the full
/// container list, sending the result back over the channel.
pub struct StoppingState {
    pub container_name: String,
    pub ls_state: LsState,
    pub rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
}

impl StoppingState {
    /// Polls the background stop task for completion.
    ///
    /// Returns `Some((containers, buildkit))` with the refreshed container
    /// list when the operation is done, or `None` if still in progress.
    pub fn try_recv(&mut self) -> color_eyre::Result<Option<Vec<SCellContainerInfo>>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(result) => result.map(Some),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "StoppingState channel cannot be disconnected without returning a result"
                )
            },
        }
    }
}
