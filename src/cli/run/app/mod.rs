mod help_window;
mod preparing;
mod running_pty;
mod ui;

use std::{path::Path, sync::mpsc::RecvTimeoutError};

use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::{
    buildkit::BuildKitD,
    cli::{
        MIN_FPS,
        run::app::{
            help_window::HelpWindowState,
            preparing::{LogType, PreparingState},
            running_pty::RunningPtyState,
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
    pub async fn run<P>(
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
            PreparingState::prepare(buildkit.clone(), scell_path, entry_target, detach, quiet);

        loop {
            if let App::Preparing(state) = app {
                app = state.try_update()?;
            }

            if let App::RunningPty(ref mut state)
            | App::HelpWindow(HelpWindowState(ref mut state)) = app
            {
                state.notify_screen_resize(buildkit).await?;
                state.try_update();
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

    /// Headless variant for `--detach`. No TTY/Terminal required:
    /// runs the same preparation pipeline as the TUI but streams logs to
    /// stderr and exits when the container is started.
    #[allow(clippy::unused_async)] // kept async to match callsite/expectations
    pub async fn run_headless<P>(
        buildkit: &BuildKitD,
        scell_path: P,
        entry_target: Option<TargetName>,
        quiet: bool,
    ) -> color_eyre::Result<()>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let app = PreparingState::prepare(buildkit.clone(), scell_path, entry_target, true, quiet);
        let App::Preparing(state) = app else {
            // PreparingState::prepare always returns App::Preparing
            return Ok(());
        };

        loop {
            match state.logs_rx.recv_timeout(MIN_FPS) {
                Ok((msg, log_type)) => match log_type {
                    LogType::MainError => eprintln!("error: {msg}"),
                    _ => eprintln!("{msg}"),
                },
                Err(RecvTimeoutError::Timeout) => {},
                Err(RecvTimeoutError::Disconnected) => break,
            }

            match state.rx.try_recv() {
                Ok(res) => {
                    res?;
                    return Ok(());
                },
                Err(std::sync::mpsc::TryRecvError::Empty) => {},
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            }
        }
        Ok(())
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
