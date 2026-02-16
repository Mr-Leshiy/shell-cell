mod app;

use crate::{
    buildkit::BuildKitD,
    cli::{Cli, run::app::App},
};

impl Cli {
    pub async fn run(self) -> color_eyre::Result<()> {
        let buildkit = BuildKitD::start().await?;
        let mut terminal = ratatui::try_init()?;
        let res = App::run(&buildkit, self.scell_path, self.target, &mut terminal);
        ratatui::try_restore()?;
        res
    }
}
