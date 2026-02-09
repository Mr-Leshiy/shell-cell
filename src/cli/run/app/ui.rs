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
        if matches!(
            self,
            App::Compiling(_) | App::BuildImage(_) | App::StartContainer(_) | App::StartSession(_)
        ) {
            let block = main_block();
            let inner = block.inner(area);
            Widget::render(block, area, buf);

            // Calculate how many log items can fit in the available height
            let available_height = inner.height as usize;

            let mut logs = Vec::new();
            // STEP 1
            if let App::Compiling(_) = self {
                const STEP: usize = 1;
                let skip_amount = STEP.saturating_sub(available_height);
                logs = logs_list_iter(&STEPS_LOGS[..STEP])
                    .skip(skip_amount)
                    .collect();
            }
            // STEP 2
            if let App::BuildImage(state) = self {
                const STEP: usize = 2;
                let logs_style = Style::default().cyan();
                let skip_amount = state
                    .logs
                    .len()
                    .saturating_add(STEP)
                    .saturating_sub(available_height);
                logs =
                    logs_list_iter(&STEPS_LOGS[..STEP])
                        .chain([ListItem::new("  └─ ").style(logs_style.clone())])
                        .chain(state.logs.iter().map(|log| {
                            ListItem::new(format!("     {log}")).style(logs_style.clone())
                        }))
                        .skip(skip_amount)
                        .collect();
            }
            // STEP 3
            if let App::StartContainer(_) = self {
                const STEP: usize = 3;
                let skip_amount = STEP.saturating_sub(available_height);
                logs = logs_list_iter(&STEPS_LOGS[..STEP])
                    .skip(skip_amount)
                    .collect();
            }
            // STEP 4
            if let App::StartSession(_) = self {
                const STEP: usize = 4;
                let skip_amount = STEP.saturating_sub(available_height);
                logs = logs_list_iter(&STEPS_LOGS[..STEP])
                    .skip(skip_amount)
                    .collect();
            }

            Widget::render(List::new(logs), inner, buf);
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
