mod help_window;
mod preparing;
mod running_pty;
mod ui;

use std::{fs::OpenOptions, io::Write, path::Path};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};
use terminput::Encoding;
use terminput_crossterm::to_terminput;

use crate::{
    buildkit::BuildKitD,
    cli::{
        MIN_FPS,
        run::app::{
            help_window::HelpWindowState, preparing::PreparingState, running_pty::RunningPtyState,
        },
    },
    pty::Pty,
    scell::{SCell, types::name::TargetName},
};

pub enum App {
    Preparing(PreparingState),
    RunningPty(Box<RunningPtyState>),
    HelpWindow(HelpWindowState),
    Finished,
    Exit,
}

impl App {
    pub fn run<B, P>(
        buildkit: &BuildKitD,
        scell_path: P,
        entry_target: Option<TargetName>,
        detach: bool,
        quiet: bool,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()>
    where
        B: ratatui::backend::Backend,
        B::Error: Send + Sync + 'static,
        P: AsRef<Path> + Send + 'static,
    {
        // First step
        let mut app = Self::preparing(buildkit.clone(), scell_path, entry_target, detach, quiet);

        loop {
            if let App::Preparing(ref mut state) = app
                && state.try_update()
                && let Ok(res) = state.rx.recv_timeout(MIN_FPS)
            {
                match res? {
                    Some((pty, scell)) => app = Self::running_pty(pty, &scell)?,
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

            terminal
                .draw(|f| {
                    f.render_widget(&mut app, f.area());
                })
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;

            app = app.handle_key_event()?;
        }
    }

    fn handle_key_event(mut self) -> color_eyre::Result<Self> {
        let mut logs = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs.txt")?;
        if event::poll(MIN_FPS)? {
            let event = event::read()?;
            logs.write(format!("{event:?}\n").as_bytes())?;
            if let Event::Key(key) = event
                && key.kind == KeyEventKind::Press
            {
                match self {
                    Self::RunningPty(ref mut state)
                        if matches!(key.code, KeyCode::Up | KeyCode::Char('k'))
                            && key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        state.scroll_up();
                    },
                    Self::RunningPty(ref mut state)
                        if matches!(key.code, KeyCode::Down | KeyCode::Char('j'))
                            && key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        state.scroll_down();
                    },
                    Self::RunningPty(state)
                        if matches!(key.code, KeyCode::Char('h'))
                            && key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        self = Self::HelpWindow(HelpWindowState(state));
                    },
                    Self::RunningPty(ref state) => {
                        let event = to_terminput(Event::Key(key))?;
                        // Convert crossterm event to terminput and encode as stdin bytes
                        let mut buf = [0u8; 32];
                        if let Ok(written) = event.encode(&mut buf, Encoding::Xterm)
                            && let Some(bytes) = buf.get(..written)
                        {
                            state.pty.process_stdin(bytes);
                        }
                    },
                    Self::HelpWindow(state)
                        if (matches!(key.code, KeyCode::Char('h'))
                            && key.modifiers.contains(event::KeyModifiers::CONTROL))
                            || matches!(key.code, KeyCode::Esc) =>
                    {
                        self = Self::RunningPty(state.0);
                    },
                    Self::HelpWindow(_)
                        if matches!(key.code, KeyCode::Char('d'))
                            && key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        self = Self::Finished;
                    },
                    Self::Preparing(ref mut state) => {
                        match key.code {
                            KeyCode::Down | KeyCode::Char('j') => {
                                state.scroll_down();
                            },
                            KeyCode::Up | KeyCode::Char('k') => {
                                state.scroll_up();
                            },
                            KeyCode::Char('c' | 'd')
                                if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                            {
                                self = App::Exit;
                            },
                            _ => {},
                        }
                    },
                    Self::Finished => {
                        // Exit on any key if finished
                        self = App::Exit;
                    },
                    _ => {},
                }
            }
        }

        Ok(self)
    }

    fn preparing<P: AsRef<Path> + Send + 'static>(
        buildkit: BuildKitD,
        scell_path: P,
        entry: Option<TargetName>,
        detach: bool,
        quiet: bool,
    ) -> Self {
        App::Preparing(PreparingState::new(
            buildkit, scell_path, entry, detach, quiet,
        ))
    }

    fn running_pty(
        pty: Pty,
        scell: &SCell,
    ) -> color_eyre::Result<Self> {
        Ok(Self::RunningPty(Box::new(RunningPtyState::new(
            pty,
            scell.container_id()?,
            scell.image().entry_point().clone(),
            scell.image().location().to_path_buf(),
        ))))
    }
}
