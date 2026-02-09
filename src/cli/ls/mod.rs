mod app;

use crate::cli::{Cli, ls::app::App};

impl Cli {
    pub async fn ls(self) -> color_eyre::Result<()> {
        let mut terminal = ratatui::try_init()?;

        let app = App::loading();
        let res = app.run(&mut terminal);
        ratatui::try_restore()?;
        res
    }
}
