//! Ticket 0399: a saved extra-field envelope converts and replays without sends.
use super::support::{Home, Proxy, listener, said};
use serde::Deserialize;
use serde_json::value::RawValue;

#[derive(Deserialize)]
struct Planned {
    url: String,
    request: Box<RawValue>,
}

#[test]
fn saved_openrouter_reply_replays_like_the_system_one_fixture_without_sending() {
    let target = listener();
    let proxy = Proxy::start();
    let home = Home::new("saved-reply-0399");
    let input = home.evidence(&["The parcel arrived on Tuesday and the box was intact."]);
    let flags = [
        "decide",
        "Did the parcel arrive undamaged?",
        "--input",
        &input,
        "--backend",
        "openrouter",
        "--url",
        target.base(),
        "--no-cache",
    ];
    let preview = home.run(&[flags.as_slice(), &["--plan"]].concat(), &[]);
    assert_eq!(preview.status.code(), Some(0), "{}", said(&preview).1);
    let planned: Planned = serde_json::from_str(said(&preview).0.lines().next().expect("plan"))
        .expect("one planned request");
    let responses = [
        include_str!("../../../../../specification/fixtures/backend-0399/openrouter.response.json"),
        include_str!("../../../../../specification/fixtures/systemone/decide-urgent.response.json"),
    ];
    let mut answers = Vec::new();
    for (index, response) in responses.into_iter().enumerate() {
        let recording = home.root.join(format!("saved-{index}"));
        crate::support::plant_recording(
            &recording,
            &planned.url,
            planned.request.get().as_bytes(),
            response,
        )
        .expect("saved response envelope");
        let folder = recording.to_str().expect("recording path");
        let converted = home.run(&["cache", "convert", folder], &[]);
        assert_eq!(converted.status.code(), Some(0), "{}", said(&converted).1);
        let replayed = home.run(
            &[flags.as_slice(), &["--replay", folder]].concat(),
            &[
                ("OPENROUTER_API_KEY", ""),
                ("THINKTHEN_API_KEY", ""),
                ("HTTPS_PROXY", &proxy.url),
            ],
        );
        assert_eq!(replayed.status.code(), Some(0), "{}", said(&replayed).1);
        assert_eq!(said(&replayed).1, "");
        answers.push(said(&replayed).0);
        assert_eq!(target.count(), 0);
        assert_eq!(proxy.count(), 0);
    }
    assert_eq!(answers, ["true\n", "true\n"]);
    home.assert_no_marker_in_files();
}
