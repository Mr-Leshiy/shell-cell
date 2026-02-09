use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};

use super::App;

const STEPS_LOGS: [&str; 4] = [
    "📝 Compiling Shell-Cell blueprint file",
    "⚙️ Building 'Shell-Cell' image",
    "📦 Starting 'Shell-Cell' container",
    "🚀 Starting 'Shell-Cell' session",
];

impl Widget for &App {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        // STEP 1
        if let App::Compiling(_) = self {
            const STEP: usize = 1;
            let block = main_block();
            let inner = block.inner(area);
            Widget::render(block, area, buf);
            let logs = logs_list(&STEPS_LOGS[..STEP]);
            Widget::render(List::new(logs), inner, buf);
        }
        // STEP 2
        if let App::BuildImage(state) = self {
            const STEP: usize = 2;
            let block = main_block();
            let inner = block.inner(area);
            Widget::render(block, area, buf);
            let mut logs = logs_list(&STEPS_LOGS[..STEP]);
            let logs_style = Style::default().cyan();
            logs.push(ListItem::new("  └─ ").style(logs_style.clone()));
            logs.extend(state.logs.iter().map(|log| {
                ListItem::new(format!("     {log}")).style(logs_style.clone())
            }));
            Widget::render(List::new(logs), inner, buf);
        }
        // STEP 3
        if let App::StartContainer(_) = self {
            const STEP: usize = 3;
            let block = main_block();
            let inner = block.inner(area);
            Widget::render(block, area, buf);
            let logs = logs_list(&STEPS_LOGS[..STEP]);
            Widget::render(List::new(logs), inner, buf);
        }
        // STEP 4
        if let App::StartSession(_) = self {
            const STEP: usize = 4;
            let block = main_block();
            let inner = block.inner(area);
            Widget::render(block, area, buf);
            let logs = logs_list(&STEPS_LOGS[..STEP]);
            Widget::render(List::new(logs), inner, buf);
        }
    }
}

fn logs_list<'a>(messages: &'a [&str]) -> Vec<ListItem<'a>> {
    messages
        .iter()
        .enumerate()
        .map(|(i, msg)| {
            let is_last = i == messages.len().saturating_sub(1) && i != 0;

            let style = Style::default().add_modifier(Modifier::BOLD);

            let line = if is_last {
                Line::from(vec![Span::styled(format!("{msg} ..."), style.yellow())])
            } else {
                Line::from(vec![Span::styled(format!("✓ {msg}"), style.green())])
            };
            ListItem::new(line)
        })
        .collect::<Vec<ListItem>>()
}

fn main_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title("'Shell-Cell'")
        .title_bottom("Ctrl-C or Ctrl-D: exit")
        .border_style(Style::new().light_magenta())
}
