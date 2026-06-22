use std::path::PathBuf;

use crate::{error::UserError, scell::types::SCELL_CUE_FILE_NAME, scell_home_dir};

const BLUEPRINT: &[u8] = include_bytes!("template_scell.cue");

pub fn init(
    dir: PathBuf,
    global: bool,
) -> color_eyre::Result<()> {
    let dir = if global { scell_home_dir()? } else { dir };
    let path = dir.join(SCELL_CUE_FILE_NAME);
    if path.exists() {
        Err(UserError::wrap(format!(
            "`{SCELL_CUE_FILE_NAME}` already exists in `{}`",
            dir.display()
        )))?;
    }
    std::fs::write(&path, BLUEPRINT)?;
    println!("Created `{}`", path.display());
    Ok(())
}
