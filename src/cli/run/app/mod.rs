mod help_window;
mod preparing;
mod running_pty;
mod ui;

use std::{path::Path, time::Duration};

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
            help_window::HelpWindowState,
            preparing::{LogType, PreparingState},
            running_pty::RunningPtyState,
        },
    },
    error::UserError,
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
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()>
    where
        B: ratatui::backend::Backend,
        B::Error: Send + Sync + 'static,
        P: AsRef<Path> + Send + 'static,
    {
        // First step
        let mut app = Self::preparing(buildkit.clone(), scell_path, entry_target, detach);

        loop {
            if let App::Preparing(ref mut state) = app
                && state.try_update()
                && let Ok(res) = state.rx.recv_timeout(MIN_FPS)
            {
                let (pty, scell) = res?;
                app = Self::running_pty(pty, &scell)?;
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
        if event::poll(MIN_FPS)?
            && let Event::Key(key) = event::read()?
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

        Ok(self)
    }

    fn preparing<P: AsRef<Path> + Send + 'static>(
        buildkit: BuildKitD,
        scell_path: P,
        entry: Option<TargetName>,
        _detach: bool,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (logs_tx, logs_rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let preparing = async || {
                drop(logs_tx.send((
                    "🧐 Checking for newer 'Shell-Cell' version".to_string(),
                    LogType::Main,
                )));

                match crate::version_check::check_for_newer_version().await {
                    Ok(Some(newer_version)) => {
                        drop(logs_tx.send((
                            format!(
                                "🆕 A newer version '{newer_version}' of 'Shell-Cell' is available"
                            ),
                            LogType::MainInfo,
                        )));
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    },
                    Ok(None) => {
                        drop(logs_tx.send((
                            "🎉 'Shell-Cell' is up to date".to_string(),
                            LogType::MainInfo,
                        )));
                    },
                    Err(_) => {
                        drop(
                            logs_tx
                                .send(("Cannot check for updates".to_string(), LogType::MainError)),
                        );
                    },
                }

                drop(logs_tx.send((
                    "📝 Compiling Shell-Cell blueprint".to_string(),
                    LogType::Main,
                )));
                let scell = SCell::compile(scell_path, entry)?;

                if buildkit.image_exists(&scell).await? {
                    drop(logs_tx.send((
                        "⚡ 'Shell-Cell' image already exists, skipping build".to_string(),
                        LogType::MainInfo,
                    )));
                } else {
                    drop(
                        logs_tx.send(("⚙️ Building 'Shell-Cell' image".to_string(), LogType::Main)),
                    );
                    buildkit
                        .build_image(&scell, |msg| {
                            drop(logs_tx.send((msg, LogType::SubLog)));
                        })
                        .await?;
                }

                drop(logs_tx.send((
                    "📦 Starting 'Shell-Cell' container".to_string(),
                    LogType::Main,
                )));
                buildkit.start_container(&scell).await?;
                let pty = buildkit.attach_to_shell(&scell).await?;

                drop(logs_tx.send((
                    "🚀 Starting 'Shell-Cell' session".to_string(),
                    LogType::Main,
                )));
                color_eyre::eyre::Ok((pty, scell))
            };

            match preparing().await {
                Ok(res) => drop(tx.send(Ok(res))),
                Err(e) if e.is::<UserError>() => {
                    drop(logs_tx.send((format!("{e}"), LogType::MainError)));
                },
                Err(e) => drop(tx.send(Err(e))),
            }
        });
        App::Preparing(PreparingState::new(rx, logs_rx))
    }

    fn running_pty(
        pty: Pty,
        scell: &SCell,
    ) -> color_eyre::Result<Self> {
        Ok(Self::RunningPty(Box::new(RunningPtyState::new(
            pty,
            scell.container_id()?,
        ))))
    }
}
