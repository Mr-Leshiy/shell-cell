use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::cli::run::app::App;

impl Widget for &mut App {
    #[allow(clippy::indexing_slicing)]
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        match self {
            App::Preparing(state) => state.render(area, buf),
            App::RunningPty(state) => state.render(area, buf),
            App::HelpWindow(state) => state.render(area, buf),
            App::Finished => {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::new().light_magenta())
                    .title("'Shell-Cell'");
                let inner = block.inner(area);
                block.render(area, buf);

                // Create a centered area using Layout
                let vertical_layout = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Length(2),
                    Constraint::Percentage(50),
                ])
                .split(inner);

                let text = vec![
                    Line::from(Span::styled(
                        "Finished 'Shell-Cell' session",
                        Style::default().add_modifier(Modifier::BOLD).green(),
                    )),
                    Line::from(Span::styled(
                        "<Press any key to exit>",
                        Style::default().cyan(),
                    )),
                ];

                let paragraph = Paragraph::new(text).alignment(Alignment::Center);
                paragraph.render(vertical_layout[1], buf);
            },
            App::Exit => {},
        }
    }
}
