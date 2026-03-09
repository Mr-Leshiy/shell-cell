use std::fmt::Display;

use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Widget},
};

use super::AppInner;
use crate::buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo};

const CONTAINERS_TITLE: &str = "Containers";
const IMAGES_TITLE: &str = "Images";

impl Widget for &mut AppInner<SCellContainerInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let inner = render_main_block(CONTAINERS_TITLE, area, buf);
        match self {
            AppInner::Loading(state) => state.render(inner, buf),
            AppInner::Ls(ls_state) => ls_state.render(inner, buf),
            AppInner::HelpWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Stopping(state) => {
                state.render(inner, buf);
            },
            AppInner::ConfirmRemove(state) => {
                state.render(inner, buf);
            },
            AppInner::Removing(state) => {
                state.render(inner, buf);
            },
            AppInner::ErrorWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Inspect(state) => {
                state.render(inner, buf);
            },
            AppInner::Exit => {},
        }
    }
}

impl Widget for &mut AppInner<SCellImageInfo> {
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) where
        Self: Sized,
    {
        let inner = render_main_block(IMAGES_TITLE, area, buf);
        match self {
            AppInner::Loading(state) => state.render(inner, buf),
            AppInner::Ls(ls_state) => {
                ls_state.render(inner, buf);
            },
            AppInner::HelpWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Stopping(state) => {
                state.render(inner, buf);
            },
            AppInner::ConfirmRemove(state) => {
                state.render(inner, buf);
            },
            AppInner::Removing(state) => {
                state.render(inner, buf);
            },
            AppInner::ErrorWindow(state) => {
                state.render(inner, buf);
            },
            AppInner::Inspect(state) => {
                state.render(inner, buf);
            },
            AppInner::Exit => {},
        }
    }
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
