mod ui;

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::cli::run::app::{App, running_pty::RunningPtyState};

pub struct HelpWindowState(pub Box<RunningPtyState>);

impl HelpWindowState {
    pub fn handle_key_event(
        self,
        event: &Event,
    ) -> App {
        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return App::RunningPty(self.0);
                },
                KeyCode::Esc => return App::RunningPty(self.0),
                _ => {},
            }
        }
        App::HelpWindow(self)
    }
}
