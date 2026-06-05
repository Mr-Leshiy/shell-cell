// mod loading;
mod stopping;
mod ui;

use std::sync::mpsc::Receiver;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
pub use stopping::StoppingState;

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo},
    cli::{MIN_FPS, terminal::Terminal},
};

pub enum App {
    Loading {
        rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
        buildkit: BuildKitD,
    },
    Stopping(stopping::StoppingState),
    Exit,
}

impl App {
    pub fn run(
        buildkit: &BuildKitD,
        terminal: &mut Terminal,
    ) -> color_eyre::Result<()> {
        // First step
        let mut app = Self::loading(buildkit.clone());
        loop {
            // Check for state transitions
            if let App::Loading {
                ref rx,
                ref buildkit,
            } = app
                && let Ok(result) = rx.recv_timeout(MIN_FPS)
            {
                let containers = result?;
                app = StoppingState::stop(containers, buildkit.clone());
            }

            if let App::Stopping(ref mut state) = app
                && state.try_update()
            {
                app = App::Exit;
            }

            if matches!(app, App::Exit) {
                return Ok(());
            }

            terminal.draw(|f| {
                f.render_widget(&app, f.area());
            })?;

            app = app.handle_key_event()?;
        }
    }

    fn loading(buildkit: BuildKitD) -> Self {
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

    fn handle_key_event(mut self) -> color_eyre::Result<Self> {
        if event::poll(MIN_FPS)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let KeyCode::Char('c' | 'd') = key.code
            && key.modifiers.contains(event::KeyModifiers::CONTROL)
        {
            self = App::Exit;
        }

        Ok(self)
    }
}
