#![allow(dead_code)]

mod def;
mod name;

use std::{collections::HashMap, path::Path};

use self::def::SCellDef;
use crate::scell_file::name::SCellName;

const SUPPORTED_VERSION: &str = "0.1";

#[derive(Debug, serde::Deserialize)]
pub struct SCellFile {
    pub version: String,
    #[serde(flatten)]
    pub cells: HashMap<SCellName, SCellDef>,
}

impl SCellFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file: std::fs::File = std::fs::File::open(path)?;
        let res: SCellFile = yaml_serde::from_reader(file)?;
        anyhow::ensure!(
            res.version == SUPPORTED_VERSION,
            "Currently supported version is {SUPPORTED_VERSION}, provided {}.",
            res.version
        );

        Ok(res)
    }
}
