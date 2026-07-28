use ratatui::{layout::Rect, style::Color, widgets::Widget};

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::{
        help_window_widget::{HelpEntry, HelpWindow},
        ls::app::help_window::HelpWindowState,
    },
};

/// Hint rendered on the bottom border.
const FOOTER: &str = "h / Esc: close this window";

const CONTAINER_HELP_ENTRIES: &[HelpEntry] = &[
    HelpEntry::Title("Keyboard Shortcuts"),
    HelpEntry::Blank,
    HelpEntry::Section("Navigation"),
    HelpEntry::Shortcut {
        key: "↑ / ↓ / k / j",
        key_color: Color::Yellow,
        description: "Move selection",
    },
    HelpEntry::Blank,
    HelpEntry::Section("Actions"),
    HelpEntry::Shortcut {
        key: "q",
        key_color: Color::Yellow,
        description: "Switch to the images view",
    },
    HelpEntry::Shortcut {
        key: "i",
        key_color: Color::Yellow,
        description: "Inspect container definition",
    },
    HelpEntry::Shortcut {
        key: "s",
        key_color: Color::Yellow,
        description: "Stop selected container",
    },
    HelpEntry::Shortcut {
        key: "r",
        key_color: Color::Yellow,
        description: "Remove selected container",
    },
    HelpEntry::Blank,
    HelpEntry::Section("General"),
    HelpEntry::Shortcut {
        key: "Ctrl-C / Ctrl-D",
        key_color: Color::Red,
        description: "Exit",
    },
];

const IMAGE_HELP_ENTRIES: &[HelpEntry] = &[
    HelpEntry::Title("Keyboard Shortcuts"),
    HelpEntry::Blank,
    HelpEntry::Section("Navigation"),
    HelpEntry::Shortcut {
        key: "↑ / ↓ / k / j",
        key_color: Color::Yellow,
        description: "Move selection",
    },
    HelpEntry::Blank,
    HelpEntry::Section("Actions"),
    HelpEntry::Shortcut {
        key: "q",
        key_color: Color::Yellow,
        description: "Switch to the containers view",
    },
    HelpEntry::Shortcut {
        key: "i",
        key_color: Color::Yellow,
        description: "Inspect image definition",
    },
    HelpEntry::Shortcut {
        key: "r",
        key_color: Color::Yellow,
        description: "Remove selected image",
    },
    HelpEntry::Note("(can't remove image, which is in use)"),
    HelpEntry::Blank,
    HelpEntry::Section("General"),
    HelpEntry::Shortcut {
        key: "Ctrl-C / Ctrl-D",
        key_color: Color::Red,
        description: "Exit",
    },
];

impl Widget for &HelpWindowState<SCellContainerInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);

        HelpWindow::new(CONTAINER_HELP_ENTRIES)
            .footer(FOOTER)
            .render(area, buf);
    }
}

impl Widget for &HelpWindowState<SCellImageInfo> {
    fn render(
        self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        self.ls_state.render(area, buf);

        HelpWindow::new(IMAGE_HELP_ENTRIES)
            .footer(FOOTER)
            .render(area, buf);
    }
}
