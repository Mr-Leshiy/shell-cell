use std::fmt::Display;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::stopping::StoppingState,
};

impl Widget for &StoppingState<SCellContainerInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        render_stopping(&self.for_stop.name, area, buf);
    }
}

impl Widget for &StoppingState<SCellImageInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        render_stopping(&self.for_stop.name, area, buf);
    }
}

#[allow(clippy::indexing_slicing)]
fn render_stopping(
    item: impl Display,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .split(vertical[1]);

    let stopping_text = vec![
        Line::from(vec![
            Span::styled(
                "Stopping",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("...", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("Stopping '{item}'"),
            Style::default().fg(Color::Gray),
        )),
    ];

    Widget::render(Clear, horizontal[1], buf);

    let paragraph = Paragraph::new(stopping_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}
