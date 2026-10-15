//! Push-all callers retain ordered output without needing a concurrent reader.
use super::*;
#[path = "../../src/test_deadline/child.rs"]
mod child;
use child::ChildEnvironment as _;

#[test]
fn push_all_then_finish_reads_ordered_prefix_at_request_cap() {
    const CASE: &str = "push::push_all_then_finish_reads_ordered_prefix_at_request_cap";
    const CHILD: &str = "THINKTHEN_TEST_PUSH_ALL_CHILD";
    if std::env::var_os(CHILD).is_none() {
        // Scheduler width is process-wide; isolate this width-one caller from
        // the parallel recognition cases in the same integration binary.
        let home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("push-all-session-{}", std::process::id()));
        std::fs::create_dir(&home).unwrap();
        let binary = std::env::current_exe().unwrap();
        let output = child::run::output(
            child::command(binary.to_str().unwrap(), &[])
                .args(["--exact", CASE, "--nocapture", "--test-threads=1"])
                .isolated_home(&home)
                .env(CHILD, "1"),
        );
        std::fs::remove_dir_all(&home).unwrap();
        let output = output.unwrap();
        let said = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success(),
            "{said}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(said.contains("1 passed"), "the child ran {CASE}");
        return;
    }
    push_all_case();
}

fn push_all_case() {
    for blocking in [false, true] {
        let listener = Listener::answering(response).unwrap();
        let mut arguments = request(feed()).call.arguments().clone();
        arguments.options.batch = Some(RequestBatch::Count(1));
        let request = Request::new(RequestCall::new(RequestFunction::Decide, arguments));
        let session = Arc::new(
            crate::api::fixed(listener.base(), "session-fixture")
                .no_cache()
                .max_retries(0)
                .throttle(1)
                .unwrap()
                .max_requests(Some(3))
                .unwrap()
                .build()
                .unwrap()
                .request_session(request)
                .unwrap(),
        );
        for index in 0..32 {
            let status = push_bounded(&session, index, blocking);
            if status == RequestSessionPushStatus::Closed {
                break;
            }
        }
        session.finish(None).unwrap();
        let packets = drain(&session);
        let kinds = packets
            .iter()
            .map(|packet| match packet {
                RequestSessionResult::Observation {
                    value: OwnedRecordObservation::Question { .. },
                    ..
                } => "question",
                RequestSessionResult::Observation { .. } => "observation",
                RequestSessionResult::Row(_) => "row",
                RequestSessionResult::Terminal(_) => "terminal",
                RequestSessionResult::Aggregate(_) => "aggregate",
            })
            .collect::<Vec<_>>();
        assert_eq!(
            kinds,
            [
                "question",
                "observation",
                "row",
                "question",
                "observation",
                "row",
                "question",
                "observation",
                "row",
                "terminal"
            ]
        );
        let rows = packets
            .iter()
            .filter_map(|packet| match packet {
                RequestSessionResult::Row(RequestSessionRow::Decision(row)) => Some(row),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            rows.iter().map(|row| row.ordinal()).collect::<Vec<_>>(),
            [0, 1, 2]
        );
        for (index, row) in rows.iter().enumerate() {
            let QuestionInput::Record(original) = row.original() else {
                panic!("feed original")
            };
            assert_eq!(
                original.original().literal(),
                Some(format!("row {index}").as_str())
            );
        }
        let RequestSessionResult::Terminal(terminal) = packets.last().unwrap() else {
            panic!("terminal follows rows")
        };
        assert_eq!(terminal.error.as_ref().unwrap().kind(), ErrorKind::Usage);
        assert_eq!(terminal.facts.as_ref().unwrap().requests_sent(), 3);
        assert_eq!(listener.count(), 3);
    }
}

fn push_bounded(
    session: &Arc<RequestSession>,
    index: usize,
    blocking: bool,
) -> RequestSessionPushStatus {
    let waiting = Arc::clone(&session);
    let (sent, received) = mpsc::channel();
    let producer = std::thread::spawn(move || {
        let item = descriptor(&format!("row {index}"));
        let status = if blocking {
            waiting.push_with(|| Ok::<_, Error>(item)).unwrap()
        } else {
            let mut item = item;
            loop {
                match waiting.try_push(item).unwrap() {
                    RequestSessionPush::Accepted => {
                        break RequestSessionPushStatus::Accepted;
                    }
                    RequestSessionPush::Closed(_) => {
                        break RequestSessionPushStatus::Closed;
                    }
                    RequestSessionPush::Full(retry) => {
                        item = retry;
                        std::thread::yield_now();
                    }
                }
            }
        };
        let _ = sent.send(status);
    });
    let status = received.recv_timeout(Duration::from_secs(2));
    if status.is_err() {
        session.cancel();
    }
    producer.join().unwrap();
    status.unwrap()
}
