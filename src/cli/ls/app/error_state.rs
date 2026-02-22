use crate::cli::ls::app::ls::LsState;

/// Holds the state when a background operation (stop/remove) has failed.
pub struct ErrorState<Item> {
    /// The list state to restore when the error is dismissed.
    pub ls_state: LsState<Item>,
    /// The error message to display.
    pub message: String,
}
