mod app;

use crate::{
    buildkit::BuildKitD,
    cli::{stop::app::App, terminal::Terminal},
};

pub async fn stop(cli: bool) -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let terminal = if cli { None } else { Some(Terminal::new()?) };
    App::run(&buildkit, terminal)
}
