use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, Widget},
};

use crate::cli::ls::{App, LsState};

impl Widget for &App {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        if let App::Ls(ls_state) = self {
            Widget::render(ls_state, area, buf);
        }
    }
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
                    .title("Shell-Cell Containers")
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
