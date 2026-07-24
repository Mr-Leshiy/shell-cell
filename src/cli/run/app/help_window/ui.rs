use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::cli::run::app::help_window::HelpWindowState;

impl Widget for &mut HelpWindowState {
    #[allow(clippy::indexing_slicing, clippy::too_many_lines)]
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.0.render(area, buf);

        let vertical = Layout::vertical([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(area);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(vertical[1]);

        let help_text = vec![
            Line::from(vec![Span::styled(
                "Keyboard Shortcuts",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Command mode (tmux-style)",
                Style::default()
                    .fg(Color::LightMagenta)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled(
                    "        Ctrl-B        ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Enter command mode (shown in status bar)",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "          d           ",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Detach and close the session",
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "    ↑ / ↓ / k / j      ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("Scroll the screen", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled(
                    "         Esc          ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("Exit command mode", Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "General",
                Style::default()
                    .fg(Color::LightMagenta)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled(
                    "        Ctrl-H        ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Open / close this help window",
                    Style::default().fg(Color::White),
                ),
            ]),
        ];

        Clear.render(horizontal[1], buf);

        let paragraph = Paragraph::new(help_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .title_bottom(" Ctrl-H / Esc: close this window")
                .title_alignment(HorizontalAlignment::Center)
                .border_style(Style::default().fg(Color::Cyan)),
        );

        paragraph.render(horizontal[1], buf);
    }
}
