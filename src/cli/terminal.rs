use ratatui::Frame;

pub struct Terminal(ratatui::DefaultTerminal);

impl Terminal {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self(ratatui::try_init()?))
    }

    pub fn draw<F>(
        &mut self,
        render_fn: F,
    ) -> color_eyre::Result<()>
    where
        F: FnOnce(&mut Frame<'_>),
    {
        self.0
            .draw(render_fn)
            .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;
        Ok(())
    }
}
