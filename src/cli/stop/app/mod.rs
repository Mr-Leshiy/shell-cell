mod loading;
mod stopping;
mod ui;

use loading::LoadingState;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use stopping::StoppingState;

use crate::{
    buildkit::BuildKitD,
    cli::{MIN_FPS, terminal::Terminal},
};

pub enum App {
    Loading(LoadingState),
    Stopping(StoppingState),
    Exit,
}

impl App {
    pub fn run(
        buildkit: &BuildKitD,
        terminal: &mut Terminal,
    ) -> color_eyre::Result<()> {
        // First step

        let mut app = LoadingState::load(buildkit.clone());
        loop {
            // Check for state transitions
            if let App::Loading(state) = app {
                app = state.try_recv(Some(MIN_FPS))?;
            }

            if let App::Stopping(ref mut state) = app
                && state.try_update(Some(MIN_FPS))
            {
                app = App::Exit;
            }

            if matches!(app, App::Exit) {
                return Ok(());
            }

            terminal.draw(|f| {
                f.render_widget(&app, f.area());
            })?;

            app = app.handle_key_event()?;
        }
    }

    fn handle_key_event(mut self) -> color_eyre::Result<Self> {
        if event::poll(MIN_FPS)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let KeyCode::Char('c' | 'd') = key.code
            && key.modifiers.contains(event::KeyModifiers::CONTROL)
        {
            self = App::Exit;
        }

        Ok(self)
    }
}
