mod ui;

use tui_scrollview::ScrollViewState;

use crate::cli::ls::app::ls::LsState;

/// Holds the state when the user is viewing the inspect overlay.
pub struct InspectState<Item> {
    /// The list state to restore when the overlay is dismissed.
    pub ls_state: LsState<Item>,
    /// The parsed YAML definition from the `scell-definition` label, if present.
    pub definition: Option<String>,
    pub scroll_state: ScrollViewState,
}
