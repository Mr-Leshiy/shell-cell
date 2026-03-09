mod ui;

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::{ls::LsState, removing::RemovingState},
};

/// Holds the state while waiting for user confirmation to remove an item.
///
/// Displays a warning that all item's state will be lost and waits for
/// the user to press 'y' (confirm) or 'n'/'Esc' (cancel).
pub struct ConfirmRemoveState<Item> {
    pub selected_to_remove: Item,
    pub ls_state: LsState<Item>,
}

impl<Item> ConfirmRemoveState<Item> {
    /// User cancelled removal - return to the list view.
    pub fn cancel(self) -> LsState<Item> {
        self.ls_state
    }
}

impl ConfirmRemoveState<SCellContainerInfo> {
    /// User confirmed removal - initiate the removal process.
    pub fn confirm(self) -> RemovingState<SCellContainerInfo> {
        RemovingState::<SCellContainerInfo>::new(self.ls_state, self.selected_to_remove)
    }
}

impl ConfirmRemoveState<SCellImageInfo> {
    /// User confirmed removal - initiate the removal process.
    pub fn confirm(self) -> RemovingState<SCellImageInfo> {
        RemovingState::<SCellImageInfo>::new(self.ls_state, self.selected_to_remove)
    }
}
