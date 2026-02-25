use ratatui::{
    style::Style,
    widgets::{Block, Borders, Widget},
};

use crate::cli::run::app::running_pty::RunningPtyState;

impl Widget for &mut RunningPtyState {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().light_magenta())
            .title(format!("'Shell-Cell' {}", self.container_id))
            .title_bottom("Ctrl-D: exit");
        let inner = block.inner(area);
        block.render(area, buf);

        // set the proper size for the terminal screen
        self.pty.set_size(inner.height, inner.width);
        tui_term::widget::PseudoTerminal::new(self.pty.screen()).render(inner, buf);
    }
}
