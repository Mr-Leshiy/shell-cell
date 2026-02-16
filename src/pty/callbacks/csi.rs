use std::sync::mpsc::Sender;

use bytes::Bytes;
use tui_term::vt100::Parser;

/// CBT - Cursor Backward Tabulation
///
/// Moves the cursor `n` tab stops to the left. Default tab stops are every 8 columns.
/// If the cursor would move past the leftmost column, it stays at column 0.
///
/// <https://ghostty.org/docs/vt/csi/cbt>
pub fn cbt(
    screen: &mut tui_term::vt100::Screen,
    n: u16,
) {
    const TAB_WIDTH: u16 = 8;
    let (rows, cols) = screen.size();
    let (cursor_row, cursor_col) = screen.cursor_position();

    // Calculate how many full tab stops to move back (n extra, plus the partial one we're
    // already in)
    let tabs_back = n.saturating_add(if cursor_col % TAB_WIDTH == 0 { 1 } else { 0 });
    let col = cursor_col.saturating_sub(tabs_back.saturating_mul(TAB_WIDTH));
    let tab_offset = col % TAB_WIDTH;
    let col = col.saturating_sub(tab_offset);

    let contents = screen.contents_formatted();
    let mut parser = Parser::new(rows, cols, 0);
    parser.process(&contents);
    parser.process(
        format!(
            "\x1B[{};{}H",
            cursor_row.saturating_add(1),
            col.saturating_add(1)
        )
        .as_bytes(),
    );
    screen.clone_from(parser.screen());
}

/// DSR - Device Status Report (operating status)
///
/// Responds with `ESC[0n` to indicate the terminal is functioning normally.
///
/// <https://ghostty.org/docs/vt/csi/dsr>
pub fn dsr_status(sender: &Sender<Bytes>) {
    drop(sender.send(b"\x1b[0n".as_slice().into()));
}

/// DSR - Device Status Report (cursor position)
///
/// Responds with `ESC[row;colR` (1-indexed) to report the current cursor position.
///
/// <https://ghostty.org/docs/vt/csi/dsr>
pub fn dsr_cursor(
    sender: &Sender<Bytes>,
    screen: &tui_term::vt100::Screen,
) {
    let (row, col) = screen.cursor_position();
    drop(
        sender.send(
            format!("\x1b[{};{}R", row.saturating_add(1), col.saturating_add(1))
                .into_bytes()
                .into(),
        ),
    );
}
