mod ui;

use crate::cli::ls::app::ls::LsState;

pub struct HelpWindowState<Item> {
    pub ls_state: LsState<Item>,
}
