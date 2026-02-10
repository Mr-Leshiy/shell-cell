mod ui;

use std::{
    path::Path,
    sync::mpsc::{Receiver, RecvTimeoutError},
};

use bytes::Bytes;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};
use terminput::Encoding;
use terminput_crossterm::to_terminput;
use tui_term::vt100::Parser;

use crate::{buildkit::BuildKitD, cli::MIN_FPS, pty::PtyStdStreams, scell::SCell};

pub enum App {
    Preparing(PreparingState),
    RunningPty(Box<RunningPtyState>),
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
                && let Ok(res) = state.rx.recv_timeout(MIN_FPS)
            {
                let (pty, scell) = res?;
                app = Self::running_pty(pty, &scell);
            }

            if let App::RunningPty(ref mut state) = app
                && state.try_update()
            {
                // Drain any buffered terminal events before transitioning
                app = App::Finished;
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
        if event::poll(MIN_FPS)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            // For `RunningPty` - forward all key events to PTY stdin
            if let Self::RunningPty(state) = self
                && let Ok(event) = to_terminput(Event::Key(key))
            {
                // Convert crossterm event to terminput and encode as stdin bytes
                let mut buf = [0u8; 32];
                if let Ok(written) = event.encode(&mut buf, Encoding::Xterm)
                    && let Some(bytes) = buf.get(..written)
                {
                    state
                        .pty_streams
                        .stdin
                        .send(Bytes::copy_from_slice(bytes))
                        .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;
                }
                // Handles every other app state
            } else if matches!(self, App::Finished) {
                // Exit on any key if finished
                *self = App::Exit;
            } else if let KeyCode::Char('c' | 'd') = key.code
                && key.modifiers.contains(event::KeyModifiers::CONTROL)
            {
                // Exit on Ctrl-C or Ctrl-D for other states
                *self = App::Exit;
            }
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
                let pty = buildkit.attach_to_shell(&scell).await?;

                drop(logs_tx.send((
                    "🚀 Starting 'Shell-Cell' session".to_string(),
                    LogType::Main,
                )));
                Ok((pty, scell))
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

    fn running_pty(
        pty_streams: PtyStdStreams,
        scell: &SCell,
    ) -> Self {
        Self::RunningPty(Box::new(RunningPtyState {
            pty_streams,
            scell_name: scell.name(),
            parser: Parser::default(),
        }))
    }
}

pub struct PreparingState {
    rx: Receiver<color_eyre::Result<(PtyStdStreams, SCell)>>,
    logs_rx: Receiver<(String, LogType)>,
    logs: Vec<(String, LogType)>,
}

enum LogType {
    Main,
    SubLog,
}

impl PreparingState {
    fn try_update(&mut self) -> bool {
        match self.logs_rx.recv_timeout(MIN_FPS) {
            Ok(log) => {
                self.logs.push(log);
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        }
    }
}

pub struct RunningPtyState {
    pty_streams: PtyStdStreams,
    scell_name: String,
    parser: Parser,
}

impl RunningPtyState {
    pub fn try_update(&mut self) -> bool {
        let stdout_res = match self.pty_streams.stdout.recv_timeout(MIN_FPS) {
            Ok(bytes) => {
                self.parser.process(&bytes);
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        };

        let stderr_res = match self.pty_streams.stderr.recv_timeout(MIN_FPS) {
            Ok(bytes) => {
                self.parser.process(&bytes);
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        };

        stdout_res && stderr_res
    }
}
