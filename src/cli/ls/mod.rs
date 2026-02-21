mod app;

use crate::{buildkit::BuildKitD, cli::ls::app::App};

pub async fn ls() -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let mut terminal = ratatui::try_init()?;
    let res = App::run(&buildkit, &mut terminal);
    ratatui::try_restore()?;
    res
}
