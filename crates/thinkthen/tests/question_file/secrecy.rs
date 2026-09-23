//! No channel ever repeats the key or the evidence, on any path either home opens.

use crate::grammar::REFUSALS;
use crate::harness::{CLOSED, refused, run_with, written};

/// The key a secrecy case sets, which no channel may ever repeat.
const SECRET: &str = "sk-never-printed";

/// The evidence a secrecy case sends, which no channel may ever repeat.
const PRIVATE: &str = "The customer wrote something private.";

#[test]
fn no_message_from_either_home_ever_carries_the_key_or_the_evidence() {
    let mut cases: Vec<Vec<String>> = Vec::new();
    // Every refusal the grammar makes, over a real key and real evidence.
    for (name, verb, text, _) in REFUSALS {
        let file = written(name, text);
        cases.push(vec![
            verb.to_owned(),
            file,
            "--url".to_owned(),
            CLOSED.to_owned(),
        ]);
    }
    // The paths the grammar never reaches: an unopenable file, a command that
    // names the wrong verb, the two `--option` refusals, and a question file
    // that is good and whose backend is not there.
    let good = written("secrecy-good", r#"{"decide":"asks for a refund"}"#);
    cases.push(vec![
        "decide".to_owned(),
        "@no-such-question-file.json".to_owned(),
        "--url".to_owned(),
        CLOSED.to_owned(),
    ]);
    cases.push(vec![
        "choose".to_owned(),
        good.clone(),
        "billing".to_owned(),
        "other".to_owned(),
    ]);
    cases.push(vec![
        "choose".to_owned(),
        "which team owns this".to_owned(),
        "billing".to_owned(),
        "other".to_owned(),
        "--option".to_owned(),
        "sales=New business.".to_owned(),
    ]);
    cases.push(vec![
        "choose".to_owned(),
        "which team owns this".to_owned(),
        "--option".to_owned(),
        "sales".to_owned(),
    ]);
    cases.push(vec![
        "decide".to_owned(),
        good,
        "--url".to_owned(),
        CLOSED.to_owned(),
    ]);

    for case in cases {
        let arguments: Vec<&str> = case.iter().map(String::as_str).collect();
        let Ok(output) = run_with(&arguments, PRIVATE.as_bytes(), SECRET) else {
            panic!("the compiled binary runs");
        };
        for channel in [&output.stderr, &output.stdout] {
            let said = String::from_utf8_lossy(channel);
            assert!(!said.contains(SECRET), "{arguments:?}: {said}");
            assert!(!said.contains(PRIVATE), "{arguments:?}: {said}");
            assert!(
                !said.to_lowercase().contains("bearer"),
                "{arguments:?}: {said}"
            );
        }
        assert!(
            matches!(output.status.code(), Some(2 | 4 | 5)),
            "{arguments:?}: {:?}",
            output.status.code()
        );
    }
}

#[test]
fn a_refusal_names_the_key_and_never_the_description_it_held() {
    let marker = "a private instruction never echoed";
    let file = written(
        "secrecy-structured",
        &format!(r#"{{"choose":"a","options":{{"x":{{"note":"{marker}"}},"y":true}}}}"#),
    );
    let (stderr, code) = refused(&["choose", &file, "--dry-run"]);
    assert_eq!(
        stderr,
        "thinkthen: `options` in the question file is a list of labels, or a map from each label to its description\n"
    );
    assert_eq!(code, Some(5));
    assert!(!stderr.contains(marker), "{stderr}");
}
