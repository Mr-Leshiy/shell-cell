use std::time::Duration;

use bollard::container::LogOutput;
use bytes::Bytes;
use test_case::test_case;

use crate::pty::Pty;

const SCREEN_SIZE: u16 = 3;

#[test_case(
    &[b"\x1B#8"]
    =>
    (
        "EEEEEEEEE".to_string(),
        (0, 0),
    )
    ;
    "DECALN V-1: Simple Usage" // <https://ghostty.org/docs/vt/esc/decaln>
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pty_test(stdout: &'static [&[u8]]) -> (String, (u16, u16)) {
    const TIMEOUT: Duration = Duration::from_secs(1);

    let input = Box::pin(Vec::new());
    let output = Box::pin(futures::stream::iter(stdout.iter().map(|s| {
        Ok(LogOutput::StdOut {
            message: Bytes::copy_from_slice(*s),
        })
    })));
    let mut pty = Pty::new("test_session".to_string(), output, input);
    pty.set_size(SCREEN_SIZE, SCREEN_SIZE);

    for _ in stdout {
         assert!(!pty.process_stdout_and_stderr(TIMEOUT));
    }
    (pty.screen().contents(), pty.screen().cursor_position())
}
