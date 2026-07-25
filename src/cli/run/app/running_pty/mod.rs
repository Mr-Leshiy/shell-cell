use std::path::PathBuf;

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use terminput::Encoding;
use terminput_crossterm::to_terminput;

use crate::{
    buildkit::BuildKitD,
    cli::{
        MIN_FPS,
        run::app::{App, help_window::HelpWindowState},
    },
    pty::Pty,
    scell::{SCell, name::SCellId, types::name::TargetName},
};

mod ui;

/// Input mode for the running PTY session, mirroring `tmux`'s prefix/command mode.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Keystrokes are forwarded to the shell inside the container.
    #[default]
    Normal,
    /// Entered with `Ctrl-B`; keys drive the session (scroll, detach) instead of
    /// being sent to the shell. Exited with `Esc`.
    Command,
}

pub struct RunningPtyState {
    pub pty: Pty,
    pub container_id: SCellId,
    pub target_name: TargetName,
    pub location: PathBuf,
    pub prev_height: u16,
    pub prev_width: u16,
    pub mode: InputMode,
}

impl RunningPtyState {
    pub fn run(
        pty: Pty,
        scell: &SCell,
    ) -> color_eyre::Result<App> {
        Ok(App::RunningPty(
            Self {
                pty,
                container_id: scell.container_id()?,
                target_name: scell.image().entry_point().clone(),
                location: scell.image().location().to_path_buf(),
                prev_height: 0,
                prev_width: 0,
                mode: InputMode::Normal,
            }
            .into(),
        ))
    }

    pub fn scroll_up(
        &mut self,
        lines: usize,
    ) {
        self.pty.scroll_up(lines);
    }

    pub fn scroll_down(
        &mut self,
        lines: usize,
    ) {
        self.pty.scroll_down(lines);
    }

    pub fn try_update(&mut self) {
        self.pty.process_stdout_and_stderr(MIN_FPS);
    }

    /// Notify container's session about screen resize
    pub async fn notify_screen_resize(
        &mut self,
        buildkit: &BuildKitD,
    ) -> color_eyre::Result<()> {
        // Notify container's session about screen resize
        let (curr_height, curr_width) = self.pty.size();
        if curr_height != self.prev_height || curr_width != self.prev_width {
            let session_id = self.pty.container_session_id().to_owned();
            buildkit
                .resize_shell(&session_id, curr_height, curr_width)
                .await?;
            self.prev_height = curr_height;
            self.prev_width = curr_width;
        }
        Ok(())
    }

    pub fn handle_key_event(
        self: Box<Self>,
        event: &Event,
    ) -> color_eyre::Result<App> {
        match self.mode {
            InputMode::Normal => self.handle_normal_key_event(event),
            InputMode::Command => Ok(self.handle_command_key_event(event)),
        }
    }

    /// Handles keys while forwarding input to the shell. `Ctrl-B` switches to
    /// command mode and `Ctrl-H` opens the help window; everything else is sent to
    /// the container's shell.
    fn handle_normal_key_event(
        mut self: Box<Self>,
        event: &Event,
    ) -> color_eyre::Result<App> {
        if let Event::Paste(to_paste) = event {
            self.pty.process_stdin(to_paste.as_bytes());
        } else if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.mode = InputMode::Command;
                },
                KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(App::HelpWindow(HelpWindowState(self)));
                },
                _ => {
                    let event = to_terminput(Event::Key(*key))?;
                    // Convert crossterm event to terminput and encode as stdin bytes
                    let mut buf = [0u8; 32];
                    if let Ok(written) = event.encode(&mut buf, Encoding::Xterm)
                        && let Some(bytes) = buf.get(..written)
                    {
                        self.pty.scroll_to_bottom();
                        self.pty.process_stdin(bytes);
                    }
                },
            }
        }

        Ok(App::RunningPty(self))
    }

    /// Handles keys while in the `tmux`-style command mode: `d` detaches, the arrow
    /// and `k`/`j` keys scroll, and `Esc` (or any unrecognized key) returns to normal
    /// mode without sending anything to the shell.
    fn handle_command_key_event(
        mut self: Box<Self>,
        event: &Event,
    ) -> App {
        const PAGE_SCROLL_STEP: usize = 3;
        const SCROLL_STEP: usize = 1;

        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('d') => return App::Finished,
                KeyCode::Up | KeyCode::Char('k') => self.scroll_up(SCROLL_STEP),
                KeyCode::Down | KeyCode::Char('j') => self.scroll_down(SCROLL_STEP),
                KeyCode::PageUp => self.scroll_up(PAGE_SCROLL_STEP),
                KeyCode::PageDown => self.scroll_down(PAGE_SCROLL_STEP),
                KeyCode::Esc => self.mode = InputMode::Normal,
                _ => {},
            }
        }

        App::RunningPty(self)
    }
}
