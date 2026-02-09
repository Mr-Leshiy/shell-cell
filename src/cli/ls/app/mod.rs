mod ui;

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::TableState,
};

use crate::{scell::container_info::SCellContainerInfo};

pub enum App {
    Loading,
    Ls(LsState),
    Exit,
}

impl App {
    pub fn run<B: ratatui::backend::Backend>(
        mut self,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()> {
        loop {
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

    fn handle_key_event(&mut self) -> color_eyre::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c')
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
                        _ => {},
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct LsState {
    containers: Vec<SCellContainerInfo>,
    table_state: TableState,
}

impl LsState {
    pub fn new(containers: Vec<SCellContainerInfo>) -> Self {
        let mut table_state = TableState::default();
        if !containers.is_empty() {
            table_state.select(Some(0));
        }
        Self {
            containers,
            table_state,
        }
    }

    fn next(&mut self) {
        if self.containers.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.containers.len().saturating_sub(1) {
                    0
                } else {
                    i.saturating_add(1)
                }
            },
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.containers.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.containers.len().saturating_sub(1)
                } else {
                    i.saturating_sub(1)
                }
            },
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
