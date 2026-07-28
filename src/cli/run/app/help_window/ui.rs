use ratatui::{layout::Rect, style::Color, widgets::Widget};

use crate::cli::{
    help_window_widget::{HelpEntry, HelpWindow},
    run::app::help_window::HelpWindowState,
};

/// Hint rendered on the bottom border.
const FOOTER: &str = "Ctrl-H / Esc: close this window";

const HELP_ENTRIES: &[HelpEntry] = &[
    HelpEntry::Title("Keyboard Shortcuts"),
    HelpEntry::Blank,
    HelpEntry::Section("Command mode (tmux-style)"),
    HelpEntry::Shortcut {
        key: "Ctrl-B",
        key_color: Color::Yellow,
        description: "Enter/Exit command mode",
    },
    HelpEntry::Shortcut {
        key: "d",
        key_color: Color::Red,
        description: "Detach and close the session",
    },
    HelpEntry::Shortcut {
        key: "↑ / ↓ / k / j",
        key_color: Color::Yellow,
        description: "Scroll the screen",
    },
    HelpEntry::Shortcut {
        key: "Esc / Ctrl-B",
        key_color: Color::Yellow,
        description: "Exit command mode",
    },
    HelpEntry::Blank,
    HelpEntry::Section("General"),
    HelpEntry::Shortcut {
        key: "Ctrl-H",
        key_color: Color::Yellow,
        description: "Open / close this help window",
    },
];

impl Widget for &mut HelpWindowState {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.0.render(area, buf);

        HelpWindow::new(HELP_ENTRIES)
            .footer(FOOTER)
            .render(area, buf);
    }
}
