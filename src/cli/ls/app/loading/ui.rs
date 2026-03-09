use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::loading::LoadingState,
};

impl Widget for &LoadingState<SCellContainerInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        render_loading("containers", area, buf);
    }
}

impl Widget for &LoadingState<SCellImageInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        render_loading("images", area, buf);
    }
}

#[allow(clippy::indexing_slicing)]
fn render_loading(
    list_type: &str,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Length(3),
        Constraint::Percentage(40),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .split(vertical[1]);

    let loading_text = vec![
        Line::from(vec![
            Span::styled(
                format!("Loading {list_type}"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("...", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Fetching 'Shell-Cell' info",
            Style::default().fg(Color::Gray),
        )),
    ];

    Widget::render(Clear, horizontal[1], buf);

    let paragraph = Paragraph::new(loading_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}
