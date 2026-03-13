use std::path::Path;

use crate::{error::UserError, scell::types::SCELL_CUE_FILE_NAME};

const BLUEPRINT: &[u8] = include_bytes!("template_scell.cue");

pub fn init<P: AsRef<Path>>(dir: P) -> color_eyre::Result<()> {
    let path = dir.as_ref().join(SCELL_CUE_FILE_NAME);
    if path.exists() {
        Err(UserError::wrap(format!(
            "`{SCELL_CUE_FILE_NAME}` already exists in `{}`",
            dir.as_ref().display()
        )))?;
    }
    std::fs::write(&path, BLUEPRINT)?;
    println!("Created `{}`", path.display());
    Ok(())
}
