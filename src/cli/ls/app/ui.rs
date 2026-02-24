use std::fmt::Display;

use ratatui::{
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use super::AppInner;
use crate::buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo};

const IMAGES_TITLE: &str = "Images";
const CONTAINERS_TITLE: &str = "Containers";

impl Widget for &AppInner<SCellContainerInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        match self {
            AppInner::Loading { .. } => render_loading(CONTAINERS_TITLE, area, buf),
            AppInner::Ls(ls_state) => {
                let inner = render_main_block(CONTAINERS_TITLE, area, buf);
                ls_state.render(inner, buf)
            },
            AppInner::Help(ls_state) => {
                let inner = render_main_block(CONTAINERS_TITLE, area, buf);
                ls_state.render(inner, buf);
                render_containers_help_overlay(area, buf);
            },
            AppInner::Stopping(state) => {
                let inner = render_main_block(CONTAINERS_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                render_stopping(CONTAINERS_TITLE, &state.for_stop.name, area, buf);
            },
            AppInner::ConfirmRemove(state) => {
                let inner = render_main_block(CONTAINERS_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                state.render(inner, buf);
            },
            AppInner::Removing(state) => {
                let inner = render_main_block(CONTAINERS_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                render_removing(CONTAINERS_TITLE, &state.for_removal.name, area, buf);
            },
            AppInner::Error(state) => {
                let inner = render_main_block(CONTAINERS_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                state.render(inner, buf);
            },
            AppInner::Inspect(state) => {
                let inner = render_main_block(CONTAINERS_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                state.render(inner, buf);
            },
            AppInner::Exit => {},
        }
    }
}

impl Widget for &AppInner<SCellImageInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        match self {
            AppInner::Loading { .. } => render_loading(IMAGES_TITLE, area, buf),
            AppInner::Ls(ls_state) => {
                let inner = render_main_block(IMAGES_TITLE, area, buf);
                ls_state.render(inner, buf);
            },
            AppInner::Help(ls_state) => {
                let inner = render_main_block(IMAGES_TITLE, area, buf);
                ls_state.render(inner, buf);
                render_images_help_overlay(area, buf);
            },
            AppInner::Stopping(state) => {
                let inner = render_main_block(IMAGES_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                render_stopping(IMAGES_TITLE, &state.for_stop.name, area, buf);
            },
            AppInner::ConfirmRemove(state) => {
                let inner = render_main_block(IMAGES_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                state.render(inner, buf);
            },
            AppInner::Removing(state) => {
                let inner = render_main_block(IMAGES_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                render_removing(IMAGES_TITLE, &state.for_removal.name, area, buf);
            },
            AppInner::Error(state) => {
                let inner = render_main_block(IMAGES_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                state.render(inner, buf);
            },
            AppInner::Inspect(state) => {
                let inner = render_main_block(IMAGES_TITLE, area, buf);
                state.ls_state.render(inner, buf);
                state.render(inner, buf);
            },
            AppInner::Exit => {},
        }
    }
}

#[allow(clippy::indexing_slicing)]
fn render_loading(
    item_title: impl Display,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let inner = render_main_block(item_title, area, buf);

    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Length(3),
        Constraint::Percentage(40),
    ])
    .split(inner);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .split(vertical[1]);

    let loading_text = vec![
        Line::from(vec![
            Span::styled(
                "Loading",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("...", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Fetching 'Shell-Cell' info",
            Style::default().fg(Color::Gray),
        )),
    ];

    Widget::render(Clear, horizontal[1], buf);

    let paragraph = Paragraph::new(loading_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}

#[allow(clippy::indexing_slicing)]
fn render_stopping(
    item_title: impl Display,
    item: impl Display,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let inner = render_main_block(item_title, area, buf);

    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .split(inner);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .split(vertical[1]);

    let stopping_text = vec![
        Line::from(vec![
            Span::styled(
                "Stopping",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("...", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("Stopping '{item}'"),
            Style::default().fg(Color::Gray),
        )),
    ];

    Widget::render(Clear, horizontal[1], buf);

    let paragraph = Paragraph::new(stopping_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}

#[allow(clippy::indexing_slicing)]
fn render_removing(
    item_title: impl Display,
    item: impl Display,
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let inner = render_main_block(item_title, area, buf);

    let vertical = Layout::vertical([
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .split(inner);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(60),
        Constraint::Percentage(20),
    ])
    .split(vertical[1]);

    let removing_text = vec![
        Line::from(vec![
            Span::styled(
                "Removing",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("...", Style::default().fg(Color::Red)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            format!("Removing '{item}'",),
            Style::default().fg(Color::Gray),
        )),
    ];

    Widget::render(Clear, horizontal[1], buf);

    let paragraph = Paragraph::new(removing_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .centered();

    paragraph.render(horizontal[1], buf);
}

#[allow(clippy::indexing_slicing)]
fn render_containers_help_overlay(
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let vertical = Layout::vertical([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ])
    .split(vertical[1]);

    let help_text = vec![
        Line::from(vec![Span::styled(
            "Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  ↑ / ↓ / k / j     ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Move selection", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Actions",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  q                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Switch to the images view",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  i                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Show container defintion",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  s                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Stop selected container", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  r                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Remove selected container",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  Ctrl-C / Ctrl-D  ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Exit", Style::default().fg(Color::White)),
        ]),
    ];

    Widget::render(Clear, horizontal[1], buf);

    let paragraph = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .title_bottom("h / Esc: close this window")
            .title_alignment(HorizontalAlignment::Center)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    paragraph.render(horizontal[1], buf);
}

#[allow(clippy::indexing_slicing)]
fn render_images_help_overlay(
    area: Rect,
    buf: &mut ratatui::prelude::Buffer,
) {
    let vertical = Layout::vertical([
        Constraint::Percentage(15),
        Constraint::Percentage(70),
        Constraint::Percentage(15),
    ])
    .split(area);

    let horizontal = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ])
    .split(vertical[1]);

    let help_text = vec![
        Line::from(vec![Span::styled(
            "Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  ↑ / ↓ / k / j     ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Move selection", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Actions",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  q                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Switch to the containers view",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  i                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Show image defintion", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  r                 ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Remove selected image", Style::default().fg(Color::White)),
        ]),
        Line::from("                    (can't remove image, which is in use)"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                "  Ctrl-C / Ctrl-D  ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Exit", Style::default().fg(Color::White)),
        ]),
    ];

    Widget::render(Clear, horizontal[1], buf);

    let paragraph = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .title_bottom("h / Esc: close this window")
            .title_alignment(HorizontalAlignment::Center)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    paragraph.render(horizontal[1], buf);
}

fn render_main_block(
    items_title: impl Display,
    area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
) -> Rect {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("'Shell-Cell' {items_title}"))
        .title_bottom("h: Help")
        .border_style(Style::new().light_magenta());
    let inner = block.inner(area);
    block.render(area, buf);
    inner
}
