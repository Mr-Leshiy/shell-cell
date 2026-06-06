use ratatui::{
    style::Style,
    widgets::{Block, Borders, Widget},
};

use super::App;

impl Widget for &App {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let block = main_block();
        let inner = block.inner(area);
        Widget::render(block, area, buf);

        if let App::Loading(loading) = self {
            Widget::render(loading, inner, buf);
        }
        if let App::Stopping(stopping) = self {
            Widget::render(stopping, inner, buf);
        }
    }
}

fn main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(format!(
            "Stopping Shell-Cell Containers{}",
            crate::debugger::Debugger::session_id()
                .map(|id| format!(" | Debug Session: {id}"))
                .unwrap_or_default()
        ))
        .title_bottom("Ctrl-C or Ctrl-D: exit")
        .border_style(Style::new().light_magenta())
}
