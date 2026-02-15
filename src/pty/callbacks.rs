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
            (Some(b'#'), None, b'8') => esc_decaln(screen),
            (None, None, b'D') => esc_ind(screen),
            (None, None, b'M') => esc_ri(screen),
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
fn esc_decaln(screen: &mut tui_term::vt100::Screen) {
    let (rows, cols) = screen.size();
    let mut parser = Parser::new(rows, cols, 0);
    // fills screen with 'E' characters
    parser.process(&(0..rows * cols).map(|_| b'E').collect::<Vec<u8>>());
    // move cursor to top left position
    parser.process(b"\x1B[H");
    screen.clone_from(parser.screen());
}

/// IND - Index
/// <https://ghostty.org/docs/vt/esc/ind>
fn esc_ind(screen: &mut tui_term::vt100::Screen) {
    let (rows, cols) = screen.size();
    let (cursor_row, cursor_col) = screen.cursor_position();

    let mut seq = Vec::new();
    if cursor_row == rows - 1 {
        // At the bottom — scroll entire screen up one line
        seq.extend_from_slice(b"\x1B[S");
    } else {
        // Move cursor down one line, same column (1-indexed)
        seq.extend_from_slice(format!("\x1B[{};{}H", cursor_row + 2, cursor_col + 1).as_bytes());
    }

    let contents = screen.contents_formatted();
    let mut parser = Parser::new(rows, cols, 0);
    parser.process(&contents);
    parser.process(&seq);
    screen.clone_from(parser.screen());
}

/// RI - Reverse Index
/// <https://ghostty.org/docs/vt/esc/ri>
fn esc_ri(screen: &mut tui_term::vt100::Screen) {
    let (rows, cols) = screen.size();
    let (cursor_row, cursor_col) = screen.cursor_position();

    let mut seq = Vec::new();

    if cursor_row == 0 {
        // At the top — scroll entire screen down one line (ESC[T or ESC[1T)
        seq.extend_from_slice(b"\x1B[T");
    } else {
        // Move cursor up one line, same column (1-indexed)
        seq.extend_from_slice(format!("\x1B[{};{}H", cursor_row, cursor_col + 1).as_bytes());
    }

    let contents = screen.contents_formatted();
    let mut parser = Parser::new(rows, cols, 0);
    parser.process(&contents);
    parser.process(&seq);
    screen.clone_from(parser.screen());
}
