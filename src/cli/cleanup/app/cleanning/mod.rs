mod ui;

use std::{
    collections::HashMap,
    hash::Hash,
    sync::mpsc::{Receiver, RecvTimeoutError},
};

use super::App;
use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::MIN_FPS,
};

pub struct CleanningState<Item> {
    pub removing_results: HashMap<Item, Option<color_eyre::Result<()>>>,
    pub rx: Receiver<(Item, color_eyre::Result<()>)>,
}

impl<Item: Clone + PartialEq + Eq + Hash> CleanningState<Item> {
    fn new(
        for_removal: Vec<Item>,
        rx: Receiver<(Item, color_eyre::Result<()>)>,
    ) -> Self {
        Self {
            removing_results: for_removal.into_iter().map(|c| (c, None)).collect(),
            rx,
        }
    }

    /// Returns boolean flag, if the udelrying channel was closed or not
    pub fn try_update(&mut self) -> bool {
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

impl CleanningState<SCellImageInfo> {
    pub fn cleaning_images(
        for_removal: Vec<SCellImageInfo>,
        buildkit: BuildKitD,
    ) -> App {
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

        App::CleanningImages(Self::new(for_removal, rx))
    }
}

impl CleanningState<SCellContainerInfo> {
    pub fn cleaning_containers(
        for_removal: Vec<SCellContainerInfo>,
        buildkit: BuildKitD,
    ) -> App {
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

        App::CleanningContainers(Self::new(for_removal, rx))
    }
}
