mod ui;

use std::{
    path::Path,
    sync::mpsc::{Receiver, RecvTimeoutError},
    time::Duration,
};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};
use terminput::Encoding;
use terminput_crossterm::to_terminput;

use crate::{
    buildkit::BuildKitD,
    cli::MIN_FPS,
    error::UserError,
    pty::Pty,
    scell::{SCell, types::name::TargetName},
};

pub enum App {
    Preparing(PreparingState),
    RunningPty(Box<RunningPtyState>),
    Finished,
    Exit,
}

impl App {
    pub fn run<B, P>(
        buildkit: &BuildKitD,
        scell_path: P,
        entry_target: Option<TargetName>,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()>
    where
        B: ratatui::backend::Backend,
        B::Error: Send + Sync + 'static,
        P: AsRef<Path> + Send + 'static,
    {
        // First step
        let mut app = Self::preparing(buildkit.clone(), scell_path, entry_target);

        loop {
            if let App::Preparing(ref mut state) = app
                && state.try_update()
                && let Ok(res) = state.rx.recv_timeout(MIN_FPS)
            {
                let (pty, scell) = res?;
                app = Self::running_pty(pty, &scell)?;
            }

            if let App::RunningPty(ref mut state) = app {
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
            // For `RunningPty` - forward all key events to PTY stdin
            if let Self::RunningPty(ref state) = self
                && let Ok(event) = to_terminput(Event::Key(key))
            {
                // Convert crossterm event to terminput and encode as stdin bytes
                let mut buf = [0u8; 32];
                if let Ok(written) = event.encode(&mut buf, Encoding::Xterm)
                    && let Some(bytes) = buf.get(..written)
                {
                    state.pty.process_stdin(bytes);
                }
                // Handles every other app state
            } else if matches!(self, App::Finished) {
                // Exit on any key if finished
                self = App::Exit;
            } else if let KeyCode::Char('c' | 'd') = key.code
                && key.modifiers.contains(event::KeyModifiers::CONTROL)
            {
                // Exit on Ctrl-C or Ctrl-D for other states
                self = App::Exit;
            }
        }

        Ok(self)
    }

    fn preparing<P: AsRef<Path> + Send + 'static>(
        buildkit: BuildKitD,
        scell_path: P,
        entry: Option<TargetName>,
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
        App::Preparing(PreparingState {
            rx,
            logs_rx,
            logs: Vec::new(),
        })
    }

    fn running_pty(
        pty: Pty,
        scell: &SCell,
    ) -> color_eyre::Result<Self> {
        Ok(Self::RunningPty(Box::new(RunningPtyState {
            pty,
            scell_name: scell.name()?.to_string(),
            prev_height: 0,
            prev_width: 0,
        })))
    }
}

pub struct PreparingState {
    rx: Receiver<color_eyre::Result<(Pty, SCell)>>,
    logs_rx: Receiver<(String, LogType)>,
    logs: Vec<(String, LogType)>,
}

#[derive(Debug, Clone, Copy)]
enum LogType {
    Main,
    MainError,
    MainInfo,
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
    pty: Pty,
    scell_name: String,
    prev_height: u16,
    prev_width: u16,
}

impl RunningPtyState {
    fn try_update(&mut self) -> bool {
        self.pty.process_stdout_and_stderr(MIN_FPS)
    }

    fn notify_screen_resize(
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
}
