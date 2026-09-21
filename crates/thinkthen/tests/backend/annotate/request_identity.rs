use super::scheduling::{folder, grouped, grouped_input, yes};
use super::set;
use crate::harness::{Canned, Listener, spawn};
use thinkthen_core::{Url, recording::Exchange as Recorded};

#[test]
fn detailed_requests_follow_group_order_when_groups_finish_in_reverse() {
    let answer = yes("local-1", 10, 2);
    let listener = Listener::answering(move |body| {
        let delay = if String::from_utf8_lossy(body).contains("group 0") {
            40
        } else {
            5
        };
        Canned::ok(&answer).after(delay)
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
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        grouped_input(1, 2).as_bytes(),
    )
    .expect("the run");

    assert_eq!(output.status.code(), Some(0));
    let row = String::from_utf8_lossy(&output.stdout);
    let url = Url::new(listener.url()).expect("the listener URL is valid");
    let requests = listener.requests();
    let digest_for = |group: &str| {
        let request = requests
            .iter()
            .find(|request| String::from_utf8_lossy(&request.body).contains(group))
            .expect("the group was requested");
        Recorded::new(&url, &request.body)
            .digest()
            .as_str()
            .to_owned()
    };
    let ordered = format!(
        r#""requests":["{}","{}"]"#,
        digest_for("group 0"),
        digest_for("group 1")
    );
    assert!(row.contains(&ordered), "{row}");
}

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
    let answer = yes("local-1", 10, 2);
    let listener = Listener::answering(move |_| Canned::ok(&answer).after(20)).expect("a listener");
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
    let request = requests.first().expect("one request").body.clone();
    let url = Url::new(listener.url()).expect("the listener URL is valid");
    let digest = Recorded::new(&url, &request).digest();
    let duplicated = format!(r#""requests":["{0}","{0}"]"#, digest.as_str());
    let row = String::from_utf8_lossy(&output.stdout);
    assert!(row.contains(&duplicated), "{row}");
}
