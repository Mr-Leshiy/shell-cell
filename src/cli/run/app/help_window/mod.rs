mod ui;

use crate::cli::run::app::running_pty::RunningPtyState;

pub struct HelpWindowState(pub Box<RunningPtyState>);
