use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
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
        if let App::Loading { .. } = self {
            render_loading(area, buf);
        }
        if let App::Ls(ls_state) = self {
            render_ls(ls_state, area, buf);
        }
        if let App::Stopping(state) = self {
            render_stopping(&state.container_name, area, buf)
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
        Constraint::Length(3),
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
}

fn main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title("'Shell-Cell' Containers")
        .title_bottom("↑↓: navigate, s: stop, Ctrl-C or Ctrl-D: exit")
        .border_style(Style::new().light_magenta())
}
