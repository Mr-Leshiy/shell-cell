use crate::{cli::Cli, error::UserError, scell::types::SCELL_FILE_NAME};

const BLUEPRINT: &[u8] = include_bytes!("template_scell.yml");

impl Cli {
    pub fn init(self) -> color_eyre::Result<()> {
        let path = self.scell_path.join(SCELL_FILE_NAME);
        if path.exists() {
            Err(UserError::wrap(format!(
                "`{SCELL_FILE_NAME}` already exists in `{}`",
                self.scell_path.display()
            )))?;
        }
        std::fs::write(&path, BLUEPRINT)?;
        println!("Created `{}`", path.display());
        Ok(())
    }
}
