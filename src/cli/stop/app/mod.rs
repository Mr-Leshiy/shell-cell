mod ui;

use std::{collections::HashMap, sync::mpsc::Receiver};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use crate::{buildkit::BuildKitD, cli::UPDATE_TIMEOUT, scell::container_info::SCellContainerInfo};

pub enum App {
    Loading {
        rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
        buildkit: BuildKitD,
    },
    Stopping(StoppingState),
    Exit,
}

impl App {
    pub fn loading(buildkit: BuildKitD) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to fetch containers for stop
        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let result = async {
                    let res = buildkit.list_containers().await;
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    res
                }
                .await;
                drop(tx.send(result));
            }
        });

        App::Loading { rx, buildkit }
    }

    pub fn run<B: ratatui::backend::Backend>(
        mut self,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()> {
        loop {
            // Check for state transitions
            if let App::Loading { rx, buildkit } = self {
                if let Ok(result) = rx.recv_timeout(UPDATE_TIMEOUT) {
                    let containers = result?;
                    self = Self::stopping(containers, buildkit);
                }
            }

            if matches!(self, App::Exit) {
                return Ok(());
            }

            terminal
                .draw(|f| {
                    f.render_widget(&self, f.area());
                })
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;

            self.handle_key_event()?;
        }
    }

    fn stopping(
        containers: Vec<SCellContainerInfo>,
        buildkit: BuildKitD,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to stop containers
        tokio::spawn({
            let containers = containers.clone();
            async move {
                for c in containers {
                    let res = buildkit.stop_container_by_name(&c.container_name).await;
                    drop(tx.send((c, res)));
                }
            }
        });

        App::Stopping(StoppingState::new(containers, rx))
    }

    fn handle_key_event(&mut self) -> color_eyre::Result<()> {
        if event::poll(UPDATE_TIMEOUT)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            *self = App::Exit;
                        },
                        _ => {},
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct StoppingState {
    containers: HashMap<SCellContainerInfo, Option<color_eyre::Result<()>>>,
    rx: Receiver<(SCellContainerInfo, color_eyre::Result<()>)>,
}

impl StoppingState {
    pub fn new(
        containers: Vec<SCellContainerInfo>,
        rx: Receiver<(SCellContainerInfo, color_eyre::Result<()>)>,
    ) -> Self {
        Self {
            containers: containers.into_iter().map(|c| (c, None)).collect(),
            rx,
        }
    }

    fn try_update(&mut self) -> color_eyre::Result<()> {
        if let Ok(update) = self.rx.recv_timeout(UPDATE_TIMEOUT) {
            self.containers.insert(update.0, Some(update.1));
        }
        Ok(())
    }
}
