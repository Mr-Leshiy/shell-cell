use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, StatefulWidget, Table, Widget},
};

use super::{App, LsState};

impl Widget for &App {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        match self {
            App::Loading { .. } => render_loading(area, buf),
            App::Ls(ls_state) => render_ls(ls_state, area, buf),
            App::Stopping(state) => render_stopping(&state.container_name, area, buf),
            App::ConfirmRemove(state) => {
                render_confirm_remove(state.selected_to_remove.name.as_str(), area, buf);
            },
            App::Removing(state) => render_removing(&state.container_name, area, buf),
            App::Exit => {},
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
            "Fetching 'Shell-Cell' containers info",
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

#[allow(clippy::indexing_slicing)]
fn render_stopping(
    container_name: &str,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = main_block();
    let inner = block.inner(area);
    Widget::render(block, area, buf);

    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .split(inner);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .split(vertical[1]);

    let stopping_text = vec![
        Line::from(vec![
            Span::styled(
                "Stopping",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("...", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("Stopping container '{container_name}'"),
            Style::default().fg(Color::Gray),
        )),
    ];

    let paragraph = Paragraph::new(stopping_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}

#[allow(clippy::indexing_slicing)]
fn render_confirm_remove(
    container_name: &str,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = main_block();
    let inner = block.inner(area);
    Widget::render(block, area, buf);

    let vertical = Layout::vertical([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .split(inner);

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
            format!("Remove container '{container_name}'?"),
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

    let paragraph = Paragraph::new(confirm_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}

#[allow(clippy::indexing_slicing)]
fn render_removing(
    container_name: &str,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = main_block();
    let inner = block.inner(area);
    Widget::render(block, area, buf);

    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .split(inner);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .split(vertical[1]);

    let removing_text = vec![
        Line::from(vec![
            Span::styled(
                "Removing",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("...", Style::default().fg(Color::Red)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("Removing container '{container_name}'"),
            Style::default().fg(Color::Gray),
        )),
    ];

    let paragraph = Paragraph::new(removing_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}

#[allow(clippy::indexing_slicing)]
fn render_ls(
    state: &LsState,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let block = main_block();
    let inner = block.inner(area);
    Widget::render(block, area, buf);

    let header_cells = [
        "Name",
        "Target",
        "Blueprint Location",
        "Created At",
        "Status",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Cyan)));
    let header = Row::new(header_cells)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1);

    let rows = state.containers.iter().map(|c| {
        let cells = vec![
            Cell::from(if c.orphan {
                format!("{} (orphan)", c.name)
            } else {
                c.name.to_string()
            }),
            Cell::from(
                c.target
                    .as_ref()
                    .map_or_else(|| "<empty>".to_string(), ToString::to_string),
            ),
            Cell::from(
                c.location
                    .as_ref()
                    .map_or_else(|| "<empty>".to_string(), |l| l.display().to_string()),
            ),
            Cell::from(c.created_at.map_or_else(
                || "<empty>".to_string(),
                |dt| dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            )),
            Cell::from(c.status.to_string()),
        ];
        Row::new(cells).height(1)
    });

    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(5),
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    StatefulWidget::render(table, inner, buf, &mut state.table_state.clone());

    if state.show_help {
        render_help_overlay(area, buf);
    }
}

#[allow(clippy::indexing_slicing)]
fn render_help_overlay(
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
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
            "Navigation",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  ↑ / ↓ / k / j     ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Move selection", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Actions",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  s                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Stop selected container", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  r                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Remove selected container",
                Style::default().fg(Color::White),
            ),
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
                "  Ctrl-C / Ctrl-D  ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Exit", Style::default().fg(Color::White)),
        ]),
    ];

    let paragraph = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .title_bottom("h / Esc: close this window")
            .title_alignment(HorizontalAlignment::Center)
            .border_style(Style::default().fg(Color::Cyan)).bg(Color::Black),
    );

    paragraph.render(horizontal[1], buf);
}

fn main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title("'Shell-Cell' Containers")
        .title_bottom("h: Help")
        .border_style(Style::new().light_magenta())
}
