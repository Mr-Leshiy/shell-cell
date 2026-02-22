use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::cli::{MIN_FPS, ls::app::ls::LsState};

/// Holds the state while a item is being removed in the background.
///
/// The spawned task removes the item and then re-fetches
/// the full item's list, sending the result back over the channel.
pub struct RemovingState<Item> {
    pub for_removal: Item,
    pub ls_state: LsState<Item>,
    pub rx: Receiver<color_eyre::Result<Vec<Item>>>,
}

impl<Item> RemovingState<Item> {
    /// Polls the background remove task for completion.
    ///
    /// Returns `Some((containers, buildkit))` with the refreshed container
    /// list when the operation is done, or `None` if still in progress.
    pub fn try_recv(&mut self) -> color_eyre::Result<Option<Vec<Item>>> {
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
