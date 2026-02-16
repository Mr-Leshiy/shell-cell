mod app;

use crate::{
    buildkit::BuildKitD,
    cli::{Cli, stop::app::App},
};

impl Cli {
    pub async fn stop(self) -> color_eyre::Result<()> {
        let buildkit = BuildKitD::start().await?;
        let mut terminal = ratatui::try_init()?;
        let res = App::run(&buildkit, &mut terminal);
        ratatui::try_restore()?;
        res
    }
}
