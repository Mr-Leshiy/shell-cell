use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::cli::cleanup::app::App;

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

        if let App::Loading { .. } = self {
            render_loading(area, buf);
        }
        if let App::CleanningContainers(state) = self {
            Widget::render(state, inner, buf);
        }
        if let App::CleanningImages(state) = self {
            Widget::render(state, inner, buf);
        }
    }
}

#[allow(clippy::indexing_slicing)]
fn render_loading(
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = main_block();
    let inner = block.inner(area);
    Widget::render(block, area, buf);

    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Length(3),
        Constraint::Percentage(40),
    ])
    .split(inner);

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
            "Fetching 'Shell-Cell' containers and images for cleaning",
            Style::default().fg(Color::Gray),
        )),
    ];

    let paragraph = Paragraph::new(loading_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .centered();

    Widget::render(paragraph, horizontal[1], buf);
}

fn main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(format!(
            "Cleaning 'Shell-Cell' Containers and Images{}",
            crate::debugger::Debugger::session_id()
                .map(|id| format!(" | Debug Session: {id}"))
                .unwrap_or_default()
        ))
        .title_bottom("Ctrl-C or Ctrl-D: exit")
        .border_style(Style::new().light_magenta())
}
