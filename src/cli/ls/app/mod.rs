mod confirm_remove;
mod ls;
mod removing;
mod stopping;
mod ui;

use std::sync::mpsc::Receiver;

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};

use crate::{
    buildkit::{BuildKitD, container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::{
        MIN_FPS,
        ls::app::{
            confirm_remove::ConfirmRemoveState, ls::LsState, removing::RemovingState,
            stopping::StoppingState,
        },
    },
};

/// State machine for the `ls` interactive TUI.
///
/// Transitions:
/// - `Loading` → `Ls` (once container list is fetched)
/// - `Ls` → `Stopping` (user presses `s` on a selected container)
/// - `Ls` → `ConfirmRemove` (user presses `r` on a selected container)
/// - `Ls` → `Helo` (user presses `h`)
/// - `ConfirmRemove` → `Removing` (user confirms with `y`)
/// - `ConfirmRemove` → `Ls` (user cancels with `n` or `Esc`)
/// - `Stopping` → `Ls` (once the container is stopped and the list is refreshed)
/// - `Removing` → `Ls` (once the container is removed and the list is refreshed)
/// - Any state → `Exit` (user presses `Ctrl-C` or `Ctrl-D`)
pub enum App {
    Containers(AppInner<SCellContainerInfo>),
    Images(AppInner<SCellImageInfo>),
}

pub enum AppInner<Item> {
    /// Fetching the item list from Docker in the background.
    Loading {
        rx: Receiver<color_eyre::Result<Vec<Item>>>,
        buildkit: BuildKitD,
    },
    /// Displaying the interactive item table.
    Ls(LsState<Item>),
    /// Displaying the help overlay over the item table.
    Help(LsState<Item>),
    /// Stopping a selected item and refreshing the list.
    Stopping(StoppingState<Item>),
    /// Confirming removal of a selected item.
    ConfirmRemove(ConfirmRemoveState<Item>),
    /// Removing a selected item and refreshing the list.
    Removing(RemovingState<Item>),
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
        let mut app = Self::Containers(AppInner::<SCellContainerInfo>::loading(buildkit.clone()));

        loop {
            let new_app = match app {
                Self::Containers(app) => app.run_one_turn(buildkit)?.map(Self::Containers),
                Self::Images(app) => app.run_one_turn(buildkit)?.map(Self::Images),
            };

            let Some(new_app) = new_app else {
                // Exit
                return Ok(());
            };
            app = new_app;

            match &app {
                Self::Containers(app) => {
                    terminal
                        .draw(|f| {
                            f.render_widget(app, f.area());
                        })
                        .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;
                },
                Self::Images(app) => {
                    terminal
                        .draw(|f| {
                            f.render_widget(app, f.area());
                        })
                        .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;
                },
            }

            app = app.handle_key_event(buildkit)?;
        }
    }

    /// Handles a single key event, dispatching navigation and actions
    /// based on the current state.
    fn handle_key_event(
        mut self,
        buildkit: &BuildKitD,
    ) -> color_eyre::Result<Self> {
        if event::poll(MIN_FPS)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match self {
                Self::Containers(app) => {
                    self = app.handle_key_event(key)?.map_or_else(
                        || Self::Images(AppInner::<SCellImageInfo>::loading(buildkit.clone())),
                        Self::Containers,
                    );
                },
                Self::Images(app) => {
                    self = app.handle_key_event(key)?.map_or_else(
                        || {
                            Self::Containers(AppInner::<SCellContainerInfo>::loading(
                                buildkit.clone(),
                            ))
                        },
                        Self::Images,
                    );
                },
            }
        }

        Ok(self)
    }
}

impl<Item: Clone> AppInner<Item> {
    /// Runs only ONE TUI event loop, polling for state transitions and key events.
    /// Returns `None` if its `Exit` state.
    fn run_one_turn(
        mut self,
        buildkit: &BuildKitD,
    ) -> color_eyre::Result<Option<Self>> {
        // Loading → Ls: container list is ready
        if let Self::Loading {
            ref rx,
            ref buildkit,
        } = self
            && let Ok(result) = rx.try_recv()
        {
            let items = result?;
            self = Self::Ls(LsState::new(items, buildkit.clone()));
        }

        // Stopping → Ls: stop finished and refreshed list is available
        if let Self::Stopping(ref mut state) = self
            && let Some(items) = state.try_recv()?
        {
            self = Self::Ls(LsState::new(items, buildkit.clone()));
        }

        // Removing → Ls: remove finished and refreshed list is available
        if let Self::Removing(ref mut state) = self
            && let Some(items) = state.try_recv()?
        {
            self = Self::Ls(LsState::new(items, buildkit.clone()));
        }

        if matches!(self, Self::Exit) {
            return Ok(None);
        }

        Ok(Some(self))
    }
}

impl AppInner<SCellContainerInfo> {
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

        Self::Loading { rx, buildkit }
    }

    /// Handles a single key event, dispatching navigation and actions
    /// based on the current state.
    fn handle_key_event(
        mut self,
        key: KeyEvent,
    ) -> color_eyre::Result<Option<Self>> {
        match key.code {
            KeyCode::Char('q') => {
                if let Self::Ls(_) = self {
                    // switching to images
                    return Ok(None);
                }
            },
            KeyCode::Char('c' | 'd') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self = Self::Exit;
            },
            KeyCode::Char('h') => {
                match self {
                    Self::Ls(ls_state) => self = Self::Help(ls_state),
                    Self::Help(ls_state) => self = Self::Ls(ls_state),
                    _ => {},
                }
            },
            KeyCode::Down | KeyCode::Char('j') => {
                if let Self::Ls(ref mut ls_state) = self {
                    ls_state.next();
                }
            },
            KeyCode::Up | KeyCode::Char('k') => {
                if let Self::Ls(ref mut ls_state) = self {
                    ls_state.previous();
                }
            },
            KeyCode::Char('s') => {
                if let Self::Ls(ls_state) = self {
                    self = Self::Stopping(ls_state.stop_selected()?);
                }
            },
            KeyCode::Char('r') => {
                if let Self::Ls(ls_state) = self {
                    self = Self::ConfirmRemove(ls_state.confirm_remove()?);
                }
            },
            KeyCode::Char('y') => {
                if let Self::ConfirmRemove(confirm_state) = self {
                    self = Self::Removing(confirm_state.confirm());
                }
            },
            KeyCode::Char('n') => {
                if let Self::ConfirmRemove(confirm_state) = self {
                    self = Self::Ls(confirm_state.cancel());
                }
            },
            KeyCode::Esc => {
                match self {
                    Self::Help(ls_state) => self = Self::Ls(ls_state),
                    Self::ConfirmRemove(confirm_state) => {
                        self = Self::Ls(confirm_state.cancel());
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        Ok(Some(self))
    }
}

impl AppInner<SCellImageInfo> {
    /// Creates a new `App` in the `Loading` state, spawning an async task
    /// that fetches the current Shell-Cell image list.
    fn loading(buildkit: BuildKitD) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        tokio::spawn({
            let buildkit = buildkit.clone();
            async move {
                let result = async {
                    let res = buildkit.list_images().await;
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    res
                }
                .await;
                drop(tx.send(result));
            }
        });

        Self::Loading { rx, buildkit }
    }

    /// Handles a single key event, dispatching navigation and actions
    /// based on the current state.
    fn handle_key_event(
        mut self,
        key: KeyEvent,
    ) -> color_eyre::Result<Option<Self>> {
        match key.code {
            KeyCode::Char('q') => {
                if let Self::Ls(_) = self {
                    // switching to containers
                    return Ok(None);
                }
            },
            KeyCode::Char('c' | 'd') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self = Self::Exit;
            },
            KeyCode::Char('h') => {
                match self {
                    Self::Ls(ls_state) => self = Self::Help(ls_state),
                    Self::Help(ls_state) => self = Self::Ls(ls_state),
                    _ => {},
                }
            },
            KeyCode::Down | KeyCode::Char('j') => {
                if let Self::Ls(ref mut ls_state) = self {
                    ls_state.next();
                }
            },
            KeyCode::Up | KeyCode::Char('k') => {
                if let Self::Ls(ref mut ls_state) = self {
                    ls_state.previous();
                }
            },
            KeyCode::Char('r') => {
                if let Self::Ls(ls_state) = self {
                    self = Self::ConfirmRemove(ls_state.confirm_remove()?);
                }
            },
            KeyCode::Char('y') => {
                if let Self::ConfirmRemove(confirm_state) = self {
                    self = Self::Removing(confirm_state.confirm());
                }
            },
            KeyCode::Char('n') => {
                if let Self::ConfirmRemove(confirm_state) = self {
                    self = Self::Ls(confirm_state.cancel());
                }
            },
            KeyCode::Esc => {
                match self {
                    Self::Help(ls_state) => self = Self::Ls(ls_state),
                    Self::ConfirmRemove(confirm_state) => {
                        self = Self::Ls(confirm_state.cancel());
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        Ok(Some(self))
    }
}
