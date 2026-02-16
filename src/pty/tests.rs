use std::{fmt::Write, time::Duration};

use bollard::container::LogOutput;
use bytes::Bytes;
use indoc::indoc;
use test_case::test_case;
use tokio::io::AsyncReadExt;

use crate::pty::Pty;

const SCREEN_SIZE_WIDTH: u16 = 10;
const SCREEN_SIZE_HEIGHT: u16 = 3;

// -----
// Control test cases
// -----
#[test_case(
    &[
        b"\x1B[1;5H",
        b"\x08"
    ]
    =>
    (
        indoc!{"
        |__________|
        |__________|
        |__________|
        "}.to_string(),
        (0, 3),
    )
    ;
    "Backspace" // <https://ghostty.org/docs/vt/control/bs>
)]
#[test_case(
    &[
        b"\x1B[11G", // move to last column
        b"A",
        b"\r",
        b"X"
    ]
    =>
    (
        indoc!{"
        |X________A|
        |__________|
        |__________|
        "}.to_string(),
        (0, 1),
    )
    ;
    "Carriage Return V-1" // <https://ghostty.org/docs/vt/control/cr>
)]
#[test_case(
    &[
        b"\x1B[4G",
        b"A",
        b"\x1B[1G",
        b"\r",
        b"X"
    ]
    =>
    (
        indoc!{"
        |X__A______|
        |__________|
        |__________|
        "}.to_string(),
        (0, 1),
    )
    ;
    "Carriage Return V-2" // <https://ghostty.org/docs/vt/control/cr>
)]
#[test_case(
    &[
        b"\x0A",
        b"A",
    ]
    =>
    (
        indoc!{"
        |__________|
        |A_________|
        |__________|
        "}.to_string(),
        (1, 1),
    )
    ;
    "Linefeed" // <https://ghostty.org/docs/vt/control/lf>
)]
#[test_case(
    &[
        b"\x09",
        b"A",
    ]
    =>
    (
        indoc!{"
        |________A_|
        |__________|
        |__________|
        "}.to_string(),
        (0, 9),
    )
    ;
    "Tab" // <https://ghostty.org/docs/vt/control/tab>
)]
// -----
// ESC test cases
// -----
#[test_case(
    &[b"\x1B#8"]
    =>
    (
        indoc!{"
        |EEEEEEEEEE|
        |EEEEEEEEEE|
        |EEEEEEEEEE|
        "}.to_string(),
        (0, 0),
    )
    ;
    "DECALN V-1: Simple Usage" // <https://ghostty.org/docs/vt/esc/decaln#decaln-v-1:-simple-usage>
)]
#[test_case(
    &[
        b"\x1B[1;5H",
        b"A",
        b"\x1B7", // Save Cursor
        b"\x1B[1;1H",
        b"B",
        b"\x1B8", // Restore Cursor
        b"X",
    ]
    =>
    (
        indoc!{"
        |B___AX____|
        |__________|
        |__________|
        "}.to_string(),
        (0, 6),
    )
    ;
    "SC V-1: Cursor Position" // <https://ghostty.org/docs/vt/esc/decsc#sc-v-1:-cursor-position>
)]
#[test_case(
    &[
        b"A",          // print A
        b"\x1BD",      // IND - index (move down one line, scroll if at bottom)
        b"X",          // print X
    ]
    =>
    (
        indoc!{"
        |A_________|
        |_X________|
        |__________|
        "}.to_string(),
        (1, 2),
    )
    ;
    "IND V-1: No Scroll Region, Top of Screen" // <https://ghostty.org/docs/vt/esc/ind#ind-v-1:-no-scroll-region-top-of-screen>
)]
#[test_case(
    &[
        b"A\r\n",      // print A + newline
        b"B\r\n",      // print B + newline
        b"\x1B[1;1H",  // move to top-left
        b"\x1BM",      // RI - reverse index (move up one line, scroll if at top)
        b"X",          // print X
    ]
    =>
    (
        indoc!{"
        |X_________|
        |A_________|
        |B_________|
        "}.to_string(),
        (0, 1),
    )
    ;
    "RI V-1: No Scroll Region, Top of Screen" // <https://ghostty.org/docs/vt/esc/ri#ri-v-1:-no-scroll-region-top-of-screen>
)]
#[test_case(
    &[
        b"A\r\n",      // print A + CR+LF
        b"B\r\n",      // print B + CR+LF
        b"C\r\n",      // print C + CR+LF
        b"\x1B[2;1H",  // move to row 2, col 1
        b"\x1BM",      // RI - reverse index (move up, no scroll needed)
        b"X",          // print X at row 1, col 1
    ]
    =>
    (
        indoc!{"
        |X_________|
        |B_________|
        |C_________|
        "}.to_string(),
        (0, 1),
    )
    ;
    "RI V-2: No Scroll Region, Not Top of Screen" // <https://ghostty.org/docs/vt/esc/ri#ri-v-2:-no-scroll-region-not-top-of-screen>
)]
// -----
// CSI test cases
// -----
#[test_case(
    &[
        b"\x1B[2Z",
        b"A",
    ]
    =>
    (
        indoc!{"
        |A_________|
        |__________|
        |__________|
        "}.to_string(),
        (0, 1),
    )
    ;
    "CBT V-1: Left Beyond First Column" // <https://ghostty.org/docs/vt/csi/cbt#cbt-v-1:-left-beyond-first-column>
)]
#[test_case(
    &[
        b"\x1B[1;10H",
        b"X",
        b"\x1B[Z",
        b"A",
    ]
    =>
    (
        indoc!{"
        |________AX|
        |__________|
        |__________|
        "}.to_string(),
        (0, 9),
    )
    ;
    "CBT V-2: Left Starting After Tab Stop" // <https://ghostty.org/docs/vt/csi/cbt#cbt-v-2:-left-starting-after-tab-stop>
)]
#[test_case(
    &[
        b"\x1B[1;9H",
        b"X",
        b"\x1B[1;9H",
        b"\x1B[Z",
        b"A",
    ]
    =>
    (
        indoc!{"
        |A_______X_|
        |__________|
        |__________|
        "}.to_string(),
        (0, 1),
    )
    ;
    "CBT V-3: Left Starting on Tabstop" // <https://ghostty.org/docs/vt/csi/cbt#cbt-v-3:-left-starting-on-tabstop>
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pty_test(stdout: &'static [&[u8]]) -> (String, (u16, u16)) {
    const TIMEOUT: Duration = Duration::from_secs(1);

    let input = Box::pin(Vec::new());
    let output = Box::pin(futures::stream::iter(stdout.iter().map(|s| {
        Ok(LogOutput::StdOut {
            message: Bytes::copy_from_slice(s),
        })
    })));
    let mut pty = Pty::new("test_session".to_string(), output, input);
    pty.set_size(SCREEN_SIZE_HEIGHT, SCREEN_SIZE_WIDTH);

    for _ in stdout {
        assert!(!pty.process_stdout_and_stderr(TIMEOUT));
    }

    let mut res = String::new();
    let (rows, cols) = pty.size();
    for i in 0..rows {
        write!(&mut res, "|").unwrap();
        for j in 0..cols {
            let cell = pty.screen().cell(i, j).unwrap();
            if cell.has_contents() {
                write!(&mut res, "{}", cell.contents()).unwrap();
            } else {
                write!(&mut res, "_").unwrap();
            }
        }
        writeln!(&mut res, "|").unwrap();
    }
    (res, pty.screen().cursor_position())
}

#[test_case(
    &[
        b"\x1B[5n",
    ]
    =>
    b"\x1b[0n".to_vec()
    ;
    "DSR V-1: Operating Status" // <https://ghostty.org/docs/vt/csi/dsr#dsr-v-1:-operating-status>
)]
#[test_case(
    &[
        b"\x1B[2;4H",
        b"\x1B[6n",
    ]
    =>
    b"\x1b[2;4R".to_vec()
    ;
    "DSR V-2: Cursor Position" // <https://ghostty.org/docs/vt/csi/dsr#dsr-v-2:-cursor-position>
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pty_with_response_test(stdout: &'static [&[u8]]) -> Vec<u8> {
    const TIMEOUT: Duration = Duration::from_secs(1);

    let (input_writer, mut input_reader) = tokio::io::duplex(1024);
    let input = Box::pin(input_writer);
    let output = Box::pin(futures::stream::iter(stdout.iter().map(|s| {
        Ok(LogOutput::StdOut {
            message: Bytes::copy_from_slice(s),
        })
    })));
    let mut pty = Pty::new("test_session".to_string(), output, input);
    pty.set_size(SCREEN_SIZE_HEIGHT, SCREEN_SIZE_WIDTH);

    for _ in stdout {
        assert!(!pty.process_stdout_and_stderr(TIMEOUT));
    }

    drop(pty);
    // The write side is dropped with pty, so this will read until EOF
    let mut result = Vec::new();
    input_reader.read_to_end(&mut result).await.unwrap();
    result
}
