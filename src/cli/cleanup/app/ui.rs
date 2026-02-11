use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

use super::{App, CleaningState};

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
        if let App::Cleaning(state) = self {
            render_cleaning(state, area, buf);
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
            "Fetching 'Shell-Cell' containers for cleaning",
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
fn render_cleaning(
    state: &CleaningState,
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
        Line::from("✓ All cleaned").style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Line::from(format!(
            "⟳ Cleaning 'Shell-Cell' containers and images... [{completed}/{total}]"
        ))
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
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
    Widget::render(progress_paragraph, layout[0], buf);

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
                Span::styled(
                    format!("{icon} {}", info.name),
                    style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        " ({}+{})",
                        info.location
                            .as_ref()
                            .map_or_else(|| "<empty>".to_string(), |l| l.display().to_string()),
                        info.target
                            .as_ref()
                            .map_or_else(|| "<empty>".to_string(), ToString::to_string)
                    ),
                    Style::default().fg(Color::DarkGray),
                ),
            ])];

            // Add error message if there's an error
            if let Some(Err(err)) = status {
                lines.push(
                    Line::from(format!("  └─ Error: {err}"))
                        .set_style(Style::default().fg(Color::Red)),
                );
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
        .title("Cleaning 'Shell-Cell' Containers and Images")
        .title_bottom("Ctrl-C or Ctrl-D: exit")
        .border_style(Style::new().light_magenta())
}
