use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

use super::App;
use crate::cli::run::app::{LogType, vt::TerminalEmulator};

impl Widget for &mut App {
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
            Widget::render(block, area, buf);

            let logs = state
                .logs
                .iter()
                .flat_map(|log| log.0.lines().map(|line| (line.to_string(), log.1)))
                .collect::<Vec<_>>();
            let logs_len = logs.len();
            // Calculate how many log items can fit in the available height
            let available_height = inner.height as usize;
            let skip_amount = logs_len.saturating_sub(available_height);

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

            Widget::render(List::new(logs), inner, buf);
        } else if let App::RunningPty(state) = self {
            let block = block
                .title(format!("'Shell-Cell' {}", state.scell_name))
                .title_bottom("Ctrl-D: exit");
            let inner = block.inner(area);
            Widget::render(block, area, buf);
            Widget::render(&mut state.term, inner, buf);
        } else if let App::Finished = self {
            let block = block.title("'Shell-Cell'");
            let inner = block.inner(area);
            Widget::render(block, area, buf);

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
            #[allow(clippy::indexing_slicing)]
            Widget::render(paragraph, vertical_layout[1], buf);
        }
    }
}

impl Widget for &mut TerminalEmulator {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        // set the proper size for the terminal screen
        self.set_size(area.height, area.width);
        Widget::render(
            tui_term::widget::PseudoTerminal::new(self.screen()),
            area,
            buf,
        );
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
