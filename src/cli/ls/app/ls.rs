use color_eyre::eyre::ContextCompat;
use ratatui::widgets::TableState;

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo},
    cli::ls::app::{confirm_remove::ConfirmRemoveState, stopping::StoppingState},
};

/// Holds the data for the interactive container table view.
pub struct LsState<Item> {
    pub items: Vec<Item>,
    pub table_state: TableState,
    pub buildkit: BuildKitD,
}

impl<Item> LsState<Item> {
    pub fn new(
        items: Vec<Item>,
        buildkit: BuildKitD,
    ) -> Self {
        let mut table_state = TableState::default();
        if !items.is_empty() {
            table_state.select(Some(0));
        }
        Self {
            items,
            table_state,
            buildkit,
        }
    }

    /// Moves the table selection to the next row, wrapping to the top.
    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) if i != self.items.len().saturating_sub(1) => i.saturating_add(1),
            _ => 0,
        };
        self.table_state.select(Some(i));
    }

    /// Moves the table selection to the previous row, wrapping to the bottom.
    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) if i != 0 => i.saturating_sub(1),
            _ => self.items.len().saturating_sub(1),
        };
        self.table_state.select(Some(i));
    }
}

impl LsState<SCellContainerInfo> {
    /// Initiates stopping of the currently selected container.
    ///
    /// Spawns an async task that stops the container and then re-fetches
    /// the full container list.
    pub fn stop_selected(self) -> color_eyre::Result<StoppingState> {
        let selected = self
            .table_state
            .selected()
            .context("Some item in the list must be selected")?;
        let container = self
            .items
            .get(selected)
            .context("Selected item must be present in the list")?;
        let buildkit = self.buildkit.clone();

        let (tx, rx) = std::sync::mpsc::channel();

        tokio::spawn({
            let container = container.clone();
            async move {
                let res = buildkit.stop_container(&container).await;
                let res = match res {
                    Ok(()) => buildkit.list_containers().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(res));
            }
        });

        Ok(StoppingState {
            for_stop: container.clone(),
            ls_state: self,
            rx,
        })
    }

    /// Shows confirmation dialog for removing the currently selected container.
    pub fn confirm_remove(self) -> color_eyre::Result<ConfirmRemoveState> {
        let selected = self
            .table_state
            .selected()
            .context("Some item in the list must be selected")?;
        let container = self
            .items
            .get(selected)
            .context("Selected item must be present in the list")?;

        Ok(ConfirmRemoveState {
            selected_to_remove: container.clone(),
            ls_state: self,
        })
    }
}
