use std::path::PathBuf;

use crate::{buildkit::BuildKitD, cli::MIN_FPS, pty::Pty, scell::{name::SCellId, types::name::TargetName}};

mod ui;

pub struct RunningPtyState {
    pub pty: Pty,
    pub container_id: SCellId,
    pub target_name: TargetName,
    pub location: PathBuf,
    pub prev_height: u16,
    pub prev_width: u16,
}

impl RunningPtyState {
    pub fn new(
        pty: Pty,
        container_id: SCellId,
        target_name: TargetName,
        location: PathBuf,
    ) -> Self {
        Self {
            pty,
            container_id,
            target_name,
            location,
            prev_height: 0,
            prev_width: 0,
        }
    }

    pub fn scroll_up(&mut self) {
        self.pty.scroll_up();
    }

    pub fn scroll_down(&mut self) {
        self.pty.scroll_down();
    }

    pub fn try_update(&mut self) -> bool {
        self.pty.process_stdout_and_stderr(MIN_FPS)
    }

    pub fn notify_screen_resize(
        &mut self,
        buildkit: BuildKitD,
    ) {
        // Notify container's session about screen resize
        let (curr_height, curr_width) = self.pty.size();
        if curr_height != self.prev_height || curr_width != self.prev_width {
            tokio::spawn({
                let session_id = self.pty.container_session_id().to_owned();
                async move {
                    buildkit
                        .resize_shell(&session_id, curr_height, curr_width)
                        .await?;
                    color_eyre::eyre::Ok(())
                }
            });

            self.prev_height = curr_height;
            self.prev_width = curr_width;
        }
    }
}
