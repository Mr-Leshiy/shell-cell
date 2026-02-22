use std::sync::mpsc::{Receiver, RecvTimeoutError};

use crate::{
    buildkit::BuildKitD,
    cli::{
        MIN_FPS,
        ls::app::{AppInner, error_state::ErrorState, ls::LsState},
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

impl<Item: Clone> RemovingState<Item> {
    /// Polls the background remove task for completion and returns the next app state.
    ///
    /// - [`AppInner::Removing`] — still waiting (self is returned back)
    /// - [`AppInner::Ls`] — removal succeeded; contains the refreshed item list
    /// - [`AppInner::Error`] — removal failed; contains the error message
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
            Err(RecvTimeoutError::Timeout) => Ok(AppInner::Removing(self)),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "RemovingState channel cannot be disconnected without returning a result"
                )
            },
        }
    }
}
