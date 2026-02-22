use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::{
    buildkit::BuildKitD,
    cli::{
        MIN_FPS,
        ls::app::{AppInner, error_state::ErrorState, ls::LsState},
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

impl<Item: Clone> StoppingState<Item> {
    /// Polls the background stop task for completion and returns the next app state.
    ///
    /// - [`AppInner::Stopping`] — still waiting (self is returned back)
    /// - [`AppInner::Ls`] — stop succeeded; contains the refreshed item list
    /// - [`AppInner::Error`] — stop failed; contains the error message
    pub fn try_recv(
        self,
        buildkit: &BuildKitD,
    ) -> color_eyre::Result<AppInner<Item>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(Ok(items)) => Ok(AppInner::Ls(LsState::new(items, buildkit.clone()))),
            Ok(Err(e)) => {
                Ok(AppInner::Error(ErrorState {
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
