use itertools::Itertools;
use ratatui::{
    layout::{Rect, Size},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, StatefulWidget, Widget},
};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

use crate::cli::run::app::preparing::{LogType, PreparingState};

impl Widget for &mut PreparingState {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().light_magenta())
            .title("'Shell-Cell'")
            .title_bottom("Ctrl-C or Ctrl-D: exit");
        let inner = block.inner(area);
        block.render(area, buf);

        let area_width = inner.width as usize;

        let logs = self
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
                    LogType::Main => ListItem::new(format!("✓ {log}")).style(main_style.green()),
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
        scroll_view.render(inner, buf, &mut self.scroll_view_state);
    }
}
