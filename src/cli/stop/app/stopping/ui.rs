use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

use super::StoppingState;

#[allow(clippy::indexing_slicing)]
impl Widget for &StoppingState {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        // Calculate progress
        let total = self.containers.len();
        let completed = self.containers.values().filter(|v| v.is_some()).count();
        let is_done = completed == total;

        // Create header with progress
        let progress_text = if is_done {
            Line::from("✓ All containers stopped").style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Line::from(format!("⟳ Stopping containers... [{completed}/{total}]")).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        };

        let layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(area);

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
        let list_items: Vec<ListItem> = self
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
                        format!("{icon} {}", info.id),
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
}
