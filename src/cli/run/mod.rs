mod app;

use std::path::Path;

use crate::{buildkit::BuildKitD, cli::run::app::App, scell::types::name::TargetName};

pub async fn run<P: AsRef<Path> + Send + 'static>(
    scell_path: P,
    target: Option<TargetName>,
) -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let mut terminal = ratatui::try_init()?;
    let res = App::run(&buildkit, scell_path, target, &mut terminal);
    ratatui::try_restore()?;
    res
}
