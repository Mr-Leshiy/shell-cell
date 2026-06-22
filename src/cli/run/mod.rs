mod app;

use std::path::Path;

use crate::{
    buildkit::BuildKitD,
    cli::{run::app::App, terminal::Terminal},
    scell::types::name::TargetName,
    scell_home_dir,
};

pub async fn run<P: AsRef<Path> + Send + 'static>(
    scell_path: P,
    target: Option<TargetName>,
    detach: bool,
    quiet: bool,
    global: bool,
) -> color_eyre::Result<()> {
    // When `--global` is set, the global blueprint in the Shell-Cell home directory is used,
    // ignoring any local `scell.cue`. Otherwise the path provided by the user is used as is.
    let scell_path = if global {
        scell_home_dir()?
    } else {
        scell_path.as_ref().to_path_buf()
    };
    let buildkit = BuildKitD::start().await?;
    let mut terminal = Terminal::new()?;
    let res = App::run(&buildkit, scell_path, target, detach, quiet, &mut terminal).await;
    ratatui::try_restore()?;
    res
}
