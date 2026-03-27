mod help_window;
mod preparing;
mod running_pty;
mod ui;

use std::path::Path;

use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::{
    buildkit::BuildKitD,
    cli::{
        MIN_FPS,
        run::app::{
            help_window::HelpWindowState, preparing::PreparingState, running_pty::RunningPtyState,
        },
        terminal::Terminal,
    },
    scell::types::name::TargetName,
};

pub enum App {
    Preparing(PreparingState),
    RunningPty(Box<RunningPtyState>),
    HelpWindow(HelpWindowState),
    Finished,
    Exit,
}

impl App {
    pub fn run<P>(
        buildkit: &BuildKitD,
        scell_path: P,
        entry_target: Option<TargetName>,
        detach: bool,
        quiet: bool,
        terminal: &mut Terminal,
    ) -> color_eyre::Result<()>
    where
        P: AsRef<Path> + Send + 'static,
    {
        // First step
        let mut app =
            PreparingState::new(buildkit.clone(), scell_path, entry_target, detach, quiet);

        loop {
            if let App::Preparing(ref mut state) = app
                && state.try_update()
                && let Ok(res) = state.rx.recv_timeout(MIN_FPS)
            {
                match res? {
                    Some((pty, scell)) => app = RunningPtyState::new(pty, &scell)?,
                    None => app = App::Exit,
                }
            }

            if let App::RunningPty(ref mut state)
            | App::HelpWindow(HelpWindowState(ref mut state)) = app
            {
                state.notify_screen_resize(buildkit.clone());
                if state.try_update() {
                    app = App::Finished;
                }
            }

            if matches!(app, App::Exit) {
                return Ok(());
            }

            terminal.draw(|f| {
                f.render_widget(&mut app, f.area());
            })?;

            app = app.handle_key_event()?;
        }
    }

    fn handle_key_event(mut self) -> color_eyre::Result<Self> {
        if event::poll(MIN_FPS)? {
            let event = event::read()?;
            match self {
                Self::RunningPty(state) => {
                    self = state.handle_key_event(&event)?;
                },
                Self::HelpWindow(state) => {
                    self = state.handle_key_event(&event);
                },
                Self::Preparing(state) => {
                    self = state.handle_key_event(&event);
                },
                Self::Finished => {
                    if let Event::Key(key) = event
                        && key.kind == KeyEventKind::Press
                    {
                        // Exit on any key if finished
                        self = App::Exit;
                    }
                },
                Self::Exit => {},
            }
        }
        Ok(self)
    }
}
