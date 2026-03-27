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
            .title(format!(
                "'Shell-Cell' | {} | {} | {}",
                self.container_id,
                self.target_name,
                self.location.display()
            ))
            .title_bottom("Ctrl-H: Help");
        let inner = block.inner(area);
        block.render(area, buf);

        // set the proper size for the terminal screen
        self.pty.set_size(inner.height, inner.width);
        let is_cursor_visible = self.pty.screen().scrollback() == 0;
        let cursor = tui_term::widget::Cursor::default().visibility(is_cursor_visible);
        tui_term::widget::PseudoTerminal::new(self.pty.screen())
            .cursor(cursor)
            .render(inner, buf);
    }
}
