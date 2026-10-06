//! Attachment input stops on the first signal even when ancillary stdin has no EOF.
use super::{Acknowledgment, finish_promptly};
use crate::harness::{Canned, Listener, command};
use std::io::Write as _;
use std::os::unix::process::ExitStatusExt as _;
use std::process::Stdio;

#[test]
fn first_signal_terminates_image_attachment_input_with_an_open_writer_and_zero_sends() {
    let red = super::super::images::fixture("red.png");
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    for (flag, signal) in [
        ("-INT", signal_hook::consts::signal::SIGINT),
        ("-TERM", signal_hook::consts::signal::SIGTERM),
    ] {
        let acknowledgment = Acknowledgment::new();
        let mut child = command(
            &[
                "decide",
                "Q?",
                "--image",
                red.to_str().unwrap(),
                "--backend",
                "liquid",
                "--model",
                "d1",
                "--url",
                listener.base(),
                "--no-cache",
            ],
            &[("THINKTHEN_API_KEY", "sk-image-test")],
        )
        .env("THINKTHEN_TEST_SIGINT_ACK", &acknowledgment.0)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
        let mut writer = child.stdin.take().unwrap();
        // Larger than the pipe buffer: completion proves the command reached
        // its input read with signal handling installed. Keep the writer open.
        writer.write_all(&vec![b'x'; 2 * 1024 * 1024]).unwrap();
        assert!(
            crate::child::command("kill", &[])
                .args([flag, &child.id().to_string()])
                .status()
                .unwrap()
                .success()
        );
        acknowledgment.wait().unwrap();
        let output = finish_promptly(child, "cancelled image attachment stdin").unwrap();
        assert_eq!(output.status.signal(), Some(signal));
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("sk-image-test"));
        assert_eq!(listener.connections(), 0);
        assert_eq!(listener.count(), 0);
        drop(writer);
    }
}

#[test]
fn a_later_invalid_attachment_refuses_before_stdin_eof_or_any_send() {
    let red = super::super::images::fixture("red.png");
    let bad = super::super::images::fixture("progressive-app14-one-component.jpg");
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let mut child = command(
        &[
            "decide",
            "Q?",
            "--image",
            red.to_str().unwrap(),
            "--image",
            bad.to_str().unwrap(),
            "--backend",
            "liquid",
            "--model",
            "d1",
            "--url",
            listener.base(),
            "--no-cache",
        ],
        &[("THINKTHEN_API_KEY", "sk-image-test")],
    )
    .stdin(Stdio::piped())
    .spawn()
    .unwrap();
    let writer = child.stdin.take().unwrap();
    let output = finish_promptly(child, "invalid indivisible attachments").unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: image media or compressed pixels are invalid or exceed decoder limits\n"
    );
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
    assert_eq!(listener.count(), 0);
    drop(writer);
}
