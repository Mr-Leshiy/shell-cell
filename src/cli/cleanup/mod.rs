mod app;

use self::app::App;
use crate::{buildkit::BuildKitD, cli::terminal::Terminal};

pub async fn cleanup(
    all: bool,
    _silent: bool,
) -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let mut terminal = Terminal::new()?;
    let res = App::run(&buildkit, all, &mut terminal);
    ratatui::try_restore()?;
    res
}
