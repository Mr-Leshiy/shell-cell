use std::{fmt::Display, str::FromStr};

use crate::scell::SCell;

const NAME_PREFIX: &str = "scell-";

/// A 'Shell-Cell' name, which is hex encoded hash of the corresponding 'Shell-Cell'
/// object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellName(String);

impl SCellName {
    pub fn new(scell: &SCell) -> color_eyre::Result<Self> {
        Ok(Self(format!("{NAME_PREFIX}{}", scell.hex_hash()?)))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for SCellName {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for SCellName {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        color_eyre::eyre::ensure!(
            s.contains(NAME_PREFIX),
            "'Shell-Cell' name must have a prefix {NAME_PREFIX}"
        );
        Ok(Self(s.to_string()))
    }
}
