use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Cell, Row, StatefulWidget, Table, Widget},
};

use super::LsState;
use crate::buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo};

impl Widget for &LsState<SCellContainerInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
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

        let rows = self.items.iter().map(|c| {
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

        StatefulWidget::render(table, area, buf, &mut self.table_state.clone());
    }
}

impl Widget for &LsState<SCellImageInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
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

        let rows = self.items.iter().map(|c| {
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
                Cell::from(if c.in_use {
                    "in use".to_string()
                } else {
                    String::new()
                }),
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

        StatefulWidget::render(table, area, buf, &mut self.table_state.clone());
    }
}
