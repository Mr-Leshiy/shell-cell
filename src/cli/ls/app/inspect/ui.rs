use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Margin, Rect, Size},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidget, Tabs, Widget},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::inspect::InspectState,
};

impl Widget for &mut InspectState<SCellContainerInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);
        render_inspect_window(
            self.description.as_deref(),
            &mut self.scroll_state,
            area,
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
        render_inspect_window(
            self.description.as_deref(),
            &mut self.scroll_state,
            area,
            buf,
        );
    }
}

#[allow(clippy::indexing_slicing)]
fn render_inspect_window(
    definition: Option<&str>,
    state: &mut ScrollViewState,
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

    let tabs_block = Tabs::new(["Container", "Image"]).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Definition ")
            .title_bottom("i / Esc: close this window")
            .title_alignment(HorizontalAlignment::Center)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    let inner_overlay_arrea = overlay_area.inner(Margin::new(1, 2));

    let content_height = u16::try_from(lines.len()).unwrap_or(u16::MAX);
    let mut scroll_view = ScrollView::new(Size::new(inner_overlay_arrea.width, content_height))
        .vertical_scrollbar_visibility(ScrollbarVisibility::Automatic)
        .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

    let paragraph = Paragraph::new(lines);

    scroll_view.render_widget(
        paragraph,
        Rect::new(0, 0, inner_overlay_arrea.width, content_height),
    );
    scroll_view.render(inner_overlay_arrea, buf, state);
    tabs_block.render(overlay_area, buf);
}
