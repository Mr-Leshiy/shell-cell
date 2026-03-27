use ratatui::{self, Frame, crossterm, prelude::CrosstermBackend};

pub struct Terminal(ratatui::DefaultTerminal);

impl Terminal {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self(try_init()?))
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

impl Drop for Terminal {
    fn drop(&mut self) {
        restore();
    }
}

fn try_init() -> color_eyre::Result<ratatui::Terminal<CrosstermBackend<std::io::Stdout>>> {
    set_panic_hook();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableBracketedPaste
    )?;
    CrosstermBackend::new(std::io::stdout());
    Ok(ratatui::Terminal::new(CrosstermBackend::new(
        std::io::stdout(),
    ))?)
}

fn restore() {
    if let Err(err) = try_restore() {
        // There's not much we can do if restoring the terminal fails, so we just print the error
        std::eprintln!("Failed to restore terminal: {err}");
    }
}

fn try_restore() -> color_eyre::Result<()> {
    // disabling raw mode first is important as it has more side effects than leaving the
    // alternate screen buffer
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableBracketedPaste
    )?;
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
}
