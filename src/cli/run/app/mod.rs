mod ui;

use std::{path::Path, sync::mpsc::Receiver};

use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use crate::{buildkit::BuildKitD, cli::UPDATE_TIMEOUT, pty::PtyStdStreams, scell::SCell};

pub enum App {
    Compiling(Receiver<color_eyre::Result<SCell>>),
    BuildImage(Receiver<color_eyre::Result<SCell>>),
    StartContainer(Receiver<color_eyre::Result<PtyStdStreams>>),
    StartSession(Receiver<()>),
    Finished,
    Exit,
}

impl App {
    pub fn run<B: ratatui::backend::Backend, P: AsRef<Path> + Send + 'static>(
        buildkit: BuildKitD,
        scell_path: P,
        _verbose: bool,
        terminal: &mut Terminal<B>,
    ) -> color_eyre::Result<()> {
        // First step
        let mut app = Self::compiling(scell_path);

        loop {
            if let App::Compiling(ref rx) = app
                && let Ok(res) = rx.recv_timeout(UPDATE_TIMEOUT)
            {
                let scell = res?;
                app = Self::build_image(buildkit.clone(), scell);
            }

            if let App::BuildImage(ref rx) = app
                && let Ok(res) = rx.recv_timeout(UPDATE_TIMEOUT)
            {
                let scell = res?;
                app = Self::start_container(buildkit.clone(), scell);
            }

            if let App::StartContainer(ref rx) = app
                && let Ok(res) = rx.recv_timeout(UPDATE_TIMEOUT)
            {
                res?;
                app = Self::start_session();
            }

            if let App::StartSession(ref rx) = app
                && let Ok(_) = rx.recv_timeout(UPDATE_TIMEOUT)
            {}

            if matches!(app, App::Exit) {
                return Ok(());
            }

            terminal
                .draw(|f| {
                    f.render_widget(&app, f.area());
                })
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?;

            app.handle_key_event()?;
        }
    }

    fn handle_key_event(&mut self) -> color_eyre::Result<()> {
        if event::poll(UPDATE_TIMEOUT)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && let KeyCode::Char('c' | 'd') = key.code
            && key.modifiers.contains(event::KeyModifiers::CONTROL)
        {
            *self = App::Exit;
        }

        Ok(())
    }

    fn compiling<P: AsRef<Path> + Send + 'static>(scell_path: P) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let res = SCell::compile(scell_path, None);
            drop(tx.send(res));
        });
        App::Compiling(rx)
    }

    fn build_image(
        buildkit: BuildKitD,
        scell: SCell,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let res = buildkit.build_image(&scell, |_| {}).await;
            drop(tx.send(res.map(|_| scell)));
        });
        App::BuildImage(rx)
    }

    fn start_container(
        buildkit: BuildKitD,
        scell: SCell,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            let start = async || {
                buildkit.start_container(&scell).await?;
                buildkit.attach_to_shell(&scell).await
            };
            let res = start().await;
            drop(tx.send(res));
        });
        App::StartContainer(rx)
    }

    fn start_session() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            let _ = tx.send(());
        });
        App::StartSession(rx)
    }
}
