mod app;

use self::app::App;
use crate::buildkit::BuildKitD;

pub async fn cleanup() -> color_eyre::Result<()> {
    let buildkit = BuildKitD::start().await?;
    let mut terminal = ratatui::try_init()?;
    let res = App::run(&buildkit, &mut terminal);
    ratatui::try_restore()?;
    res
}
