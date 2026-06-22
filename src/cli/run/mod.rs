mod app;

use std::path::{Path, PathBuf};

use crate::{
    buildkit::BuildKitD,
    cli::{run::app::App, terminal::Terminal},
    scell::types::{SCELL_CUE_FILE_NAME, name::TargetName},
    scell_home_dir,
};

pub async fn run<P: AsRef<Path> + Send + 'static>(
    scell_path: P,
    target: Option<TargetName>,
    detach: bool,
    quiet: bool,
) -> color_eyre::Result<()> {
    let scell_path = resolve_scell_path(scell_path)?;
    let buildkit = BuildKitD::start().await?;
    let mut terminal = Terminal::new()?;
    let res = App::run(&buildkit, scell_path, target, detach, quiet, &mut terminal).await;
    ratatui::try_restore()?;
    res
}

/// Resolves the blueprint path provided by the user. When the path does not contain a
/// `scell.cue` file, falls back to the global blueprint located at
/// `scell_home_dir()/scell.cue` if it exists.
fn resolve_scell_path<P: AsRef<Path>>(scell_path: P) -> color_eyre::Result<PathBuf> {
    if let Ok(home_dir) = scell_home_dir()
        && home_dir.join(SCELL_CUE_FILE_NAME).is_file()
        && !scell_path.as_ref().join(SCELL_CUE_FILE_NAME).is_file()
    {
        Ok(home_dir)
    } else {
        Ok(scell_path.as_ref().to_path_buf())
    }
}
