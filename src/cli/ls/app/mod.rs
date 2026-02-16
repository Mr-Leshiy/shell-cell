mod ui;

use std::sync::mpsc::{Receiver, RecvTimeoutError};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::TableState,
};

use crate::{
    buildkit::BuildKitD,
    cli::MIN_FPS,
    scell::container_info::SCellContainerInfo,
};

/// State machine for the `ls` interactive TUI.
///
/// Transitions:
/// - `Loading` → `Ls` (once container list is fetched)
/// - `Ls` → `Stopping` (user presses `s` on a selected container)
/// - `Ls` → `ConfirmRemove` (user presses `r` on a selected container)
/// - `ConfirmRemove` → `Removing` (user confirms with `y`)
/// - `ConfirmRemove` → `Ls` (user cancels with `n` or `Esc`)
/// - `Stopping` → `Ls` (once the container is stopped and the list is refreshed)
/// - `Removing` → `Ls` (once the container is removed and the list is refreshed)
/// - Any state → `Exit` (user presses `Ctrl-C` or `Ctrl-D`)
pub enum App {
    /// Fetching the container list from Docker in the background.
    Loading {
        rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
        buildkit: BuildKitD,
    },
    /// Displaying the interactive container table.
    Ls(LsState),
    /// Stopping a selected container and refreshing the list.
    Stopping(StoppingState),
    /// Confirming removal of a selected container.
    ConfirmRemove(ConfirmRemoveState),
    /// Removing a selected container and refreshing the list.
    Removing(RemovingState),
    /// Terminal state — the event loop exits.
    Exit,
}

impl App {
    /// Creates a new `App` in the `Loading` state, spawning an async task
    /// that fetches the current Shell-Cell container list.
    pub fn loading(buildkit: BuildKitD) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

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

    /// Runs the TUI event loop, polling for state transitions and key events.
    pub fn run<B: ratatui::backend::Backend>(
        mut self,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()> {
        loop {
            // Loading → Ls: container list is ready
            if let App::Loading {
                ref rx,
                ref buildkit,
            } = self
                && let Ok(result) = rx.try_recv()
            {
                let containers = result?;
                self = App::Ls(LsState::new(containers, buildkit.clone()));
            }

            // Stopping → Ls: stop finished and refreshed list is available
            if let App::Stopping(ref mut state) = self
                && let Some((containers, buildkit)) = state.try_recv()
            {
                self = App::Ls(LsState::new(containers, buildkit));
            }

            // Removing → Ls: remove finished and refreshed list is available
            if let App::Removing(ref mut state) = self
                && let Some((containers, buildkit)) = state.try_recv()
            {
                self = App::Ls(LsState::new(containers, buildkit));
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

    /// Handles a single key event, dispatching navigation and actions
    /// based on the current state.
    fn handle_key_event(&mut self) -> color_eyre::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('c' | 'd')
                    if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    *self = App::Exit;
                },
                KeyCode::Down | KeyCode::Char('j') => {
                    if let App::Ls(ls_state) = self {
                        ls_state.next();
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => {
                    if let App::Ls(ls_state) = self {
                        ls_state.previous();
                    }
                },
                KeyCode::Char('s') => {
                    if let App::Ls(ls_state) = self
                        && let Some(stopping) = ls_state.stop_selected()
                    {
                        *self = App::Stopping(stopping);
                    }
                },
                KeyCode::Char('r') => {
                    if let App::Ls(ls_state) = self
                        && let Some(confirm) = ls_state.confirm_remove()
                    {
                        *self = App::ConfirmRemove(confirm);
                    }
                },
                KeyCode::Char('y') => {
                    if let App::ConfirmRemove(_) = self {
                        let confirm_state = std::mem::replace(self, App::Exit);
                        if let App::ConfirmRemove(state) = confirm_state {
                            *self = App::Removing(state.confirm());
                        }
                    }
                },
                KeyCode::Char('n') | KeyCode::Esc => {
                    if let App::ConfirmRemove(_) = self {
                        let confirm_state = std::mem::replace(self, App::Exit);
                        if let App::ConfirmRemove(state) = confirm_state {
                            *self = App::Ls(state.cancel());
                        }
                    }
                },
                _ => {},
            }
        }
        Ok(())
    }
}

/// Holds the data for the interactive container table view.
pub struct LsState {
    containers: Vec<SCellContainerInfo>,
    table_state: TableState,
    buildkit: BuildKitD,
}

impl LsState {
    pub fn new(containers: Vec<SCellContainerInfo>, buildkit: BuildKitD) -> Self {
        let mut table_state = TableState::default();
        if !containers.is_empty() {
            table_state.select(Some(0));
        }
        Self {
            containers,
            table_state,
            buildkit,
        }
    }

    /// Moves the table selection to the next row, wrapping to the top.
    fn next(&mut self) {
        if self.containers.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) if i != self.containers.len().saturating_sub(1) => i.saturating_add(1),
            _ => 0,
        };
        self.table_state.select(Some(i));
    }

    /// Moves the table selection to the previous row, wrapping to the bottom.
    fn previous(&mut self) {
        if self.containers.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) if i != 0 => i.saturating_sub(1),
            _ => self.containers.len().saturating_sub(1),
        };
        self.table_state.select(Some(i));
    }

    /// Initiates stopping of the currently selected container.
    ///
    /// Spawns an async task that stops the container and then re-fetches
    /// the full container list. Returns `None` if no container is selected.
    fn stop_selected(&mut self) -> Option<StoppingState> {
        let selected = self.table_state.selected()?;
        let container = self.containers.get(selected)?.clone();
        let buildkit = self.buildkit.clone();

        let (tx, rx) = std::sync::mpsc::channel();
        let container_name = container.name.to_string();

        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let res = buildkit
                    .stop_container_by_name(&container_name)
                    .await;
                let result = match res {
                    Ok(()) => buildkit.list_containers().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(result));
            }
        });

        Some(StoppingState {
            container_name: container.name.to_string(),
            buildkit,
            rx,
        })
    }

    /// Shows confirmation dialog for removing the currently selected container.
    ///
    /// Returns `None` if no container is selected.
    fn confirm_remove(&self) -> Option<ConfirmRemoveState> {
        let selected = self.table_state.selected()?;
        let container = self.containers.get(selected)?.clone();

        Some(ConfirmRemoveState {
            container_name: container.name.to_string(),
            containers: self.containers.clone(),
            buildkit: self.buildkit.clone(),
        })
    }
}

/// Holds the state while waiting for user confirmation to remove a container.
///
/// Displays a warning that all container state will be lost and waits for
/// the user to press 'y' (confirm) or 'n'/'Esc' (cancel).
pub struct ConfirmRemoveState {
    /// Name of the container to be removed (used for UI display).
    container_name: String,
    containers: Vec<SCellContainerInfo>,
    buildkit: BuildKitD,
}

impl ConfirmRemoveState {
    /// User confirmed removal - initiate the removal process.
    fn confirm(self) -> RemovingState {
        let (tx, rx) = std::sync::mpsc::channel();
        let container_name = self.container_name.clone();
        let buildkit = self.buildkit.clone();

        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let res = buildkit
                    .cleanup_container_by_name(&container_name)
                    .await;
                let result = match res {
                    Ok(()) => buildkit.list_containers().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(result));
            }
        });

        RemovingState {
            container_name: self.container_name,
            buildkit: self.buildkit,
            rx,
        }
    }

    /// User cancelled removal - return to the list view.
    fn cancel(self) -> LsState {
        LsState::new(self.containers, self.buildkit)
    }
}

/// Holds the state while a container is being stopped in the background.
///
/// The spawned task stops the container and then re-fetches the full
/// container list, sending the result back over the channel.
pub struct StoppingState {
    /// Name of the container being stopped (used for UI display).
    container_name: String,
    buildkit: BuildKitD,
    rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
}

impl StoppingState {
    /// Polls the background stop task for completion.
    ///
    /// Returns `Some((containers, buildkit))` with the refreshed container
    /// list when the operation is done, or `None` if still in progress.
    fn try_recv(&mut self) -> Option<(Vec<SCellContainerInfo>, BuildKitD)> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(result) => {
                let containers = result.unwrap_or_default();
                Some((containers, self.buildkit.clone()))
            },
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                Some((Vec::new(), self.buildkit.clone()))
            },
        }
    }
}

/// Holds the state while a container is being removed in the background.
///
/// The spawned task removes the container (and its image) and then re-fetches
/// the full container list, sending the result back over the channel.
pub struct RemovingState {
    /// Name of the container being removed (used for UI display).
    container_name: String,
    buildkit: BuildKitD,
    rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
}

impl RemovingState {
    /// Polls the background remove task for completion.
    ///
    /// Returns `Some((containers, buildkit))` with the refreshed container
    /// list when the operation is done, or `None` if still in progress.
    fn try_recv(&mut self) -> Option<(Vec<SCellContainerInfo>, BuildKitD)> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(result) => {
                let containers = result.unwrap_or_default();
                Some((containers, self.buildkit.clone()))
            },
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                Some((Vec::new(), self.buildkit.clone()))
            },
        }
    }
}
