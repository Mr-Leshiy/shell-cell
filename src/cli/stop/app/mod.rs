mod ui;

use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, RecvTimeoutError},
};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use crate::{buildkit::BuildKitD, cli::MIN_FPS, scell::container_info::SCellContainerInfo};

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
            if let App::Loading {
                ref rx,
                ref buildkit,
            } = self
                && let Ok(result) = rx.recv_timeout(MIN_FPS)
            {
                let containers = result?;
                self = Self::stopping(containers, buildkit.clone());
            }

            if let App::Stopping(ref mut state) = self
                && state.try_update()
            {
                self = App::Exit;
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
                    let res = buildkit.stop_container_by_name(&c.name.to_string()).await;
                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                    drop(tx.send((c, res)));
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        App::Stopping(StoppingState::new(containers, rx))
    }

    fn handle_key_event(&mut self) -> color_eyre::Result<()> {
        if event::poll(MIN_FPS)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let KeyCode::Char('c' | 'd') = key.code
            && key.modifiers.contains(event::KeyModifiers::CONTROL)
        {
            *self = App::Exit;
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

    /// Returns boolean flag, if the udelrying channel was closed or not
    fn try_update(&mut self) -> bool {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(update) => {
                self.containers.insert(update.0, Some(update.1));
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        }
    }
}
