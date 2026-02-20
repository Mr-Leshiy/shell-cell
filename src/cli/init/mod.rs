use std::fs;

use crate::{cli::Cli, error::{UserError, WrapUserError}};

const BLUEPRINT_FILENAME: &str = "scell.yml";

const BLUEPRINT: &str = include_str!("template_scell.yml");

impl Cli {
    pub fn init(self) -> color_eyre::Result<()> {
        let path = self.scell_path.join(BLUEPRINT_FILENAME);

        if path.exists() {
            return Err(UserError::wrap(format!(
                "`{BLUEPRINT_FILENAME}` already exists in `{}`",
                self.scell_path.display()
            ))
            .into());
        }

        fs::write(&path, BLUEPRINT)
            .wrap_user_err(format!("Failed to create `{}`", path.display()))?;

        println!("Created `{}`", path.display());

        Ok(())
    }
}
