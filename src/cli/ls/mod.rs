mod app;

use crate::{
    buildkit::BuildKitD,
    cli::{ls::app::App, terminal::Terminal},
};

pub async fn ls() -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let mut terminal = Terminal::new()?;
    let res = App::run(&buildkit, &mut terminal);
    ratatui::try_restore()?;
    res
}
