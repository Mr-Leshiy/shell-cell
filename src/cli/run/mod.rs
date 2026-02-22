mod app;

use std::path::Path;

use crate::{
    buildkit::BuildKitD,
    cli::run::app::App,
    scell::types::{args::StartupArguments, name::TargetName},
};

pub async fn run<P: AsRef<Path> + Send + 'static>(
    scell_path: P,
    target: Option<TargetName>,
    args: StartupArguments,
) -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let mut terminal = ratatui::try_init()?;
    let res = App::run(&buildkit, scell_path, target, args, &mut terminal);
    ratatui::try_restore()?;
    res
}
