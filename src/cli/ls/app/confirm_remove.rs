use crate::{
    buildkit::container_info::SCellContainerInfo,
    cli::ls::app::{ls::LsState, removing::RemovingState},
};

/// Holds the state while waiting for user confirmation to remove an item.
///
/// Displays a warning that all item's state will be lost and waits for
/// the user to press 'y' (confirm) or 'n'/'Esc' (cancel).
pub struct ConfirmRemoveState {
    pub selected_to_remove: SCellContainerInfo,
    pub ls_state: LsState<SCellContainerInfo>,
}

impl ConfirmRemoveState {
    /// User confirmed removal - initiate the removal process.
    pub fn confirm(self) -> RemovingState {
        let (tx, rx) = std::sync::mpsc::channel();
        let buildkit = self.ls_state.buildkit.clone();
        tokio::spawn({
            let for_removal = self.selected_to_remove.clone();
            async move {
                let res = buildkit.cleanup_container(&for_removal).await;
                let res = match res {
                    Ok(()) => buildkit.list_containers().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(res));
            }
        });

        RemovingState {
            for_removal: self.selected_to_remove,
            ls_state: self.ls_state,
            rx,
        }
    }

    /// User cancelled removal - return to the list view.
    pub fn cancel(self) -> LsState<SCellContainerInfo> {
        self.ls_state
    }
}
