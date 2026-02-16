mod app;

use crate::{
    buildkit::BuildKitD,
    cli::{Cli, ls::app::App},
};

impl Cli {
    pub async fn ls(self) -> color_eyre::Result<()> {
        let buildkit = BuildKitD::start().await?;
        let mut terminal = ratatui::try_init()?;
        let res = App::run(&buildkit, &mut terminal);
        ratatui::try_restore()?;
        res
    }
}
