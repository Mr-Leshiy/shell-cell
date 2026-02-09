mod ui;

use std::{
    path::Path,
    sync::mpsc::{Receiver, RecvTimeoutError},
};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use crate::{buildkit::BuildKitD, cli::UPDATE_TIMEOUT, scell::SCell};

pub enum App {
    Preparing(PreparingState),
    Finished,
    Exit,
}

impl App {
    pub fn run<B: ratatui::backend::Backend, P: AsRef<Path> + Send + 'static>(
        buildkit: BuildKitD,
        scell_path: P,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()> {
        // First step
        let mut app = Self::preparing(buildkit, scell_path);

        loop {
            if let App::Preparing(ref mut state) = app
                && state.try_update()
                && let Ok(res) = state.rx.recv_timeout(UPDATE_TIMEOUT)
            {
                res?;
            }

            if matches!(app, App::Exit) {
                return Ok(());
            }

            terminal
                .draw(|f| {
                    f.render_widget(&app, f.area());
                })
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;

            app.handle_key_event()?;
        }
    }

    fn handle_key_event(&mut self) -> color_eyre::Result<()> {
        if event::poll(UPDATE_TIMEOUT)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let KeyCode::Char('c' | 'd') = key.code
            && key.modifiers.contains(event::KeyModifiers::CONTROL)
        {
            *self = App::Exit;
        }

        Ok(())
    }

    fn preparing<P: AsRef<Path> + Send + 'static>(
        buildkit: BuildKitD,
        scell_path: P,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (logs_tx, logs_rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let preparing = async || {
                drop(logs_tx.send((
                    "📝 Compiling Shell-Cell blueprint file".to_string(),
                    LogType::Main,
                )));
                let scell = SCell::compile(scell_path, None)?;

                drop(logs_tx.send(("⚙️ Building 'Shell-Cell' image".to_string(), LogType::Main)));
                buildkit
                    .build_image(&scell, |msg| {
                        drop(logs_tx.send((msg, LogType::SubLog)));
                    })
                    .await?;

                drop(logs_tx.send((
                    "📦 Starting 'Shell-Cell' container".to_string(),
                    LogType::Main,
                )));
                buildkit.start_container(&scell).await?;
                let _pty = buildkit.attach_to_shell(&scell).await?;

                drop(logs_tx.send((
                    "🚀 Starting 'Shell-Cell' session".to_string(),
                    LogType::Main,
                )));

                Ok(())
            };

            let res = preparing().await;

            drop(tx.send(res));
        });
        App::Preparing(PreparingState {
            rx,
            logs_rx,
            logs: Vec::new(),
        })
    }
}

pub struct PreparingState {
    rx: Receiver<color_eyre::Result<()>>,
    logs_rx: Receiver<(String, LogType)>,
    logs: Vec<(String, LogType)>,
}

enum LogType {
    Main,
    SubLog,
}

impl PreparingState {
    fn try_update(&mut self) -> bool {
        match self.logs_rx.recv_timeout(UPDATE_TIMEOUT) {
            Ok(log) => {
                self.logs.push(log);
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        }
    }
}
