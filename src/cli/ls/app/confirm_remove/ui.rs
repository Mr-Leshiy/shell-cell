use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::confirm_remove::ConfirmRemoveState,
};

impl Widget for &ConfirmRemoveState<SCellContainerInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let vertical = Layout::vertical([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(area);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(vertical[1]);

        let confirm_text = vec![
            Line::from(vec![Span::styled(
                "⚠ WARNING",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                format!("Remove container '{}'?", self.selected_to_remove.name),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "This will permanently delete:",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(Span::styled(
                "  • The container and all its state",
                Style::default().fg(Color::Gray),
            )),
            Line::from(Span::styled(
                "  • The associated image",
                Style::default().fg(Color::Gray),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "y",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to confirm, ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "n",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" or ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "Esc",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to cancel", Style::default().fg(Color::Gray)),
            ]),
        ];

        Widget::render(Clear, horizontal[1], buf);

        let paragraph = Paragraph::new(confirm_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
            )
            .centered();

        paragraph.render(horizontal[1], buf);
    }
}

impl Widget for &ConfirmRemoveState<SCellImageInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let vertical = Layout::vertical([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(area);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(vertical[1]);

        let confirm_text = vec![
            Line::from(vec![Span::styled(
                "⚠ WARNING",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(Span::styled(
                format!("Remove image '{}'?", self.selected_to_remove.name),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "This will permanently delete:",
                Style::default().fg(Color::Yellow),
            )),
            Line::from(Span::styled(
                "  • The image and all its state",
                Style::default().fg(Color::Gray),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "y",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to confirm, ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "n",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" or ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "Esc",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" to cancel", Style::default().fg(Color::Gray)),
            ]),
        ];

        Widget::render(Clear, horizontal[1], buf);

        let paragraph = Paragraph::new(confirm_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)),
            )
            .centered();

        paragraph.render(horizontal[1], buf);
    }
}
