//! This module implements a high level abstraction of the PTY terminal.

use std::{
    pin::Pin,
    sync::mpsc::{Receiver, Sender},
};

use bollard::container::LogOutput;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use tokio::io::{AsyncWrite, AsyncWriteExt};

type Output = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;
type Input = Pin<Box<dyn AsyncWrite + Send>>;

pub struct PtyStdSession {
    pub stdin: Sender<Bytes>,
    pub stdout: Receiver<Bytes>,
    pub stderr: Receiver<Bytes>,
    session_id: String,
}

impl PtyStdSession {
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn new(
        session_id: String,
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

        Self {
            stdin,
            stdout,
            stderr,
            session_id,
        }
    }
}
