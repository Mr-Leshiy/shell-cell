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
        match self {
            App::Loading { .. } => {
                render_loading(area, buf);
            },
            App::Stopping(state) => {
                render_stopping(state, area, buf);
            },
            App::Exit => {},
        }
    }
}

fn render_loading(
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Shell-Cell Containers")
        .border_style(Style::new().light_green());

    let inner = block.inner(area);
    block.render(area, buf);

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

    paragraph.render(horizontal[1], buf);
}

fn render_stopping(
    state: &StoppingState,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Stopping Shell-Cell Containers")
        .border_style(Style::new().light_yellow());

    let inner = block.inner(area);
    block.render(area, buf);

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
                format!("All {} containers stopped", total),
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
                format!("Stopping containers... [{}/{}]", completed, total),
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
                .border_style(Style::default().fg(Color::Gray)),
        );
    progress_paragraph.render(layout[0], buf);

    // Create list items for each container
    let mut items: Vec<_> = state.containers.iter().collect();
    items.sort_by_key(|(info, _)| &info.container_name);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|(info, status)| {
            let (icon, style) = match status {
                None => ("◌", Style::default().fg(Color::Gray)),
                Some(Ok(())) => ("✓", Style::default().fg(Color::Green)),
                Some(Err(_)) => ("✗", Style::default().fg(Color::Red)),
            };

            let name = info.name.to_string();
            let location = info
                .location
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("");

            let mut lines = vec![Line::from(vec![
                Span::styled(format!("{} ", icon), style.add_modifier(Modifier::BOLD)),
                Span::styled(name, style.add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!(" ({})", location),
                    Style::default().fg(Color::DarkGray),
                ),
            ])];

            // Add error message if there's an error
            if let Some(Err(err)) = status {
                lines.push(Line::from(vec![
                    Span::styled("  └─ ", Style::default().fg(Color::Red)),
                    Span::styled(format!("Error: {}", err), Style::default().fg(Color::Red)),
                ]));
            }

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(list_items).block(Block::default());

    list.render(layout[1], buf);
}
