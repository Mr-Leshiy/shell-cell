mod ui;

use std::sync::mpsc::{Receiver, RecvTimeoutError};

use color_eyre::eyre::ContextCompat;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::TableState,
};

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo},
    cli::MIN_FPS,
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
    /// Runs the TUI event loop, polling for state transitions and key events.
    pub fn run<B: ratatui::backend::Backend>(
        buildkit: &BuildKitD,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()> {
        // First step
        let mut app = Self::loading(buildkit.clone());

        loop {
            // Loading → Ls: container list is ready
            if let App::Loading {
                ref rx,
                ref buildkit,
            } = app
                && let Ok(result) = rx.try_recv()
            {
                let containers = result?;
                app = App::Ls(LsState::new(containers, buildkit.clone()));
            }

            // Stopping → Ls: stop finished and refreshed list is available
            if let App::Stopping(ref mut state) = app
                && let Some(containers) = state.try_recv()?
            {
                app = App::Ls(LsState::new(containers, buildkit.clone()));
            }

            // Removing → Ls: remove finished and refreshed list is available
            if let App::Removing(ref mut state) = app
                && let Some(containers) = state.try_recv()?
            {
                app = App::Ls(LsState::new(containers, buildkit.clone()));
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

    /// Handles a single key event, dispatching navigation and actions
    /// based on the current state.
    fn handle_key_event(mut self) -> color_eyre::Result<Self> {
        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('c' | 'd')
                    if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    self = App::Exit;
                },
                KeyCode::Char('i') => {
                    if let App::Ls(ref mut ls_state) = self {
                        ls_state.show_help = !ls_state.show_help;
                    }
                },
                KeyCode::Down | KeyCode::Char('j') => {
                    if let App::Ls(ref mut ls_state) = self
                        && !ls_state.show_help
                    {
                        ls_state.next();
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => {
                    if let App::Ls(ref mut ls_state) = self
                        && !ls_state.show_help
                    {
                        ls_state.previous();
                    }
                },
                KeyCode::Char('s') => {
                    if let App::Ls(ls_state) = self {
                        if ls_state.show_help {
                            self = App::Ls(ls_state);
                        } else {
                            self = App::Stopping(ls_state.stop_selected()?);
                        }
                    }
                },
                KeyCode::Char('r') => {
                    if let App::Ls(ls_state) = self {
                        self = App::ConfirmRemove(ls_state.confirm_remove()?);
                    }
                },
                KeyCode::Char('y') => {
                    if let App::ConfirmRemove(confirm_state) = self {
                        self = App::Removing(confirm_state.confirm());
                    }
                },
                KeyCode::Char('n') => {
                    if let App::ConfirmRemove(confirm_state) = self {
                        self = App::Ls(confirm_state.cancel());
                    }
                },
                KeyCode::Esc => {
                    match self {
                        App::Ls(ref mut ls_state) if ls_state.show_help => {
                            ls_state.show_help = false;
                        },
                        App::ConfirmRemove(confirm_state) => {
                            self = App::Ls(confirm_state.cancel());
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        Ok(self)
    }

    /// Creates a new `App` in the `Loading` state, spawning an async task
    /// that fetches the current Shell-Cell container list.
    fn loading(buildkit: BuildKitD) -> Self {
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
}

/// Holds the data for the interactive container table view.
pub struct LsState {
    containers: Vec<SCellContainerInfo>,
    table_state: TableState,
    buildkit: BuildKitD,
    show_help: bool,
}

impl LsState {
    pub fn new(
        containers: Vec<SCellContainerInfo>,
        buildkit: BuildKitD,
    ) -> Self {
        let mut table_state = TableState::default();
        if !containers.is_empty() {
            table_state.select(Some(0));
        }
        Self {
            containers,
            table_state,
            buildkit,
            show_help: false,
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
    /// the full container list.
    fn stop_selected(self) -> color_eyre::Result<StoppingState> {
        let selected = self
            .table_state
            .selected()
            .context("Some item in the list must be selected")?;
        let container = self
            .containers
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
            container_name: container.name.to_string(),
            rx,
        })
    }

    /// Shows confirmation dialog for removing the currently selected container.
    fn confirm_remove(self) -> color_eyre::Result<ConfirmRemoveState> {
        let selected = self
            .table_state
            .selected()
            .context("Some item in the list must be selected")?;
        let container = self
            .containers
            .get(selected)
            .context("Selected item must be present in the list")?;

        Ok(ConfirmRemoveState {
            selected_to_remove: container.clone(),
            containers: self.containers,
            buildkit: self.buildkit,
        })
    }
}

/// Holds the state while waiting for user confirmation to remove a container.
///
/// Displays a warning that all container state will be lost and waits for
/// the user to press 'y' (confirm) or 'n'/'Esc' (cancel).
pub struct ConfirmRemoveState {
    selected_to_remove: SCellContainerInfo,
    containers: Vec<SCellContainerInfo>,
    buildkit: BuildKitD,
}

impl ConfirmRemoveState {
    /// User confirmed removal - initiate the removal process.
    fn confirm(self) -> RemovingState {
        let (tx, rx) = std::sync::mpsc::channel();
        let buildkit = self.buildkit;
        let container_name = self.selected_to_remove.name.to_string();
        tokio::spawn({
            async move {
                let res = buildkit.cleanup_container(&self.selected_to_remove).await;
                let res = match res {
                    Ok(()) => buildkit.list_containers().await,
                    Err(e) => Err(e),
                };
                drop(tx.send(res));
            }
        });

        RemovingState { container_name, rx }
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
    container_name: String,
    rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
}

impl StoppingState {
    /// Polls the background stop task for completion.
    ///
    /// Returns `Some((containers, buildkit))` with the refreshed container
    /// list when the operation is done, or `None` if still in progress.
    fn try_recv(&mut self) -> color_eyre::Result<Option<Vec<SCellContainerInfo>>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(result) => result.map(Some),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "StoppingState channel cannot be disconnected without returning a result"
                )
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
    rx: Receiver<color_eyre::Result<Vec<SCellContainerInfo>>>,
}

impl RemovingState {
    /// Polls the background remove task for completion.
    ///
    /// Returns `Some((containers, buildkit))` with the refreshed container
    /// list when the operation is done, or `None` if still in progress.
    fn try_recv(&mut self) -> color_eyre::Result<Option<Vec<SCellContainerInfo>>> {
        match self.rx.recv_timeout(MIN_FPS) {
            Ok(result) => result.map(Some),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => {
                color_eyre::eyre::bail!(
                    "RemovingState channel cannot be disconnected without returning a result"
                )
            },
        }
    }
}
