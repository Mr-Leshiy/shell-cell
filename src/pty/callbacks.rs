use std::sync::mpsc::Sender;

use bytes::Bytes;
use tui_term::vt100::{Callbacks, Parser};

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
            (Some(b'#'), None, b'8') => decaln(screen),
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

/// DECALN - Screen Alignment Test
/// <https://ghostty.org/docs/vt/esc/decaln>
fn decaln(screen: &mut tui_term::vt100::Screen) {
    let (rows, cols) = screen.size();
    let mut parser = Parser::new(rows, cols, 0);
    // fills screen with 'E' characters
    parser.process(&(0..rows*cols).map(|_| b'E').collect::<Vec<u8>>());
    // move cursor to top left position
    parser.process(b"\x1B[H");
    screen.clone_from(parser.screen());
}
