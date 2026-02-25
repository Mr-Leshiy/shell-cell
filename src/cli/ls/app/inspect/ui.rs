use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Rect, Size},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::inspect::InspectState,
};

impl Widget for &mut InspectState<SCellContainerInfo> {
    #[allow(clippy::indexing_slicing)]
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        let (container_desc, _image_id) = &self.data;

        let window_area = prepare_window_area(area, buf);

        let vertical = Layout::vertical([
            Constraint::Length(2),       // image_id line + horizontal separator
            Constraint::Percentage(100), // description scroll area
        ])
        .split(window_area);

        Paragraph::new(Line::from(vec![
            Span::styled(
                "Image ID: ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("N/A", Style::default().fg(Color::White)),
        ]))
        .alignment(HorizontalAlignment::Center)
        .render(vertical[0], buf);

        render_description(
            container_desc.as_deref(),
            &mut self.scroll_state,
            vertical[1],
            buf,
        );
    }
}

impl Widget for &mut InspectState<SCellImageInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        let image_desc = &self.data;

        let window_area = prepare_window_area(area, buf);
        render_description(
            image_desc.as_deref(),
            &mut self.scroll_state,
            window_area,
            buf,
        );
    }
}

#[allow(clippy::indexing_slicing)]
fn prepare_window_area(
    area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
) -> ratatui::prelude::Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .split(vertical[1]);

    let window_area = horizontal[1];
    Clear.render(window_area, buf);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Definition ")
        .title_bottom("i / Esc: close this window")
        .title_alignment(HorizontalAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan));
    let inner_window_area = block.inner(window_area);
    block.render(window_area, buf);
    inner_window_area
}

fn render_description(
    desc: Option<&str>,
    state: &mut ScrollViewState,
    area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let content = desc.unwrap_or("No description available");
    let lines: Vec<Line> = content
        .lines()
        .map(|l| {
            Line::from(Span::styled(
                l.to_string(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let content_height = u16::try_from(lines.len()).unwrap_or(u16::MAX);
    let mut scroll_view = ScrollView::new(Size::new(area.width, content_height))
        .vertical_scrollbar_visibility(ScrollbarVisibility::Automatic)
        .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    let paragraph = Paragraph::new(lines);

    scroll_view.render_widget(paragraph, Rect::new(0, 0, area.width, content_height));
    scroll_view.render(area, buf, state);
}
