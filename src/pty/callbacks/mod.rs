mod csi;
mod esc;

use std::sync::mpsc::Sender;

use bytes::Bytes;
use tui_term::vt100::Callbacks;

/// `vt100::Callbacks` implementation, to properly handle unhandled by `vt100::Parser`
/// escape codes.
pub struct TerminalCallback(pub Sender<Bytes>);

impl Callbacks for TerminalCallback {
    fn unhandled_escape(
        &mut self,
        screen: &mut tui_term::vt100::Screen,
        i1: Option<u8>,
        i2: Option<u8>,
        b: u8,
    ) {
        match (i1, i2, b) {
            // https://ghostty.org/docs/vt/esc/decaln
            (Some(b'#'), None, b'8') => esc::decaln(screen),
            (None, None, b'D') => esc::ind(screen),
            (None, None, b'M') => esc::ri(screen),
            _ => {},
        }
    }

    /// This callback is called when the terminal receives a CSI sequence
    /// (`\e[`) which is otherwise not implemented.
    ///
    /// <https://ghostty.org/docs/vt/concepts/sequences#csi>
    fn unhandled_csi(
        &mut self,
        screen: &mut tui_term::vt100::Screen,
        i1: Option<u8>,
        i2: Option<u8>,
        params: &[&[u16]],
        c: char,
    ) {
        match (i1, i2, params, c) {
            // Cursor Backward Tabulation <https://ghostty.org/docs/vt/csi/cbt>
            (None, None, &[&[n]], 'Z') => csi::cbt(screen, n),
            // Cursor Horizontal Tabulation <https://ghostty.org/docs/vt/csi/cht>
            (None, None, &[&[n]], 'I') => csi::cht(screen, n),
            // Device Status Report (operating status) <https://ghostty.org/docs/vt/csi/dsr>
            (None, None, &[&[5]], 'n') => csi::dsr_status(&self.0),
            // Device Status Report (cursor position) <https://ghostty.org/docs/vt/csi/dsr>
            (None, None, &[&[6]], 'n') => csi::dsr_cursor(&self.0, screen),
            _ => {},
        }
    }
}
