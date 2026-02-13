use std::sync::mpsc::Sender;

use bytes::Bytes;
use tui_term::vt100::Callbacks;

/// `vt100::Callbacks` implementation, to properly handle unhandled by `vt100::Parser`
/// escape codes.
pub struct TerminalCallback(pub Sender<Bytes>);

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
        // Device Status Report <https://ghostty.org/docs/vt/csi/dsr>
        if c == 'n' {
            // The operating status is requested
            if let &[&[5]] = params {
                drop(self.0.send(b"\x1b[0n".as_slice().into()));
            }
            // The cursor position is requested
            if let &[&[6]] = params {
                let (row, col) = screen.cursor_position();
                drop(
                    self.0.send(
                        format!("\x1b[{};{}R", row.saturating_add(1), col.saturating_add(1))
                            .into_bytes()
                            .into(),
                    ),
                );
            }
        }
    }
}
