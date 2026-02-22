mod confirm_remove;
mod ls;
mod removing;
mod stopping;
mod ui;

use std::sync::mpsc::Receiver;

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo},
    cli::ls::app::{
        confirm_remove::ConfirmRemoveState, ls::LsState, removing::RemovingState,
        stopping::StoppingState,
    },
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
    Ls(LsState<SCellContainerInfo>),
    /// Displaying the help overlay over the container table.
    Help(LsState<SCellContainerInfo>),
    /// Stopping a selected container and refreshing the list.
    Stopping(StoppingState<SCellContainerInfo>),
    /// Confirming removal of a selected container.
    ConfirmRemove(ConfirmRemoveState<SCellContainerInfo>),
    /// Removing a selected container and refreshing the list.
    Removing(RemovingState<SCellContainerInfo>),
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
                KeyCode::Char('h') => {
                    match self {
                        App::Ls(ls_state) => self = App::Help(ls_state),
                        App::Help(ls_state) => self = App::Ls(ls_state),
                        _ => {},
                    }
                },
                KeyCode::Down | KeyCode::Char('j') => {
                    if let App::Ls(ref mut ls_state) = self {
                        ls_state.next();
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => {
                    if let App::Ls(ref mut ls_state) = self {
                        ls_state.previous();
                    }
                },
                KeyCode::Char('s') => {
                    if let App::Ls(ls_state) = self {
                        self = App::Stopping(ls_state.stop_selected()?);
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
                        App::Help(ls_state) => self = App::Ls(ls_state),
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
