//! Terminal emulator implementation.
//! In the current implementation would follow the same API as <https://ghostty.org> has.
//! The full reference to the ghostty terminal API documentation <https://ghostty.org/docs/vt>.

use std::sync::mpsc::Sender;

use bytes::Bytes;
use tui_term::vt100::{Callbacks, Parser, Screen};

///  Terminal emulator implementation (VT)
pub struct TerminalEmulator(Parser<TerminalCallback>);

/// `vt100::Callbacks` implementation, to properly handle unhandled by `vt100::Parser`
/// escape codes.
struct TerminalCallback {
    stdin: Sender<Bytes>,
}

impl TerminalEmulator {
    pub fn new(stdin: Sender<Bytes>) -> Self {
        Self(Parser::new_with_callbacks(24, 80, 0, TerminalCallback {
            stdin,
        }))
    }

    pub fn process(
        &mut self,
        bytes: &[u8],
    ) {
        self.0.process(bytes);
    }

    pub fn screen(&self) -> &Screen {
        self.0.screen()
    }

    pub fn size(&self) -> (u16, u16) {
        self.0.screen().size()
    }

    pub fn set_size(
        &mut self,
        height: u16,
        width: u16,
    ) {
        self.0.screen_mut().set_size(height, width);
    }
}

impl TerminalCallback {
    fn send(
        &self,
        data: impl Into<Bytes>,
    ) {
        drop(self.stdin.send(data.into()));
    }
}

impl Callbacks for TerminalCallback {
    /// This callback is called when the terminal receives a CSI sequence
    /// (`\e[`) which is otherwise not implemented.
    ///
    /// <https://ghostty.org/docs/vt/concepts/sequences#csi>
    fn unhandled_csi(
        &mut self,
        screen: &mut tui_term::vt100::Screen,
        _i1: Option<u8>,
        _i2: Option<u8>,
        params: &[&[u16]],
        c: char,
    ) {
        match c {
            // Device Status Report <https://ghostty.org/docs/vt/csi/dsr>
            'n' => {
                // The operating status is requested
                if let &[&[5]] = params {
                    self.send(b"\x1b[0n".as_slice());
                }
                // The cursor position is requested
                if let &[&[6]] = params {
                    let (row, col) = screen.cursor_position();
                    self.send(format!("\x1b[{};{}R", row + 1, col + 1).into_bytes());
                }
            },
            _ => {},
        }
    }
}

#[cfg(test)]
mod tests {}
