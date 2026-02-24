use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::error_window::ErrorWindowState,
};

impl Widget for &ErrorWindowState<SCellContainerInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        render_error_window(&self.message, area, buf)
    }
}

impl Widget for &ErrorWindowState<SCellImageInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        render_error_window(&self.message, area, buf)
    }
}

#[allow(clippy::indexing_slicing)]
fn render_error_window(
    message: &str,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let vertical = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .split(vertical[1]);

    let overlay_area = horizontal[1];

    let mut lines = vec![Line::from(vec![Span::styled(
        "Error",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )])];
    lines.extend(message.lines().map(|l| {
        Line::from(Span::styled(
            l.to_string(),
            Style::default().fg(Color::White),
        ))
    }));

    Widget::render(Clear, overlay_area, buf);

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Error ")
                .title_bottom("Esc: dismiss")
                .title_alignment(HorizontalAlignment::Center)
                .border_style(Style::default().fg(Color::Red)),
        )
        .wrap(Wrap { trim: false });

    paragraph.render(overlay_area, buf);
}
