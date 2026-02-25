mod ui;

use tui_scrollview::ScrollViewState;

use crate::{
    buildkit::{container_info::SCellContainerInfo, image_info::SCellImageInfo},
    cli::ls::app::ls::LsState,
};

pub trait ItemToInspect {
    type Data;

    fn inspect_data(&self) -> color_eyre::Result<Self::Data>;
}

/// Holds the state when the user is viewing the inspect overlay.
pub struct InspectState<Item: ItemToInspect> {
    /// The list state to restore when the overlay is dismissed.
    pub ls_state: LsState<Item>,
    data: Item::Data,
    scroll_state: ScrollViewState,
}

impl<Item: ItemToInspect> InspectState<Item> {
    pub fn new(
        ls_state: LsState<Item>,
        item: &Item,
    ) -> color_eyre::Result<Self> {
        Ok(Self {
            ls_state,
            data: item.inspect_data()?,
            scroll_state: ScrollViewState::new(),
        })
    }

    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll_state.scroll_up();
    }

    /// Scroll down
    pub fn scroll_down(&mut self) {
        self.scroll_state.scroll_down();
    }
}

type ContainerDescription = Option<String>;
type ImageDescription = Option<String>;
type ImageId = Option<String>;

impl ItemToInspect for SCellContainerInfo {
    type Data = (ContainerDescription, ImageId);

    fn inspect_data(&self) -> color_eyre::Result<Self::Data> {
        let description = self
            .container_desc
            .as_ref()
            .map(yaml_serde::to_string)
            .transpose()?
            .clone();
        let image_id = self.image_id.as_ref().map(ToString::to_string);
        Ok((description, image_id))
    }
}

impl ItemToInspect for SCellImageInfo {
    type Data = ImageDescription;

    fn inspect_data(&self) -> color_eyre::Result<Self::Data> {
        let description = self
            .desc
            .as_ref()
            .map(yaml_serde::to_string)
            .transpose()?
            .clone();
        Ok(description)
    }
}
