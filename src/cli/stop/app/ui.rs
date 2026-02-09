use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

use super::{App, StoppingState};

impl Widget for &App {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        if let App::Loading { .. } = self {
            render_loading(area, buf);
        }
        if let App::Stopping(state) = self {
            render_stopping(state, area, buf);
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
            "Fetching containers from Docker",
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

#[allow(clippy::indexing_slicing)]
fn render_stopping(
    state: &StoppingState,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = main_block();
    let inner = block.inner(area);
    Widget::render(block, area, buf);

    // Calculate progress
    let total = state.containers.len();
    let completed = state.containers.values().filter(|v| v.is_some()).count();
    let is_done = completed == total;

    // Create header with progress
    let progress_text = if is_done {
        Line::from(vec![
            Span::styled(
                "✓ ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "All containers stopped",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                "⟳ ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("Stopping containers... [{completed}/{total}]"),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    };

    let layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(inner);

    // Render progress header
    let progress_paragraph = Paragraph::new(progress_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().light_magenta()),
        );
    Widget::render(progress_paragraph,layout[0], buf);

    // Create list items for each container
    let list_items: Vec<ListItem> = state
        .containers
        .iter()
        .map(|(info, status)| {
            let (icon, style) = match status {
                None => ("◌", Style::default().fg(Color::Gray)),
                Some(Ok(())) => ("✓", Style::default().fg(Color::Green)),
                Some(Err(_)) => ("✗", Style::default().fg(Color::Red)),
            };

            let mut lines = vec![Line::from(vec![
                Span::styled(icon, style.add_modifier(Modifier::BOLD)),
                Span::styled(
                    info.container_name.as_str(),
                    style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" ({}+{})", info.location.display(), info.name),
                    Style::default().fg(Color::DarkGray),
                ),
            ])];

            // Add error message if there's an error
            if let Some(Err(err)) = status {
                lines.push(Line::from(vec![
                    Span::styled("  └─ ", Style::default().fg(Color::Red)),
                    Span::styled(format!("Error: {err}"), Style::default().fg(Color::Red)),
                ]));
            }

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(list_items);

     Widget::render(list, layout[1], buf);
}

fn main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title("Stopping Shell-Cell Containers")
        .title_bottom("Ctrl-C or Ctrl-D: exit")
        .border_style(Style::new().light_magenta())
}
