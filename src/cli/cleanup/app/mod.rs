mod cleanning;
mod ui;

use std::sync::mpsc::Receiver;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use self::cleanning::CleanningState;
use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::{MIN_FPS, terminal::Terminal},
};

pub enum App {
    Loading {
        rx: Receiver<color_eyre::Result<(Vec<SCellContainerInfo>, Vec<SCellImageInfo>)>>,
        buildkit: BuildKitD,
    },
    CleanningContainers(CleanningState<SCellContainerInfo>),
    CleanningImages(CleanningState<SCellImageInfo>),
    Exit,
}

impl App {
    pub fn run(
        buildkit: &BuildKitD,
        all: bool,
        terminal: &mut Terminal,
    ) -> color_eyre::Result<()> {
        // First step
        let mut app = Self::loading(buildkit.clone(), all);
        let mut images_for_removal = Vec::new();
        loop {
            // Check for state transitions
            if let App::Loading {
                ref rx,
                ref buildkit,
            } = app
                && let Ok(result) = rx.recv_timeout(MIN_FPS)
            {
                let (containers_for_removal, images_for_removal_res) = result?;
                images_for_removal = images_for_removal_res;
                app = CleanningState::cleaning_containers(containers_for_removal, buildkit.clone());
            }

            if let App::CleanningContainers(ref mut state) = app
                && state.try_update()
            {
                let images_for_removal = std::mem::take(&mut images_for_removal);
                app = CleanningState::cleaning_images(images_for_removal, buildkit.clone());
            }

            if let App::CleanningImages(ref mut state) = app
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

    fn loading(
        buildkit: BuildKitD,
        all: bool,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to fetch containers for stop
        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let for_removal_fn = async || {
                    let containers = buildkit.list_containers().await?;
                    let images = buildkit.list_images().await?;
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let containers = if all {
                        containers
                    } else {
                        containers.into_iter().filter(|c| c.orphan).collect()
                    };
                    let images = if all {
                        images
                    } else {
                        images.into_iter().filter(|c| c.orphan).collect()
                    };

                    color_eyre::eyre::Ok((containers, images))
                };
                drop(tx.send(for_removal_fn().await));
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
