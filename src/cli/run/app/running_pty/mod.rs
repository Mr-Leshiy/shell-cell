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

pub struct RunningPtyState {
    pub pty: Pty,
    pub container_id: SCellId,
    pub target_name: TargetName,
    pub location: PathBuf,
    pub prev_height: u16,
    pub prev_width: u16,
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
            }
            .into(),
        ))
    }

    pub fn scroll_up(&mut self) {
        self.pty.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.pty.scroll_down();
    }

    pub fn try_update(&mut self) -> bool {
        self.pty.process_stdout_and_stderr(MIN_FPS)
    }

    pub fn notify_screen_resize(
        &mut self,
        buildkit: BuildKitD,
    ) {
        // Notify container's session about screen resize
        let (curr_height, curr_width) = self.pty.size();
        if curr_height != self.prev_height || curr_width != self.prev_width {
            tokio::spawn({
                let session_id = self.pty.container_session_id().to_owned();
                async move {
                    buildkit
                        .resize_shell(&session_id, curr_height, curr_width)
                        .await?;
                    color_eyre::eyre::Ok(())
                }
            });

            self.prev_height = curr_height;
            self.prev_width = curr_width;
        }
    }

    pub fn handle_key_event(
        mut self: Box<Self>,
        event: &Event,
    ) -> color_eyre::Result<App> {
        if let Event::Paste(to_paste) = event {
            self.pty.process_stdin(to_paste.as_bytes());
        } else if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Up | KeyCode::Char('k')
                    if key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    self.scroll_up();
                },
                KeyCode::Down | KeyCode::Char('j')
                    if key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    self.scroll_down();
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
}
