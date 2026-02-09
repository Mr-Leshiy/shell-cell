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
        match self {
            App::Loading(_) => {
                render_loading(area, buf);
            },
            App::Ls(ls_state) => {
                Widget::render(ls_state, area, buf);
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
        .title("'Shell-Cell' Containers")
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

impl Widget for &LsState {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let header_cells = ["Name", "Blueprint Location", "Created At", "ID", "Status"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Cyan)));
        let header = Row::new(header_cells)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .height(1);

        let rows = self.containers.iter().map(|c| {
            let cells = vec![
                Cell::from(c.name.to_string()),
                Cell::from(format!("{}", c.location.display())),
                Cell::from(
                    c.created_at
                        .to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
                ),
                Cell::from(c.container_name.clone()),
                Cell::from(c.status.to_string()),
            ];
            Row::new(cells).height(1)
        });

        let widths = [
            Constraint::Length(15),
            Constraint::Min(30),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("'Shell-Cell' Containers")
                    .title_bottom("↑↓: navigate, Ctrl-C: exit")
                    .border_style(Style::new().light_green()),
            )
            .row_highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        StatefulWidget::render(table, area, buf, &mut self.table_state.clone());
    }
}
