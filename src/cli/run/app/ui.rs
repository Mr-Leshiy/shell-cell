use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

use super::App;
use crate::cli::run::app::LogType;

impl Widget for &App {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        if let App::Preparing(state) = self {
            let block = main_block();
            let inner = block.inner(area);
            Widget::render(block, area, buf);

            // Calculate how many log items can fit in the available height
            let available_height = inner.height as usize;
            let skip_amount = state.logs.len().saturating_sub(available_height);

            let logs = state
                .logs
                .iter()
                .enumerate()
                .map(|(i, (log, log_type))| {
                    let is_last = i == state.logs.len().saturating_sub(1) && i != 0;

                    let main_style = Style::default().add_modifier(Modifier::BOLD);

                    match log_type {
                        LogType::Main if is_last => {
                            ListItem::new(format!("{log} ...")).style(main_style.yellow())
                        },
                        LogType::Main => {
                            ListItem::new(format!("✓ {log}")).style(main_style.green())
                        },
                        LogType::SubLog => {
                            ListItem::new(format!("     {log}")).style(Style::default().cyan())
                        },
                    }
                })
                .skip(skip_amount);

            Widget::render(List::new(logs), inner, buf);
        }

        if let App::RunningPty(state) = self {
            let block = main_block();
            let inner = block.inner(area);
            Widget::render(block, area, buf);

            Widget::render(
                tui_term::widget::PseudoTerminal::new(state.parser.screen()),
                inner,
                buf,
            );
        }
    }
}

fn logs_list_iter<'a>(messages: &'a [&str]) -> impl Iterator<Item = ListItem<'a>> {
    messages.iter().enumerate().map(|(i, msg)| {
        let is_last = i == messages.len().saturating_sub(1) && i != 0;

        let style = Style::default().add_modifier(Modifier::BOLD);

        let line = if is_last {
            Line::from(vec![Span::styled(format!("{msg} ..."), style.yellow())])
        } else {
            Line::from(vec![Span::styled(format!("✓ {msg}"), style.green())])
        };
        ListItem::new(line)
    })
}

fn main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title("'Shell-Cell'")
        .title_bottom("Ctrl-C or Ctrl-D: exit")
        .border_style(Style::new().light_magenta())
}
