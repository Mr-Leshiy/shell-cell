use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Widget},
};
use tui_scrollbar::{ScrollBar, ScrollBarArrows, ScrollLengths};
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::inspect::InspectState,
};

impl Widget for &InspectState<SCellContainerInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        render_inspect_window(self.definition.as_deref(), area, buf);
    }
}

impl Widget for &InspectState<SCellImageInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        render_inspect_window(self.definition.as_deref(), area, buf);
    }
}

#[allow(clippy::indexing_slicing)]
fn render_inspect_window(
    definition: Option<&str>,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
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

    let overlay_area = horizontal[1];

    let content = definition.unwrap_or("No definition available");

    let lines: Vec<Line> = content
        .lines()
        .map(|l| {
            Line::from(Span::styled(
                l.to_string(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    Clear.render(overlay_area, buf);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Definition ")
        .title_bottom("i / Esc: close this window")
        .title_alignment(HorizontalAlignment::Center)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_overlay_arrea = overlay_area.inner(Margin::new(1, 1));
    let inner_split = Layout::horizontal([Constraint::Percentage(100), Constraint::Length(1)])
        .split(inner_overlay_arrea);
    let content_area = inner_split[0];
    let scrollbar_area = inner_split[1];

    let mut state = ScrollViewState::new();
    let mut scroll_view = ScrollView::new(content_area.as_size());

    let scroll_bar_lengths = ScrollLengths {
        content_len: lines.len(),
        viewport_len: content_area.height.into(),
    };
    let paragraph = Paragraph::new(lines);

    scroll_view.render_widget(
        paragraph,
        Rect::new(0, 0, content_area.width, content_area.height),
    );
    scroll_view.render(content_area, buf, &mut state);
    if scroll_bar_lengths.content_len > scroll_bar_lengths.viewport_len {
        let scroll_bar = ScrollBar::vertical(scroll_bar_lengths)
            .arrows(ScrollBarArrows::Both)
            .offset(state.offset().y.into());
        scroll_bar.render(scrollbar_area, buf);
    }
    block.render(overlay_area, buf);
}
