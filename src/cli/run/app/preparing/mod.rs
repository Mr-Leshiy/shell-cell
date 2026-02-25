mod ui;

use std::sync::mpsc::{Receiver, RecvTimeoutError};

use tui_scrollview::ScrollViewState;

use crate::{cli::MIN_FPS, pty::Pty, scell::SCell};

pub struct PreparingState {
    pub rx: Receiver<color_eyre::Result<(Pty, SCell)>>,
    pub logs_rx: Receiver<(String, LogType)>,
    pub logs: Vec<(String, LogType)>,
    pub scroll_view_state: ScrollViewState,
}

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Main,
    MainError,
    MainInfo,
    SubLog,
}

impl PreparingState {
    pub fn try_update(&mut self) -> bool {
        match self.logs_rx.recv_timeout(MIN_FPS) {
            Ok(log) => {
                self.logs.push(log);
                self.scroll_view_state.scroll_to_bottom();
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_view_state.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_view_state.scroll_down();
    }
}
