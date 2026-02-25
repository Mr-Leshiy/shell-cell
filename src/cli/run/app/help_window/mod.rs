mod ui;

use crate::cli::run::app::running_pty::RunningPtyState;

pub struct HelpWindowState {
    pub running_pty_state: Box<RunningPtyState>,
}
