use ratatui::{
    style::Style,
    widgets::{Block, Borders, Widget},
};

use crate::cli::run::app::running_pty::{InputMode, RunningPtyState};

impl Widget for &mut RunningPtyState {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let (border_style, title_bottom) = match self.mode {
            InputMode::Normal => {
                (
                    Style::new().light_magenta(),
                    "Ctrl-B: commands | Ctrl-H: help".to_owned(),
                )
            },
            InputMode::Command => {
                (
                    Style::new().yellow(),
                    "-- COMMAND -- d: detach | ↑/↓/k/j: scroll | Esc: exit".to_owned(),
                )
            },
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(format!(
                "{} | {} | {}{}",
                self.container_id,
                self.target_name,
                self.location.display(),
                crate::debugger::Debugger::session_id()
                    .map(|id| format!(" | Debug Session: {id}"))
                    .unwrap_or_default()
            ))
            .title_bottom(title_bottom);
        let inner = block.inner(area);
        block.render(area, buf);

        // set the proper size for the terminal screen
        self.pty.set_size(inner.height, inner.width);
        tui_term::widget::PseudoTerminal::new(self.pty.screen()).render(inner, buf);
    }
}
