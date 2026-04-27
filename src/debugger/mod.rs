mod conf;

use std::{
    io::Write as _,
    sync::{Mutex, OnceLock},
};

use self::conf::DebuggerConfig;

static DEBUGGER: OnceLock<Debugger> = OnceLock::new();

#[derive(Debug)]
pub struct Debugger {
    id: uuid::Uuid,
    pty_stdin_logs: Mutex<std::fs::File>,
    pty_stdout_logs: Mutex<std::fs::File>,
}

impl Debugger {
    pub fn init() -> color_eyre::Result<()> {
        let config = DebuggerConfig::init()?;
        if config.enabled {
            let debug_dir = crate::scell_home_dir()?.join("debug");
            std::fs::create_dir(&debug_dir).or_else(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    Ok(())
                } else {
                    Err(e)
                }
            })?;
            let id = uuid::Uuid::now_v7();
            let pty_stdin_logs =
                std::fs::File::create_new(debug_dir.join(format!("{id}_pty_stdin.logs")))?;
            let pty_stdout_logs =
                std::fs::File::create_new(debug_dir.join(format!("{id}_pty_stdout.logs")))?;

            let debugger = Debugger {
                id,
                pty_stdin_logs: Mutex::new(pty_stdin_logs),
                pty_stdout_logs: Mutex::new(pty_stdout_logs),
            };
            DEBUGGER
                .set(debugger)
                .map_err(|_| color_eyre::eyre::eyre!("Debugger already initialised"))?;
        }
        Ok(())
    }

    pub fn session_id() -> Option<uuid::Uuid> {
        DEBUGGER.get().map(|dbg| dbg.id)
    }

    pub fn log_pty_stdin(bytes: &[u8]) -> color_eyre::Result<()> {
        if let Some(dbg) = DEBUGGER.get() {
            dbg.pty_stdin_logs
                .lock()
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?
                .write_all(bytes)?;
        }
        Ok(())
    }

    pub fn log_pty_stdout(bytes: &[u8]) -> color_eyre::Result<()> {
        if let Some(dbg) = DEBUGGER.get() {
            dbg.pty_stdout_logs
                .lock()
                .map_err(|e| color_eyre::eyre::eyre!("{e}"))?
                .write_all(bytes)?;
        }
        Ok(())
    }
}
