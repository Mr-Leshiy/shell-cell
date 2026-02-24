use std::{fmt::Display, str::FromStr};

use crate::scell::SCell;

const ID_PREFIX: &str = "scell-";

/// A 'Shell-Cell' ID, which is hex encoded hash of the corresponding 'Shell-Cell'
/// object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellId(String);

impl SCellId {
    pub fn new(scell: &SCell) -> color_eyre::Result<Self> {
        Ok(Self(format!("{ID_PREFIX}{}", scell.hex_hash()?)))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for SCellId {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for SCellId {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        color_eyre::eyre::ensure!(
            s.contains(ID_PREFIX),
            "'Shell-Cell' ID must have a prefix {ID_PREFIX}"
        );
        Ok(Self(s.to_string()))
    }
}
