use std::{fmt::Display, hash::Hasher, str::FromStr};

use hex::ToHex;

const ID_PREFIX: &str = "scell-";

/// A 'Shell-Cell' ID, which is hex encoded hash of the corresponding 'Shell-Cell'
/// object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SCellId(String);

impl SCellId {
    pub fn new(
        to_hash: impl FnOnce(&mut metrohash::MetroHash64) -> color_eyre::Result<()>
    ) -> color_eyre::Result<Self> {
        let mut hasher = metrohash::MetroHash64::new();
        to_hash(&mut hasher)?;
        let hex_hash: String = hasher.finish().to_be_bytes().encode_hex();
        Ok(Self(format!("{ID_PREFIX}{hex_hash}")))
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
