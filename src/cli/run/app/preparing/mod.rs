mod ui;

use std::{
    collections::VecDeque,
    path::Path,
    sync::mpsc::{Receiver, RecvTimeoutError},
    time::Duration,
};

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use tui_scrollview::ScrollViewState;

use crate::{
    buildkit::BuildKitD,
    cli::{MIN_FPS, run::app::App},
    error::UserError,
    pty::Pty,
    scell::{SCell, types::name::TargetName},
};

const LOGS_WINDOW: usize = 5000;

pub struct PreparingState {
    pub rx: Receiver<color_eyre::Result<Option<(Pty, SCell)>>>,
    pub logs_rx: Receiver<(String, LogType)>,
    pub logs: VecDeque<(String, LogType)>,
    pub scroll_view_state: ScrollViewState,
}

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Main,
    MainError,
    MainInfo,
    SubLog,
}

impl PreparingState {
    #[allow(clippy::too_many_lines)]
    pub fn prepare<P: AsRef<Path> + Send + 'static>(
        buildkit: BuildKitD,
        scell_path: P,
        entry: Option<TargetName>,
        detach: bool,
        quiet: bool,
    ) -> App {
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
                        tokio::time::sleep(Duration::from_secs(2)).await;
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
                if buildkit
                    .build_image(scell.image(), |msg| {
                        if !quiet {
                            drop(logs_tx.send((msg, LogType::SubLog)));
                        }
                    })
                    .await?
                {
                    drop(logs_tx.send((
                        "⚡ 'Shell-Cell' image already exists, skipping build".to_string(),
                        LogType::MainInfo,
                    )));
                }

                for (s_name, s) in scell.services() {
                    drop(logs_tx.send((
                        format!("⚙️ Building 'Shell-Cell' service '{s_name}' image"),
                        LogType::Main,
                    )));
                    if buildkit
                        .build_image(&s.image, |msg| {
                            if !quiet {
                                drop(logs_tx.send((msg, LogType::SubLog)));
                            }
                        })
                        .await?
                    {
                        drop(logs_tx.send((
                            format!("⚡ 'Shell-Cell' service '{s_name}' image already exists, skipping build"),
                            LogType::MainInfo
                        )));
                    }
                    drop(logs_tx.send((
                        format!("📦 Starting 'Shell-Cell' service '{s_name}' container"),
                        LogType::Main,
                    )));
                    buildkit
                        .start_service_container(&scell, s_name, &s.image, &s.container)
                        .await?;
                }

                drop(logs_tx.send((
                    "📦 Starting 'Shell-Cell' container".to_string(),
                    LogType::Main,
                )));
                buildkit.start_container(&scell).await?;

                if detach {
                    return color_eyre::eyre::Ok(None);
                }

                let pty = buildkit.attach_to_shell(&scell).await?;

                drop(logs_tx.send((
                    "🚀 Starting 'Shell-Cell' session".to_string(),
                    LogType::Main,
                )));
                color_eyre::eyre::Ok(Some((pty, scell)))
            };

            match preparing().await {
                Ok(res) => drop(tx.send(Ok(res))),
                Err(e) if e.is::<UserError>() => {
                    drop(logs_tx.send((format!("{e}"), LogType::MainError)));
                },
                Err(e) => drop(tx.send(Err(e))),
            }
        });
        App::Preparing(Self {
            rx,
            logs_rx,
            logs: VecDeque::new(),
            scroll_view_state: ScrollViewState::new(),
        })
    }

    pub fn try_update(&mut self) -> bool {
        match self.logs_rx.recv_timeout(MIN_FPS) {
            Ok(log) => {
                if self.logs.len() == LOGS_WINDOW {
                    self.logs.pop_front();
                }
                self.logs.push_back(log);
                self.scroll_view_state.scroll_to_bottom();
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_view_state.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.scroll_view_state.scroll_down();
    }

    pub fn handle_key_event(
        mut self,
        event: &Event,
    ) -> App {
        if let Event::Key(key) = event
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Down | KeyCode::Char('j') => {
                    self.scroll_down();
                },
                KeyCode::Up | KeyCode::Char('k') => {
                    self.scroll_up();
                },
                KeyCode::Char('c' | 'd') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return App::Exit;
                },
                _ => {},
            }
        }
        App::Preparing(self)
    }
}
