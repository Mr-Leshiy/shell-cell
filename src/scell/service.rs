use crate::scell::{container::SCellContainer, image::SCellImage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    pub image: SCellImage,
    pub container: SCellContainer,
}
