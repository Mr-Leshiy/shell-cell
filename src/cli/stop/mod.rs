mod app;

use crate::{buildkit::BuildKitD, cli::stop::app::App};

pub async fn stop() -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let mut terminal = ratatui::try_init()?;
    let res = App::run(&buildkit, &mut terminal);
    ratatui::try_restore()?;
    res
}
