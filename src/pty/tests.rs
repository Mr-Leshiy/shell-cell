use std::time::Duration;

use bollard::container::LogOutput;
use bytes::Bytes;
use test_case::test_case;
use tokio::io::AsyncReadExt;

use crate::pty::Pty;

const SCREEN_SIZE: u16 = 10;

// -----
// ESC test cases
// -----
#[test_case(
    &[b"\x1B#8"]
    =>
    (
        vec!['E'; usize::from(SCREEN_SIZE * SCREEN_SIZE)].into_iter().collect::<String>(),
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
        "B   AX".to_string(),
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
        "A\n X".to_string(),
        (1, 2),
    )
    ;
    "IND V-1: No Scroll Region, Top of Screen" // <https://ghostty.org/docs/vt/esc/ind#ind-v-1:-no-scroll-region-top-of-screen>
)]
#[test_case(
    &[
        b"A\r\n",        // print A + newline
        b"B\r\n",        // print B + newline
        b"C\r\n",        // print C + newline
        b"\x1B[1;1H",  // move to top-left
        b"\x1BM",      // RI - reverse index (move up one line, scroll if at top)
        b"X",          // print X
    ]
    =>
    (
        "X\nA\nB\nC".to_string(),
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
        "X\nB\nC".to_string(),
        (0, 1),
    )
    ;
    "RI V-2: No Scroll Region, Not Top of Screen" // <https://ghostty.org/docs/vt/esc/ri#ri-v-2:-no-scroll-region-not-top-of-screen>
)]
// -----
// CSI test cases
// -----
// TODO: implement
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
    pty.set_size(SCREEN_SIZE, SCREEN_SIZE);

    for _ in stdout {
        assert!(!pty.process_stdout_and_stderr(TIMEOUT));
    }
    (pty.screen().contents(), pty.screen().cursor_position())
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
    pty.set_size(SCREEN_SIZE, SCREEN_SIZE);

    for _ in stdout {
        assert!(!pty.process_stdout_and_stderr(TIMEOUT));
    }

    drop(pty);
    // The write side is dropped with pty, so this will read until EOF
    let mut result = Vec::new();
    input_reader.read_to_end(&mut result).await.unwrap();
    result
}
