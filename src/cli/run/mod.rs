mod app;

use std::path::Path;

use crate::{
    buildkit::BuildKitD,
    cli::{run::app::App, terminal::Terminal},
    scell::types::name::TargetName,
};

pub async fn run<P: AsRef<Path> + Send + 'static>(
    scell_path: P,
    target: Option<TargetName>,
    detach: bool,
    quiet: bool,
) -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    if detach {
        // Headless path: no TTY/raw-mode/Terminal needed when we don't
        // attach a shell session. Drains build logs to stderr instead.
        return App::run_headless(&buildkit, scell_path, target, quiet).await;
    }
    let mut terminal = Terminal::new()?;
    let res = App::run(&buildkit, scell_path, target, detach, quiet, &mut terminal).await;
    ratatui::try_restore()?;
    res
}
