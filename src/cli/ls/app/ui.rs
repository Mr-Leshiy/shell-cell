use std::fmt::Display;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use super::AppInner;
use crate::buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo};

const CONTAINERS_TITLE: &str = "Containers";

impl Widget for &mut AppInner<SCellContainerInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let inner = render_main_block(CONTAINERS_TITLE, area, buf);
        match self {
            AppInner::Loading { .. } => render_loading(inner, buf),
            AppInner::Ls(ls_state) => ls_state.render(inner, buf),
            AppInner::HelpWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Stopping(state) => {
                state.render(inner, buf);
            },
            AppInner::ConfirmRemove(state) => {
                state.render(inner, buf);
            },
            AppInner::Removing(state) => {
                state.render(inner, buf);
            },
            AppInner::ErrorWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Inspect(state) => {
                state.render(inner, buf);
            },
            AppInner::Exit => {},
        }
    }
}

impl Widget for &mut AppInner<SCellImageInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let inner = render_main_block(CONTAINERS_TITLE, area, buf);
        match self {
            AppInner::Loading { .. } => render_loading(inner, buf),
            AppInner::Ls(ls_state) => {
                ls_state.render(inner, buf);
            },
            AppInner::HelpWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Stopping(state) => {
                state.render(inner, buf);
            },
            AppInner::ConfirmRemove(state) => {
                state.render(inner, buf);
            },
            AppInner::Removing(state) => {
                state.render(inner, buf);
            },
            AppInner::ErrorWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Inspect(state) => {
                state.render(inner, buf);
            },
            AppInner::Exit => {},
        }
    }
}

#[allow(clippy::indexing_slicing)]
fn render_loading(
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
                "Loading",
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

fn render_main_block(
    items_title: impl Display,
    area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
) -> Rect {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("'Shell-Cell' {items_title}"))
        .title_bottom("h: Help")
        .border_style(Style::new().light_magenta());
    let inner = block.inner(area);
    block.render(area, buf);
    inner
}
