//! This module implements a high level abstraction of the PTY terminal.

use std::{
    io::{Read, Write},
    pin::Pin,
};

use bollard::container::LogOutput;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use tokio::io::{AsyncWrite, AsyncWriteExt};

type Output = Pin<Box<dyn Stream<Item = Result<LogOutput, bollard::errors::Error>> + Send>>;
type Input = Pin<Box<dyn AsyncWrite + Send>>;

const BUF_SIZE: usize = 1024;

pub async fn run(stream: &PtyStdStreams) -> anyhow::Result<()> {
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
        crossterm::cursor::MoveTo(0, 0)
    )?;
    crossterm::terminal::enable_raw_mode()?;
    {
        let stdout_jh: tokio::task::JoinHandle<Result<(), anyhow::Error>> = tokio::spawn({
            let pty_stdout = stream.stdout();
            async move {
                let mut stdout = std::io::stdout();
                while let Ok(bytes) = pty_stdout.recv().await {
                    print!("{}", String::from_utf8_lossy(&bytes));
                    stdout.flush()?;
                }
                anyhow::Ok(())
            }
        });

        let stderr_jh: tokio::task::JoinHandle<Result<(), anyhow::Error>> = tokio::spawn({
            let pty_stderr = stream.stderr();
            async move {
                let mut stderr = std::io::stderr();
                while let Ok(bytes) = pty_stderr.recv().await {
                    eprint!("{}", String::from_utf8_lossy(&bytes));
                    stderr.flush()?;
                }
                anyhow::Ok(())
            }
        });

        let stdin_jh = tokio::task::spawn_blocking({
            let pty_stdin = stream.stdin();
            #[allow(clippy::indexing_slicing)]
            move || {
                let mut stdin = std::io::stdin();
                let mut buf = [0u8; BUF_SIZE];
                loop {
                    let n = stdin.read(&mut buf)?;
                    if n == 0 {
                        break;
                    }
                    pty_stdin.send(Bytes::copy_from_slice(&buf[..n]))?;
                }
                anyhow::Ok(())
            }
        });

        tokio::select! {
            _ = stdout_jh => (),
            _ = stderr_jh => (),
            _ = stdin_jh => (),
        };
    }

    crossterm::terminal::disable_raw_mode()?;

    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
        crossterm::cursor::MoveTo(0, 0)
    )?;

    Ok(())
}

pub struct PtyStdStreams {
    stdin: tokio::sync::mpsc::UnboundedSender<Bytes>,
    stdout: async_channel::Receiver<Bytes>,
    stderr: async_channel::Receiver<Bytes>,
}

impl PtyStdStreams {
    #[allow(unused_variables)]
    pub fn new(
        mut output: Output,
        mut input: Input,
    ) -> Self {
        let (stdout_in, stdout) = async_channel::unbounded();
        let (stderr_in, stderr) = async_channel::unbounded();
        let _jh = tokio::spawn(async move {
            while let Some(Ok(msg)) = output.next().await {
                match msg {
                    LogOutput::StdOut { message }
                    | LogOutput::StdIn { message }
                    | LogOutput::Console { message } => {
                        stdout_in.send(message).await?;
                    },
                    LogOutput::StdErr { message } => {
                        stderr_in.send(message).await?;
                    },
                }
            }
            anyhow::Ok(())
        });

        let (stdin, mut stdin_out) = tokio::sync::mpsc::unbounded_channel::<Bytes>();
        let _jh = tokio::spawn(async move {
            while let Some(bytes) = stdin_out.recv().await {
                input.write_all(&bytes).await?;
                input.flush().await?;
            }
            anyhow::Ok(())
        });

        Self {
            stdin,
            stdout,
            stderr,
        }
    }

    pub fn stdin(&self) -> tokio::sync::mpsc::UnboundedSender<Bytes> {
        self.stdin.clone()
    }

    pub fn stdout(&self) -> async_channel::Receiver<Bytes> {
        self.stdout.clone()
    }

    pub fn stderr(&self) -> async_channel::Receiver<Bytes> {
        self.stderr.clone()
    }
}
