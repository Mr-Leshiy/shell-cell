mod app;
mod headless;

use crate::{
    buildkit::BuildKitD,
    cli::{ls::app::App, terminal::Terminal},
};

pub async fn ls(headless: bool) -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    if headless {
        return headless::ls(&buildkit).await;
    }
    let mut terminal = Terminal::new()?;
    let res = App::run(&buildkit, &mut terminal);
    ratatui::try_restore()?;
    res
}
