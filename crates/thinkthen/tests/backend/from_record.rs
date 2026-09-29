//! The compiled binary taking each record's candidate list from the record.

use std::io;
use std::process::Output;

use crate::harness::{Canned, Listener, spawn};

/// The question every case on this page asks.
const QUESTION: &str = "Which of these codes fits the note?";

/// Two records, each carrying its own list of codes under `codes`.
const NOTES: &str = concat!(
    "{\"id\":\"N-1\",\"note\":\"The parcel arrived on Friday, three days late.\",",
    "\"codes\":[\"late\",\"lost\"]}\n",
    "{\"id\":\"N-2\",\"note\":\"The box was empty when it reached us.\",",
    "\"codes\":[\"empty\",\"damaged\",\"other\"]}\n",
);

/// Two records whose codes carry a description each.
const DESCRIBED: &str = concat!(
    "{\"id\":\"N-1\",\"note\":\"The parcel arrived on Friday, three days late.\",",
    "\"codes\":{\"late\":\"It came after the promised day.\",",
    "\"lost\":\"It never came at all.\"}}\n",
);

/// The answer the listener gives, giving the first option the winning share.
fn picked(options: &[&str]) -> String {
    let odds: Vec<String> = options
        .iter()
        .enumerate()
        .map(|(place, option)| {
            let share = if place == 0 {
                0.9
            } else {
                0.1 / (options.len() - 1) as f64
            };
            format!("\"{option}\":{share}")
        })
        .collect();
    format!(
        concat!(
            r#"{{"model":"jev-1.13.0","answers":{{"q1":{{"type":"choice","choice":"{first}","#,
            r#""probabilities":{{{odds}}}}}}},"#,
            r#""usage":{{"input_tokens":88,"output_tokens":12}}}}"#,
        ),
        first = options.first().copied().unwrap_or_default(),
        odds = odds.join(",")
    )
}

/// Run `choose` against one URL over the records on standard input.
///
/// One request at a time, because the listener answers in the order it was
/// asked and each case here pins which record carried which options.
fn choose(base: &str, arguments: &[&str], input: &str) -> io::Result<Output> {
    let asked = [
        "choose", QUESTION, "--url", base, "--model", "local-1", "--jobs", "1",
    ];
    spawn(
        &[&asked[..], arguments].concat(),
        &[
            ("THINKTHEN_API_KEY", "sk-test-value"),
            ("THINKTHEN_BATCH", "1"),
        ],
        input.as_bytes(),
    )
}

/// What the run printed on standard output.
fn printed(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// What the run said on standard error.
fn said(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The request bodies the listener read, in the order it read them.
fn bodies(listener: &Listener) -> Vec<String> {
    listener
        .requests()
        .iter()
        .map(|request| String::from_utf8_lossy(&request.body).into_owned())
        .collect()
}

#[test]
fn a_list_in_each_record_becomes_that_record_s_own_options() {
    let listener = Listener::serving(vec![
        Canned::ok(&picked(&["late", "lost"])),
        Canned::ok(&picked(&["empty", "damaged", "other"])),
    ])
    .expect("a loopback listener");
    let output = choose(
        listener.base(),
        &["--jsonl", "--field", "/note", "--options", "/codes"],
        NOTES,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        printed(&output),
        concat!(
            r#"{"input":{"id":"N-1","note":"The parcel arrived on Friday, three days late.","codes":["late","lost"]},"value":"late"}"#,
            "\n",
            r#"{"input":{"id":"N-2","note":"The box was empty when it reached us.","codes":["empty","damaged","other"]},"value":"empty"}"#,
            "\n",
        )
    );
    let sent = bodies(&listener);
    assert!(
        sent[0].contains(r#""criteria":{"late":null,"lost":null}"#),
        "{}",
        sent[0]
    );
    assert!(
        sent[1].contains(r#""criteria":{"empty":null,"damaged":null,"other":null}"#),
        "{}",
        sent[1]
    );
    // Only the pointed value is the evidence, and --options never changes it.
    assert!(
        sent[0].contains(r#""state":"The parcel arrived on Friday, three days late.""#),
        "{}",
        sent[0]
    );
}

#[test]
fn a_structured_file_text_pairs_with_each_records_string_options() {
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("from-record");
    std::fs::create_dir_all(&folder).expect("a scratch folder");
    let file = folder.join("structured-options.json");
    std::fs::write(
        &file,
        r#"{"choose":{"ask":"Which of these codes fits the note?"},"on":"/note"}"#,
    )
    .expect("a question file");

    let listener =
        Listener::serving(vec![Canned::ok(&picked(&["late", "lost"]))]).expect("a listener");
    let output = spawn(
        &[
            "choose",
            &format!("@{}", file.to_string_lossy()),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jsonl",
            "--options",
            "/codes",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        NOTES.split_once('\n').expect("a first line").0.as_bytes(),
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    let sent = bodies(&listener);
    assert_eq!(sent.len(), 1);
    assert!(
        sent[0].contains(r#""instructions":{"ask":"Which of these codes fits the note?"}"#),
        "{}",
        sent[0]
    );
    assert!(
        sent[0].contains(r#""criteria":{"late":null,"lost":null}"#),
        "{}",
        sent[0]
    );
}

#[test]
fn a_map_in_a_record_sends_each_description_under_its_own_option() {
    let listener = Listener::serving(vec![Canned::ok(&picked(&["late", "lost"]))])
        .expect("a loopback listener");
    let output = choose(
        listener.base(),
        &["--jsonl", "--field", "/note", "--options", "/codes"],
        DESCRIBED,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        printed(&output),
        concat!(
            r#"{"input":{"id":"N-1","note":"The parcel arrived on Friday, three days late.","codes":{"late":"It came after the promised day.","lost":"It never came at all."}},"value":"late"}"#,
            "\n",
        )
    );
    let sent = bodies(&listener);
    assert!(
        sent[0].contains(concat!(
            r#""criteria":{"late":"It came after the promised day.","#,
            r#""lost":"It never came at all."}"#
        )),
        "{}",
        sent[0]
    );
}

#[test]
fn a_candidate_list_the_verb_refuses_stops_the_run_and_sends_nothing_for_itself() {
    let good = concat!(
        "{\"id\":\"N-1\",\"note\":\"The parcel arrived late.\",",
        "\"codes\":[\"late\",\"lost\"]}\n",
    );
    let cases: [(&str, &str); 5] = [
        (
            "{\"note\":\"n\",\"codes\":[\"only\"]}\n",
            "2 to 255 options",
        ),
        (
            "{\"note\":\"n\",\"codes\":[\"late\",\"late\"]}\n",
            "each option once",
        ),
        (
            "{\"note\":\"n\",\"codes\":[\"late\",\"  \"]}\n",
            "text, not white space",
        ),
        (
            "{\"note\":\"n\",\"codes\":[\"late\",7]}\n",
            "a list of labels or a map",
        ),
        (
            "{\"note\":\"n\",\"other\":[]}\n",
            "holds nothing at `/codes`",
        ),
    ];

    for (bad, said_part) in cases {
        let listener = Listener::serving(vec![
            Canned::ok(&picked(&["late", "lost"])),
            Canned::ok(&picked(&["late", "lost"])),
        ])
        .expect("a loopback listener");
        let input = format!("{good}{bad}");
        let output = choose(
            listener.base(),
            &["--jsonl", "--field", "/note", "--options", "/codes"],
            &input,
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{bad}");
        assert_eq!(
            printed(&output),
            concat!(
                r#"{"input":{"id":"N-1","note":"The parcel arrived late.","codes":["late","lost"]},"value":"late"}"#,
                "\n",
            ),
            "{bad}"
        );
        assert_eq!(listener.requests().len(), 1, "{bad}");
        let message = said(&output);
        assert!(message.contains(said_part), "{message}");
        assert!(
            message.contains("stopped at record 2; 1 record finished"),
            "{message}"
        );
        assert!(!message.contains("parcel"), "{message}");
        assert!(!message.contains("from a recording"), "{message}");
        assert!(!message.contains("rm -rf"), "{message}");
    }
}

#[test]
fn a_typed_option_holding_a_control_character_is_refused_before_any_request() {
    // `--raw` prints a label byte for byte, so a label carrying a line feed
    // would write a line of its own into the caller's output.
    let listener =
        Listener::serving(vec![Canned::ok(&picked(&["late", "lost"]))]).expect("a listener");
    let output = spawn(
        &[
            "choose",
            QUESTION,
            "late",
            "lost\nrm -rf /",
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--raw",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The parcel arrived late.",
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
    assert!(printed(&output).is_empty());
    let message = said(&output);
    assert!(message.contains("one line of printable text"), "{message}");
    assert!(!message.contains("rm -rf"), "{message}");
    assert!(listener.requests().is_empty());
}

#[test]
fn options_from_the_record_cannot_act_beside_a_typed_list_or_outside_jsonl() {
    let cases: [(&[&str], &str); 3] = [
        (
            &["late", "lost", "--jsonl", "--field", "/note"],
            "--options takes the options from each record",
        ),
        (
            &["--lines", "--options", "/codes"],
            "--options needs --jsonl",
        ),
        (&["--options", "/codes"], "--options needs --jsonl"),
    ];

    for (arguments, said_part) in cases {
        let listener = Listener::serving(vec![Canned::ok(&picked(&["late", "lost"]))])
            .expect("a loopback listener");
        let mut asked = arguments.to_vec();
        if !asked.contains(&"/codes") {
            asked.extend_from_slice(&["--options", "/codes"]);
        }
        let output = choose(listener.base(), &asked, NOTES).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{asked:?}");
        assert!(printed(&output).is_empty(), "{asked:?}");
        assert!(said(&output).contains(said_part), "{}", said(&output));
        assert!(listener.requests().is_empty(), "{asked:?}");
    }
}

#[test]
fn a_row_names_the_options_it_was_asked_with_and_the_plan_names_the_first_record_s() {
    let listener = Listener::serving(vec![
        Canned::ok(&picked(&["late", "lost"])),
        Canned::ok(&picked(&["empty", "damaged", "other"])),
    ])
    .expect("a loopback listener");
    let output = choose(
        listener.base(),
        &[
            "--jsonl",
            "--field",
            "/note",
            "--options",
            "/codes",
            "--details",
        ],
        NOTES,
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let rows = printed(&output);
    let mut lines = rows.lines();
    assert!(
        lines
            .next()
            .expect("a first row")
            .contains(r#""options":["late","lost"]"#),
        "{rows}"
    );
    assert!(
        lines
            .next()
            .expect("a second row")
            .contains(r#""options":["empty","damaged","other"]"#),
        "{rows}"
    );

    let output = spawn(
        &[
            "choose",
            QUESTION,
            "--jsonl",
            "--field",
            "/note",
            "--options",
            "/codes",
            "--plan",
        ],
        &[],
        NOTES.as_bytes(),
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(0));
    assert!(
        printed(&output).contains(r#""criteria":{"late":null,"lost":null}"#),
        "{}",
        printed(&output)
    );
}
