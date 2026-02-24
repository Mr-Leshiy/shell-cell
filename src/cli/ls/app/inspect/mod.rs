mod ui;

use tui_scrollview::ScrollViewState;

use crate::cli::ls::app::ls::LsState;

/// Holds the state when the user is viewing the inspect overlay.
pub struct InspectState<Item> {
    /// The list state to restore when the overlay is dismissed.
    pub ls_state: LsState<Item>,
    /// The parsed YAML definition from the `scell-description` label, if present.
    description: Option<String>,
    scroll_state: ScrollViewState,
}

impl<Item> InspectState<Item> {
    pub fn new(
        ls_state: LsState<Item>,
        description: Option<String>,
    ) -> Self {
        Self {
            ls_state,
            description,
            scroll_state: ScrollViewState::new(),
        }
    }

    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll_state.scroll_up();
    }

    /// Scroll down
    pub fn scroll_down(&mut self) {
        self.scroll_state.scroll_down();
    }
}
