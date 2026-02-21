mod ui;

use std::{
    collections::HashMap,
    hash::Hash,
    sync::mpsc::{Receiver, RecvTimeoutError},
};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::MIN_FPS,
};

pub enum App {
    Loading {
        rx: Receiver<color_eyre::Result<(Vec<SCellContainerInfo>, Vec<SCellImageInfo>)>>,
        buildkit: BuildKitD,
    },
    CleaningContainers(CleaningState<SCellContainerInfo>),
    CleaningImages(CleaningState<SCellImageInfo>),
    Exit,
}

impl App {
    pub fn run<B: ratatui::backend::Backend>(
        buildkit: &BuildKitD,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()> {
        // First step
        let mut app = Self::loading(buildkit.clone());
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
                app = Self::cleaning_containers(containers_for_removal, buildkit.clone());
            }

            if let App::CleaningContainers(ref mut state) = app
                && state.try_update()
            {
                let images_for_removal = std::mem::take(&mut images_for_removal);
                app = Self::cleaning_images(images_for_removal, buildkit.clone());
            }

            if let App::CleaningImages(ref mut state) = app
                && state.try_update()
            {
                app = App::Exit;
            }

            if matches!(app, App::Exit) {
                return Ok(());
            }

            terminal
                .draw(|f| {
                    f.render_widget(&app, f.area());
                })
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;

            app = app.handle_key_event()?;
        }
    }

    fn loading(buildkit: BuildKitD) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to fetch containers for stop
        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let for_removal_fn = async || {
                    let containers = buildkit.list_containers().await?;
                    let images = buildkit.list_images().await?;
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    // consider only orphan Shell-Cell containers and images
                    let containers = containers
                        .into_iter()
                        .filter(|c| c.orphan)
                        .collect::<Vec<_>>();
                    let images = images
                        .into_iter()
                        .filter(|c| c.orphan && !c.in_use)
                        .collect::<Vec<_>>();

                    color_eyre::eyre::Ok((containers, images))
                };
                drop(tx.send(for_removal_fn().await));
            }
        });

        App::Loading { rx, buildkit }
    }

    fn cleaning_containers(
        for_removal: Vec<SCellContainerInfo>,
        buildkit: BuildKitD,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to cleanup orphan containers and their corresponding images
        tokio::spawn({
            let containers = for_removal.clone();
            async move {
                for c in containers {
                    let res = buildkit.cleanup_container(&c).await;
                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                    drop(tx.send((c, res)));
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        App::CleaningContainers(CleaningState::new(for_removal, rx))
    }

    fn cleaning_images(
        for_removal: Vec<SCellImageInfo>,
        buildkit: BuildKitD,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn async task to cleanup orphan images
        tokio::spawn({
            let images = for_removal.clone();
            async move {
                for c in images {
                    let res = buildkit.cleanup_image(&c).await;
                    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                    drop(tx.send((c, res)));
                }
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });

        App::CleaningImages(CleaningState::new(for_removal, rx))
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

pub struct CleaningState<Item> {
    removing_results: HashMap<Item, Option<color_eyre::Result<()>>>,
    rx: Receiver<(Item, color_eyre::Result<()>)>,
}

impl<Item: Clone + PartialEq + Eq + Hash> CleaningState<Item> {
    pub fn new(
        for_removal: Vec<Item>,
        rx: Receiver<(Item, color_eyre::Result<()>)>,
    ) -> Self {
        Self {
            removing_results: for_removal.into_iter().map(|c| (c, None)).collect(),
            rx,
        }
    }

    /// Returns boolean flag, if the udelrying channel was closed or not
    fn try_update(&mut self) -> bool {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(update) => {
                self.removing_results.insert(update.0, Some(update.1));
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        }
    }
}
