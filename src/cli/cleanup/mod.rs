mod app;

use self::app::App;
use crate::{buildkit::BuildKitD, cli::Cli};

impl Cli {
    pub async fn cleanup(self) -> color_eyre::Result<()> {
        let buildkit = BuildKitD::start().await?;
        let mut terminal = ratatui::try_init()?;
        let app = App::loading(buildkit);
        let res = app.run(&mut terminal);
        ratatui::try_restore()?;
        res
    }
}
