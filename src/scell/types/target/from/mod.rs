pub mod image;
pub mod target_ref;

use std::path::PathBuf;

use serde::Deserialize;

use crate::scell::types::target::from::{image::ImageDef, target_ref::TargetRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum FromStmt {
    #[serde(rename = "from")]
    Target(TargetRef),
    #[serde(rename = "from_image")]
    Image(ImageDef),
    #[serde(rename = "from_docker")]
    Docker(PathBuf),
}
