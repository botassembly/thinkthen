//! Each answer names its question key, in set order, by ADR 0111 section 7.

use super::scheduling::{all_yes, folder, grouped, grouped_input};
use super::{one_question, set};
use crate::harness::{Canned, Listener, spawn};
use crate::support::keys;

#[test]
fn detailed_requests_follow_group_order_when_groups_finish_in_reverse() {
    let listener = Listener::answering(move |body| {
        let delay = if String::from_utf8_lossy(body).contains("group 0") {
            40
        } else {
            5
        };
        Canned::ok(&all_yes(body, "local-1", 10, 2)).after(delay)
    })
    .expect("a listener");
    let file = grouped("reverse-completion-details", 2);
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--details",
            "--jobs",
            "2",
            "--profile",
            &one_question().to_string_lossy(),
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        grouped_input(1, 2).as_bytes(),
    )
    .expect("the run");

    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let row = String::from_utf8_lossy(&output.stdout);
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let key_for = |group: &str| {
        let request = requests
            .iter()
            .find(|request| String::from_utf8_lossy(&request.body).contains(group))
            .expect("the group was requested");
        keys(listener.url(), &request.body).remove(0)
    };
    let ordered = format!(
        r#""requests":["{}","{}"]"#,
        key_for("group 0"),
        key_for("group 1")
    );
    assert!(row.contains(&ordered), "{row}");
}

/// Two groups that ask the same question of the same evidence ask it once,
/// and both answers name its key.
#[test]
fn equal_logical_group_requests_keep_both_positions() {
    let cache = folder("annotate-equal-group-identities");
    let file = set(
        "equal-group-identities",
        concat!(
            r#"{"version":1,"questions":{"first":{"decide":"Same question?","on":"/left/text"},"#,
            r#""second":{"decide":"Same question?","on":"/right/text"}}}"#,
        ),
    );
    let listener =
        Listener::answering(move |body| Canned::ok(&all_yes(body, "local-1", 10, 2)).after(20))
            .expect("a listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--details",
            "--cache",
            &cache.to_string_lossy(),
            "--jobs",
            "2",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"left":{"text":"same evidence"},"right":{"text":"same evidence"}}"#,
    )
    .expect("the run");

    assert_eq!(output.status.code(), Some(0));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let asked = keys(listener.url(), &requests[0].body);
    assert_eq!(asked.len(), 1, "the equal question is asked once");
    let duplicated = format!(r#""requests":["{0}","{0}"]"#, asked[0]);
    let row = String::from_utf8_lossy(&output.stdout);
    assert!(row.contains(&duplicated), "{row}");
}
