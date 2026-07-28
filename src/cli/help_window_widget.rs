//! Reusable, centered "Keyboard Shortcuts" popup shared by the TUI apps.
//!
//! Callers only describe *what* the help window contains, as a slice of
//! [`HelpEntry`]; the widget takes care of sizing the popup, centering it in
//! the available area and aligning the key / description columns.

use ratatui::{
    layout::{Constraint, Flex, HorizontalAlignment, Layout, Rect},
    prelude::Buffer,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Widget},
};

/// Horizontal gap between the key column and the description column.
const COLUMN_GAP: u16 = 2;
/// Free space kept between the help text and the window borders.
const WINDOW_PADDING: u16 = 2;
/// Space taken by the left and right borders.
const BORDERS_WIDTH: u16 = 2;
/// Space taken by the top and bottom borders.
const BORDERS_HEIGHT: u16 = 2;

/// A single line of a help window.
pub enum HelpEntry {
    /// Empty spacer line.
    Blank,
    /// Title of the whole window, centered.
    Title(&'static str),
    /// Header of a group of shortcuts, centered.
    Section(&'static str),
    /// A key binding and what it does.
    Shortcut {
        /// Key (or key combination) to press.
        key: &'static str,
        /// Color of the key, used to highlight destructive actions.
        key_color: Color,
        /// What the key does.
        description: &'static str,
    },
    /// Remark attached to the shortcut above, aligned with the descriptions.
    Note(&'static str),
}

impl HelpEntry {
    /// Width of the key, for entries that have one.
    fn key_width(&self) -> Option<u16> {
        match self {
            Self::Shortcut { key, .. } => Some(text_width(key)),
            _ => None,
        }
    }

    /// Width of the description column content, for entries that have one.
    fn description_width(&self) -> Option<u16> {
        match self {
            Self::Shortcut { description, .. } => Some(text_width(description)),
            Self::Note(text) => Some(text_width(text)),
            _ => None,
        }
    }

    /// Width of a standalone (centered) line, for entries that have one.
    fn centered_width(&self) -> Option<u16> {
        match self {
            Self::Title(text) | Self::Section(text) => Some(text_width(text)),
            _ => None,
        }
    }
}

/// Centered popup listing keyboard shortcuts.
pub struct HelpWindow<'a> {
    /// Lines of the window, in render order.
    entries: &'a [HelpEntry],
    /// Text of the top border.
    title: &'a str,
    /// Text of the bottom border, usually telling how to close the window.
    footer: &'a str,
}

impl<'a> HelpWindow<'a> {
    /// Creates a help window rendering the given entries.
    pub const fn new(entries: &'a [HelpEntry]) -> Self {
        Self {
            entries,
            title: "Help",
            footer: "",
        }
    }

    /// Overrides the text rendered on the top border.
    #[expect(dead_code, reason = "part of the widget API")]
    pub const fn title(
        mut self,
        title: &'a str,
    ) -> Self {
        self.title = title;
        self
    }

    /// Sets the hint rendered on the bottom border.
    pub const fn footer(
        mut self,
        footer: &'a str,
    ) -> Self {
        self.footer = footer;
        self
    }

    /// Width of the key column, shared by every shortcut row.
    fn key_width(&self) -> u16 {
        self.entries
            .iter()
            .filter_map(HelpEntry::key_width)
            .max()
            .unwrap_or(0)
    }

    /// Width of the description column, shared by every shortcut row.
    fn description_width(&self) -> u16 {
        self.entries
            .iter()
            .filter_map(HelpEntry::description_width)
            .max()
            .unwrap_or(0)
    }

    /// Size of the popup, borders included.
    fn window_size(&self) -> (u16, u16) {
        // Widest line that is centered on its own, borders included.
        let centered_width = self
            .entries
            .iter()
            .filter_map(HelpEntry::centered_width)
            .chain([
                text_width(self.title).saturating_add(BORDERS_WIDTH),
                text_width(self.footer).saturating_add(BORDERS_WIDTH),
            ])
            .max()
            .unwrap_or(0);

        let content_width = self
            .key_width()
            .saturating_add(COLUMN_GAP)
            .saturating_add(self.description_width())
            .max(centered_width);

        let width = content_width
            .saturating_add(WINDOW_PADDING.saturating_mul(2))
            .saturating_add(BORDERS_WIDTH);
        let height = u16::try_from(self.entries.len())
            .unwrap_or(u16::MAX)
            .saturating_add(BORDERS_HEIGHT);

        (width, height)
    }
}

impl Widget for HelpWindow<'_> {
    fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
    ) where
        Self: Sized,
    {
        let key_width = self.key_width();
        let description_width = self.description_width();
        let (width, height) = self.window_size();

        // Center the window itself inside the available area.
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        let [window] = Layout::vertical([Constraint::Length(height)])
            .flex(Flex::Center)
            .areas(area);

        Clear.render(window, buf);

        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.title))
            .title_alignment(HorizontalAlignment::Center)
            .border_style(Style::default().fg(Color::Cyan));
        if !self.footer.is_empty() {
            block = block.title_bottom(format!(" {} ", self.footer));
        }

        let inner = block.inner(window);
        block.render(window, buf);

        // One row per entry.
        let rows = Layout::vertical(vec![Constraint::Length(1); self.entries.len()]).split(inner);

        for (entry, row) in self.entries.iter().zip(rows.iter()) {
            match entry {
                HelpEntry::Blank => {},
                HelpEntry::Title(text) => {
                    Line::styled(
                        *text,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .centered()
                    .render(*row, buf)
                },
                HelpEntry::Section(text) => {
                    Line::styled(
                        *text,
                        Style::default()
                            .fg(Color::LightMagenta)
                            .add_modifier(Modifier::BOLD),
                    )
                    .centered()
                    .render(*row, buf)
                },
                HelpEntry::Shortcut {
                    key,
                    key_color,
                    description,
                } => {
                    let (key_area, description_area) = columns(*row, key_width, description_width);

                    Line::styled(
                        *key,
                        Style::default().fg(*key_color).add_modifier(Modifier::BOLD),
                    )
                    .centered()
                    .render(key_area, buf);

                    Line::styled(*description, Style::default().fg(Color::White))
                        .left_aligned()
                        .render(description_area, buf);
                },
                HelpEntry::Note(text) => {
                    let (_, description_area) = columns(*row, key_width, description_width);

                    Line::styled(*text, Style::default().fg(Color::DarkGray))
                        .left_aligned()
                        .render(description_area, buf);
                },
            }
        }
    }
}

/// Splits a row into the key and the description columns, keeping the pair
/// centered so that every row shares the same column edges.
fn columns(
    row: Rect,
    key_width: u16,
    description_width: u16,
) -> (Rect, Rect) {
    let [key_area, _, description_area] = Layout::horizontal([
        Constraint::Length(key_width),
        Constraint::Length(COLUMN_GAP),
        Constraint::Length(description_width),
    ])
    .flex(Flex::Center)
    .areas(row);

    (key_area, description_area)
}

/// Rendered width of `text`, saturating at [`u16::MAX`].
fn text_width(text: &str) -> u16 {
    u16::try_from(Span::raw(text).width()).unwrap_or(u16::MAX)
}
