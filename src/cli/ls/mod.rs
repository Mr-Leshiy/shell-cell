mod app;

use crate::{buildkit::BuildKitD, cli::{ls::app::{App, LsState}, Cli}};

impl Cli {
    pub async fn ls(self) -> color_eyre::Result<()> {
        let buildkit = BuildKitD::start().await?;
        let containers = buildkit.list_containers().await?;

        let mut terminal = ratatui::try_init()?;

        let app = App::Ls(LsState::new(containers));
        let res = app.run(&mut terminal);
        ratatui::try_restore()?;
        res
    }
}

