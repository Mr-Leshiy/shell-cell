use tui_term::vt100::Parser;

/// DECALN - Screen Alignment Test
/// <https://ghostty.org/docs/vt/esc/decaln>
pub fn decaln(screen: &mut tui_term::vt100::Screen) {
    let (rows, cols) = screen.size();
    let mut parser = Parser::new(rows, cols, 0);
    // fills screen with 'E' characters
    let items = u32::from(rows).saturating_mul(cols.into());
    parser.process(&(0..items).map(|_| b'E').collect::<Vec<u8>>());
    // move cursor to top left position
    parser.process(b"\x1B[H");
    screen.clone_from(parser.screen());
}

/// IND - Index
/// <https://ghostty.org/docs/vt/esc/ind>
pub fn ind(screen: &mut tui_term::vt100::Screen) {
    let (rows, cols) = screen.size();
    let (cursor_row, cursor_col) = screen.cursor_position();

    let mut seq = Vec::new();
    if cursor_row == rows.saturating_sub(1) {
        // At the bottom — scroll entire screen up one line
        seq.extend_from_slice(b"\x1B[S");
    } else {
        // Move cursor down one line, same column (1-indexed)
        seq.extend_from_slice(
            format!(
                "\x1B[{};{}H",
                cursor_row.saturating_add(2),
                cursor_col.saturating_add(1)
            )
            .as_bytes(),
        );
    }

    let contents = screen.contents_formatted();
    let mut parser = Parser::new(rows, cols, 0);
    parser.process(&contents);
    parser.process(&seq);
    screen.clone_from(parser.screen());
}

/// RI - Reverse Index
/// <https://ghostty.org/docs/vt/esc/ri>
pub fn ri(screen: &mut tui_term::vt100::Screen) {
    let (rows, cols) = screen.size();
    let (cursor_row, cursor_col) = screen.cursor_position();

    let mut seq = Vec::new();

    if cursor_row == 0 {
        // At the top — scroll entire screen down one line (ESC[T or ESC[1T)
        seq.extend_from_slice(b"\x1B[T");
    } else {
        // Move cursor up one line, same column (1-indexed)
        seq.extend_from_slice(
            format!("\x1B[{};{}H", cursor_row, cursor_col.saturating_add(1)).as_bytes(),
        );
    }

    let contents = screen.contents_formatted();
    let mut parser = Parser::new(rows, cols, 0);
    parser.process(&contents);
    parser.process(&seq);
    screen.clone_from(parser.screen());
}
