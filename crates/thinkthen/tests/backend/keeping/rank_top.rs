//! Completed top-N outputs and the ordered admission window through the CLI.

use super::{Canned, Listener, RECORDS, answered, code, over, printed, said};
use conformance_backend::{Observed, Rendezvous};
use std::io;
use std::sync::{Arc, mpsc};
use std::time::Duration;

#[test]
fn top_edges_keep_literal_order_across_widths_and_batch_one() -> io::Result<()> {
    let expected = [
        (
            "1",
            "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n",
        ),
        (
            "2",
            concat!(
                "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n",
                "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n"
            ),
        ),
        (
            "8",
            concat!(
                "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n",
                "{\"id\":\"R-4\",\"body\":\"The refund never arrived.\"}\n",
                "{\"id\":\"R-1\",\"body\":\"The payout failed again.\"}\n",
                "{\"id\":\"R-3\",\"body\":\"The card was refused at checkout.\"}\n"
            ),
        ),
    ];
    for jobs in ["1", "4"] {
        for (top, lines) in expected {
            let listener = super::by_body(&[
                ("payout", "0.4"),
                ("quick fix", "0.9"),
                ("checkout", "0.4"),
                ("refund never", "0.9"),
            ])?;
            let output = over(
                "rank",
                listener.base(),
                &[
                    "--jsonl", "--field", "/body", "--batch", "1", "--jobs", jobs, "--top", top,
                ],
                RECORDS,
            )?;
            assert_eq!(code(&output), 0, "{jobs}/{top}: {}", said(&output));
            assert_eq!(printed(&output), lines, "{jobs}/{top}");
            assert_eq!(listener.requests().len(), 4, "every record was judged");
        }
    }
    Ok(())
}

#[test]
fn a_failed_later_row_keeps_rank_top_stdout_empty() -> io::Result<()> {
    let listener = Listener::serving(vec![
        Canned::ok(&answered("0.9")),
        Canned::status(500, "{}"),
    ])?;
    let output = over(
        "rank",
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/body",
            "--batch",
            "1",
            "--jobs",
            "1",
            "--top",
            "1",
            "--max-retries",
            "0",
        ],
        RECORDS,
    )?;
    assert_eq!(code(&output), 4);
    assert_eq!(printed(&output), "");
    assert_eq!(
        said(&output),
        concat!(
            "thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries\n",
            "thinkthen: stopped at record 2; 1 record finished, and nothing was printed because an order needs every record\n",
        )
    );
    Ok(())
}

#[test]
fn a_held_first_answer_bounds_top_dispatch_until_release() -> io::Result<()> {
    let release = Arc::new(Rendezvous::new(2));
    let (answered_send, answered_recv) = mpsc::channel();
    let (events_send, events_recv) = mpsc::channel();
    let listener = Listener::answering_with_events(
        {
            let release = Arc::clone(&release);
            move |body| {
                let sent = String::from_utf8_lossy(body);
                if sent.contains("payout") {
                    Canned::ok(&answered("0.4")).after_release(Arc::clone(&release))
                } else {
                    Canned::ok(&answered("0.9")).notifying(answered_send.clone())
                }
            }
        },
        events_send,
    )?;
    let address = listener.base().to_owned();
    let run = std::thread::spawn(move || {
        over(
            "rank",
            &address,
            &[
                "--jsonl", "--field", "/body", "--batch", "1", "--jobs", "2", "--top", "1",
            ],
            RECORDS,
        )
    });
    for _ in 0..2 {
        assert!(matches!(
            events_recv.recv_timeout(Duration::from_secs(2)),
            Ok(Observed::Request)
        ));
    }
    answered_recv
        .recv_timeout(Duration::from_secs(2))
        .expect("second answer completed");
    let next = events_recv.recv_timeout(Duration::from_millis(200));
    assert!(matches!(next, Err(mpsc::RecvTimeoutError::Timeout)));
    release.wait();
    let output = run.join().expect("command thread")?;
    assert_eq!(code(&output), 0, "{}", said(&output));
    assert_eq!(listener.requests().len(), 4);
    assert_eq!(
        printed(&output),
        "{ \"id\" : \"R-2\" ,  \"body\" : \"Thanks for the quick fix.\" }\n"
    );
    Ok(())
}
