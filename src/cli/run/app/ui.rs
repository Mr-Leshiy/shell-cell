use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect, Size},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

use crate::{
    cli::run::app::{App, LogType},
    pty::Pty,
};

impl Widget for &mut App {
    #[allow(clippy::indexing_slicing)]
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().light_magenta());

        if let App::Preparing(state) = self {
            let block = block
                .title("'Shell-Cell'")
                .title_bottom("Ctrl-C or Ctrl-D: exit");
            let inner = block.inner(area);
            block.render(area, buf);

            let area_width = inner.width as usize;

            let logs = state
                .logs
                .iter()
                .flat_map(|log| {
                    log.0
                        .lines()
                        .flat_map(|line| {
                            // Splitting each line to lines, if they exceed the `area_width`.
                            // Adding some extra identation, so the text would always fits, even
                            // while adding some extra prefixes/suffixes etc. for different
                            // `LogType`s
                            let area_width = area_width.saturating_sub(5);

                            line.chars()
                                .chunks(area_width)
                                .into_iter()
                                .map(|chunk| (chunk.collect::<String>(), log.1))
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let logs_len = logs.len();
            let logs_height = u16::try_from(logs_len).unwrap_or(u16::MAX);
            let skip_amount = logs_len.saturating_sub(logs_height.into());

            let logs = logs
                .iter()
                .enumerate()
                .map(|(i, (log, log_type))| {
                    let is_last = i == logs_len.saturating_sub(1) && i != 0;
                    let main_style = Style::default().add_modifier(Modifier::BOLD);
                    match log_type {
                        // the main line is always must be a one liner
                        LogType::Main if is_last => {
                            ListItem::new(format!("{log} ...")).style(main_style.yellow())
                        },
                        LogType::Main => {
                            ListItem::new(format!("✓ {log}")).style(main_style.green())
                        },
                        LogType::MainError => {
                            ListItem::new(format!("   {log}")).style(main_style.red())
                        },
                        LogType::MainInfo => ListItem::new(log.as_str()).style(main_style.blue()),
                        LogType::SubLog => {
                            ListItem::new(format!("     {log}")).style(Style::default().cyan())
                        },
                    }
                })
                .skip(skip_amount);

            let mut scroll_view = ScrollView::new(Size::new(inner.width, logs_height))
                .vertical_scrollbar_visibility(ScrollbarVisibility::Automatic)
                .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

            scroll_view.render_widget(List::new(logs), Rect::new(0, 0, inner.width, logs_height));
            scroll_view.render(inner, buf, &mut state.scroll_view_state);
        } else if let App::RunningPty(state) = self {
            let block = block
                .title(format!("'Shell-Cell' {}", state.container_id))
                .title_bottom("Ctrl-D: exit");
            let inner = block.inner(area);
            block.render(area, buf);
            state.pty.render(inner, buf);
        } else if let App::Finished = self {
            let block = block.title("'Shell-Cell'");
            let inner = block.inner(area);
            block.render(area, buf);

            // Create a centered area using Layout
            let vertical_layout = Layout::vertical([
                Constraint::Percentage(50),
                Constraint::Length(2),
                Constraint::Percentage(50),
            ])
            .split(inner);

            let text = vec![
                Line::from(Span::styled(
                    "Finished 'Shell-Cell' session",
                    Style::default().add_modifier(Modifier::BOLD).green(),
                )),
                Line::from(Span::styled(
                    "<Press any key to exit>",
                    Style::default().cyan(),
                )),
            ];

            let paragraph = Paragraph::new(text).alignment(Alignment::Center);
            paragraph.render(vertical_layout[1], buf);
        }
    }
}

impl Widget for &mut Pty {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        // set the proper size for the terminal screen
        self.set_size(area.height, area.width);
        tui_term::widget::PseudoTerminal::new(self.screen()).render(area, buf);
    }
}
