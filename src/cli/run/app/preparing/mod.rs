mod ui;

use std::{
    collections::VecDeque,
    sync::mpsc::{Receiver, RecvTimeoutError},
};

use tui_scrollview::ScrollViewState;

use crate::{cli::MIN_FPS, pty::Pty, scell::SCell};

const LOGS_WINDOW: usize = 5000;

pub struct PreparingState {
    pub rx: Receiver<color_eyre::Result<Option<(Pty, SCell)>>>,
    pub logs_rx: Receiver<(String, LogType)>,
    pub logs: VecDeque<(String, LogType)>,
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
    pub fn new(
        rx: Receiver<color_eyre::Result<Option<(Pty, SCell)>>>,
        logs_rx: Receiver<(String, LogType)>,
    ) -> Self {
        Self {
            rx,
            logs_rx,
            logs: VecDeque::new(),
            scroll_view_state: ScrollViewState::new(),
        }
    }

    pub fn try_update(&mut self) -> bool {
        match self.logs_rx.recv_timeout(MIN_FPS) {
            Ok(log) => {
                if self.logs.len() == LOGS_WINDOW {
                    self.logs.pop_front();
                }
                self.logs.push_back(log);
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
