//! This module implements a high level abstraction of the PTY terminal.
//! In the current implementation would follow the same API as <https://ghostty.org> has.
//! The full reference to the ghostty terminal API documentation <https://ghostty.org/docs/vt>.

mod callbacks;
#[cfg(test)]
mod tests;

use std::{
    pin::Pin,
    sync::mpsc::{Receiver, RecvTimeoutError, Sender},
    time::Duration,
};

use bollard::container::LogOutput;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tui_term::vt100::{Parser, Screen};

use crate::pty::callbacks::TerminalCallback;

type Output = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;
type Input = Pin<Box<dyn AsyncWrite + Send>>;

pub struct Pty {
    stdin: Sender<Bytes>,
    stdout: Receiver<Bytes>,
    stderr: Receiver<Bytes>,
    container_session_id: String,
    parser: Parser<TerminalCallback>,
}

impl Pty {
    pub fn new(
        container_session_id: String,
        mut output: Output,
        mut input: Input,
    ) -> Self {
        let (stdout_in, stdout) = std::sync::mpsc::channel();
        let (stderr_in, stderr) = std::sync::mpsc::channel();
        let _jh = tokio::spawn(async move {
            while let Some(Ok(msg)) = output.next().await {
                match msg {
                    LogOutput::StdOut { message }
                    | LogOutput::StdIn { message }
                    | LogOutput::Console { message } => {
                        stdout_in.send(message)?;
                    },
                    LogOutput::StdErr { message } => {
                        stderr_in.send(message)?;
                    },
                }
            }
            color_eyre::eyre::Ok(())
        });

        let (stdin, stdin_out) = std::sync::mpsc::channel::<Bytes>();
        let _jh = tokio::spawn(async move {
            while let Ok(bytes) = stdin_out.recv() {
                input.write_all(&bytes).await?;
                input.flush().await?;
            }
            color_eyre::eyre::Ok(())
        });

        let parser = Parser::new_with_callbacks(24, 80, 0, TerminalCallback(stdin.clone()));
        Self {
            stdin,
            stdout,
            stderr,
            container_session_id,
            parser,
        }
    }

    pub fn container_session_id(&self) -> &str {
        &self.container_session_id
    }

    pub fn screen(&self) -> &Screen {
        self.parser.screen()
    }

    /// Returns the current size of the terminal.
    pub fn size(&self) -> (u16, u16) {
        self.parser.screen().size()
    }

    /// Resizes the terminal.
    pub fn set_size(
        &mut self,
        height: u16,
        width: u16,
    ) {
        self.parser.screen_mut().set_size(height, width);
    }

    /// Processes new updates from the `stdout` and `stderr` channels.
    /// Returns `true` if both channels are closed already.
    pub fn process_stdout_and_stderr(
        &mut self,
        timeout: Duration,
    ) -> bool {
        let stdout_res = match self.stdout.recv_timeout(timeout) {
            Ok(bytes) => {
                self.parser.process(&bytes);
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        };

        let stderr_res = match self.stderr.recv_timeout(timeout) {
            Ok(bytes) => {
                self.parser.process(&bytes);
                false
            },
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        };

        stdout_res && stderr_res
    }

    pub fn process_stdin(
        &self,
        bytes: &[u8],
    ) {
        drop(self.stdin.send(Bytes::copy_from_slice(bytes)));
    }
}
