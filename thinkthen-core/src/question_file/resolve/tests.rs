//! The precedence rule, over a file, over a command line, and over both.

use proptest::prelude::Strategy;
use proptest::{prop_assert_eq, proptest};
use serde::Serialize;

use super::{Resolved, Typed, resolve};
use crate::adapters::built_in::encode;
use crate::plan::Plan;
use crate::question::{LabelsError, Question};
use crate::question_file::{Described, QuestionFile, QuestionFileError as Refused, Source, Verb};
use crate::render::json_line;
use crate::text::Evidence;

fn file(text: &str) -> QuestionFile {
    QuestionFile::parse(text).expect("a question file")
}

fn typed_labels(values: &[&str]) -> Option<Described> {
    Some(
        values
            .iter()
            .map(|name| ((*name).to_owned(), None))
            .collect(),
    )
}

#[test]
fn a_file_of_each_verb_holds_the_settings_it_names() {
    let decide = file(
        r#"{"decide":"Does this ask for a refund?","true":"Money back.","false":"Anything else.","threshold":"0.2:0.8","on":"/body","model":"jev-1.13.0"}"#,
    );
    assert_eq!(decide.verb(), Verb::Decide);
    let resolved =
        resolve(Verb::Decide, None, Some(&decide), &Typed::default()).expect("a resolved question");
    assert_eq!(
        json_line(resolved.question().expect("a question")).expect("a question is writable"),
        concat!(
            r#"{"verb":"decide","text":"Does this ask for a refund?","#,
            r#""true":"Money back.","false":"Anything else."}"#,
        )
    );
    assert_eq!(resolved.model().as_str(), "jev-1.13.0");
    assert_eq!(resolved.on().len(), 1);
    assert_eq!(
        resolved.threshold().map(|rule| rule.to_string()),
        Some("0.2:0.8".to_owned())
    );

    let choose = file(
        r#"{"choose":"Which team?","options":{"billing":"Money.","other":null},"threshold":0.8}"#,
    );
    let resolved =
        resolve(Verb::Choose, None, Some(&choose), &Typed::default()).expect("a resolved pick");
    assert!(matches!(resolved.question(), Some(Question::Choose { .. })));
    assert!(resolved.threshold().expect("a cut").is_cut());

    let tag = file(r#"{"tag":"Which topics?","labels":{"billing":"Money.","urgent":null}}"#);
    let resolved = resolve(Verb::Tag, None, Some(&tag), &Typed::default()).expect("a resolved tag");
    assert!(matches!(resolved.question(), Some(Question::Tag { .. })));
    assert_eq!(
        json_line(resolved.question().expect("a question")).expect("a question is writable"),
        r#"{"verb":"tag","text":"Which topics?","labels":["billing","urgent"]}"#
    );
    assert_eq!(
        resolved.threshold().map(|rule| rule.to_string()).as_deref(),
        Some("0.5")
    );

    let score = file(r#"{"score":"How much?","levels":["None.","Some."]}"#);
    let resolved =
        resolve(Verb::Score, None, Some(&score), &Typed::default()).expect("a resolved placement");
    assert!(matches!(resolved.question(), Some(Question::Score { .. })));
    assert_eq!(resolved.threshold(), None);
}

#[test]
fn a_command_that_does_not_match_the_file_is_refused_by_name() {
    let held = file(r#"{"choose":"Which team?","options":["a","b"]}"#);
    assert_eq!(
        resolve(Verb::Decide, None, Some(&held), &Typed::default()),
        Err(Refused::VerbMismatch {
            asked: Verb::Decide,
            held: Verb::Choose,
        })
    );
    assert_eq!(
        resolve(Verb::Decide, None, Some(&held), &Typed::default())
            .expect_err("a refusal")
            .to_string(),
        "the command is `decide` and the question file holds a `choose` question"
    );
}

#[test]
fn the_command_line_beats_the_file_and_the_file_beats_the_default() {
    let held = file(
        r#"{"decide":"Does this ask for a refund?","true":"Money back.","threshold":"0.2:0.8","on":"/body","model":"jev-1.13.0"}"#,
    );
    let typed = Typed {
        threshold: Some("0.9".to_owned()),
        yes: Some("Cash back.".to_owned()),
        model: Some("jev-1.14.0".to_owned()),
        on: Some(vec!["/note".to_owned()]),
        ..Typed::default()
    };
    let resolved = resolve(Verb::Decide, None, Some(&held), &typed).expect("a resolved question");
    assert_eq!(
        json_line(resolved.question().expect("a question")).expect("a question is writable"),
        r#"{"verb":"decide","text":"Does this ask for a refund?","true":"Cash back."}"#
    );
    assert_eq!(resolved.model().as_str(), "jev-1.14.0");
    assert_eq!(resolved.on()[0].as_str(), "/note");
    assert_eq!(
        json_line(resolved.sources()).expect("sources are writable"),
        concat!(
            r#"{"question":"file","true":"command line","false":"default","#,
            r#""threshold":"command line","on":"command line","model":"command line"}"#,
        )
    );

    let alone =
        resolve(Verb::Decide, None, Some(&held), &Typed::default()).expect("a resolved question");
    assert_eq!(
        json_line(alone.sources()).expect("sources are writable"),
        concat!(
            r#"{"question":"file","true":"file","false":"default","#,
            r#""threshold":"file","on":"file","model":"file"}"#,
        )
    );
}

#[test]
fn a_typed_list_replaces_the_files_whole_list_and_never_merges_with_it() {
    let held = file(
        r#"{"choose":"Which team?","options":{"billing":"Money.","shipping":"Parcels.","other":null}}"#,
    );
    let typed = Typed {
        labels: typed_labels(&["urgent", "later"]),
        ..Typed::default()
    };
    let resolved = resolve(Verb::Choose, None, Some(&held), &typed).expect("a resolved pick");
    assert_eq!(
        json_line(resolved.question().expect("a question")).expect("a question is writable"),
        r#"{"verb":"choose","text":"Which team?","options":["urgent","later"]}"#
    );
    assert_eq!(
        json_line(resolved.sources()).expect("sources are writable"),
        concat!(
            r#"{"question":"file","options":"command line","threshold":"default","#,
            r#""on":"default","model":"default"}"#,
        )
    );
}

#[test]
fn a_question_with_no_file_reads_every_setting_from_the_command_line() {
    let typed = Typed {
        labels: typed_labels(&["bug", "other"]),
        threshold: Some("0.8".to_owned()),
        ..Typed::default()
    };
    let resolved =
        resolve(Verb::Choose, Some("Which kind?"), None, &typed).expect("a resolved pick");
    assert_eq!(resolved.model().as_str(), "jev-latest");
    assert!(resolved.on().is_empty());
    assert_eq!(
        json_line(resolved.sources()).expect("sources are writable"),
        concat!(
            r#"{"question":"command line","options":"command line","#,
            r#""threshold":"command line","on":"default","model":"default"}"#,
        )
    );
}

#[test]
fn a_band_on_a_pick_is_refused_from_either_source() {
    let held = file(r#"{"choose":"Which team?","options":["a","b"],"threshold":"0.1:0.9"}"#);
    assert_eq!(
        resolve(Verb::Choose, None, Some(&held), &Typed::default()),
        Err(Refused::BandOnChoose(Source::File))
    );
    let typed = Typed {
        labels: typed_labels(&["a", "b"]),
        threshold: Some("0.1:0.9".to_owned()),
        ..Typed::default()
    };
    let refusal =
        resolve(Verb::Choose, Some("Which team?"), None, &typed).expect_err("a refused band");
    assert_eq!(refusal, Refused::BandOnChoose(Source::CommandLine));
    assert_eq!(
        refusal.to_string(),
        "--threshold: `choose` takes a single cut and never a band"
    );
    assert_eq!(refusal.origin(), Source::CommandLine);
}

#[test]
fn a_list_the_verb_does_not_take_names_the_source_it_came_from() {
    let held = file(r#"{"choose":"Which team?","options":["only"]}"#);
    let refusal =
        resolve(Verb::Choose, None, Some(&held), &Typed::default()).expect_err("a refused list");
    assert_eq!(
        refusal,
        Refused::Labels {
            origin: Source::File,
            key: "options",
            error: LabelsError::OptionCount,
        }
    );
    assert_eq!(
        refusal.to_string(),
        "the question file's `options`: `choose` takes 2 to 255 options"
    );
    assert_eq!(refusal.origin(), Source::File);

    let typed = Typed {
        labels: typed_labels(&["only"]),
        ..Typed::default()
    };
    let refusal =
        resolve(Verb::Choose, Some("Which team?"), None, &typed).expect_err("a refused list");
    assert_eq!(refusal.to_string(), "`choose` takes 2 to 255 options");
    assert_eq!(refusal.origin(), Source::CommandLine);
}

#[test]
fn a_label_holding_a_control_character_is_refused_from_a_file_too() {
    let held = file(r#"{"choose":"Which team?","options":["other","bug\nrm -rf /"]}"#);
    let refusal =
        resolve(Verb::Choose, None, Some(&held), &Typed::default()).expect_err("a refused list");
    assert_eq!(
        refusal,
        Refused::Labels {
            origin: Source::File,
            key: "options",
            error: LabelsError::OptionControl,
        }
    );
    let said = refusal.to_string();
    assert!(!said.contains("rm -rf"), "{said}");
    assert!(!said.contains('\n'), "{said:?}");
}

#[test]
fn options_that_each_record_carries_settle_every_other_setting_once() {
    let held = file(r#"{"choose":"Which code fits?","options":["a","b"],"threshold":0.8}"#);
    let typed = Typed {
        options_from_record: true,
        ..Typed::default()
    };
    let resolved = resolve(Verb::Choose, None, Some(&held), &typed).expect("a resolved pick");
    assert_eq!(resolved.question(), None);
    assert_eq!(resolved.text().as_str(), "Which code fits?");
    assert!(resolved.threshold().expect("a cut").is_cut());
    assert_eq!(
        json_line(resolved.sources()).expect("sources are writable"),
        concat!(
            r#"{"question":"file","options":"command line","threshold":"file","#,
            r#""on":"default","model":"default"}"#,
        )
    );
}

#[test]
fn a_file_may_be_read_with_no_command_line_at_all() {
    let held = file(r#"{"score":"How much?","levels":["None.","Some.","Blocked."]}"#);
    let resolved =
        resolve(Verb::Score, None, Some(&held), &Typed::default()).expect("a resolved placement");
    assert_eq!(
        json_line(resolved.question().expect("a question")).expect("a question is writable"),
        r#"{"verb":"score","text":"How much?","levels":["None.","Some.","Blocked."]}"#
    );
    assert_eq!(
        json_line(resolved.sources()).expect("sources are writable"),
        r#"{"question":"file","levels":"file","on":"default","model":"default"}"#
    );
}

#[test]
fn a_typed_question_and_the_same_question_in_a_file_ask_one_thing() {
    let held = file(r#"{"decide":"Does this ask for a refund?","threshold":0.8}"#);
    let from_file =
        resolve(Verb::Decide, None, Some(&held), &Typed::default()).expect("a resolved question");
    let typed = Typed {
        threshold: Some("0.8".to_owned()),
        ..Typed::default()
    };
    let from_line = resolve(
        Verb::Decide,
        Some("Does this ask for a refund?"),
        None,
        &typed,
    )
    .expect("a resolved question");
    assert_eq!(from_file.question(), from_line.question());
    assert_eq!(from_file.text(), from_line.text());
    assert_eq!(from_file.threshold(), from_line.threshold());
}

/// One question file with the three text settings `decide` reads.
#[derive(Serialize)]
struct Written<'a> {
    decide: &'a str,
    #[serde(rename = "true", skip_serializing_if = "Option::is_none")]
    yes: Option<&'a str>,
    #[serde(rename = "false", skip_serializing_if = "Option::is_none")]
    no: Option<&'a str>,
}

/// The request body this resolved question asks over one fixed evidence.
fn request(settled: &Resolved) -> Vec<u8> {
    let question = settled.question().expect("a question").clone();
    let plan = Plan::new(
        Evidence::new("Refund me please.").expect("not blank"),
        settled.model().clone(),
        vec![question],
    )
    .expect("one question is a plan");
    encode(&plan).expect("a plan is writable")
}

/// Text that is not blank and holds no control character.
fn saying() -> impl Strategy<Value = String> {
    "[^\\p{Cc}]{1,24}".prop_filter("not blank", |text| !text.trim().is_empty())
}

proptest! {
    /// The two homes are one question, whatever the texts hold.
    ///
    /// The file is written by the same JSON writer the tool prints with, so a
    /// quotation mark, a backslash, and every letter outside ASCII make the
    /// round trip that a hand-written fixture would miss.
    #[test]
    fn a_question_typed_and_the_same_question_in_a_file_send_one_request(
        text in saying(),
        yes in proptest::option::of(saying()),
        no in proptest::option::of(saying()),
    ) {
        let written = json_line(&Written {
            decide: &text,
            yes: yes.as_deref(),
            no: no.as_deref(),
        })
        .expect("a question file is writable");
        let held = QuestionFile::parse(&written).expect("a question file");
        let from_file = resolve(Verb::Decide, None, Some(&held), &Typed::default())
            .expect("a resolved question");
        let from_line = resolve(
            Verb::Decide,
            Some(&text),
            None,
            &Typed {
                yes: yes.clone(),
                no: no.clone(),
                ..Typed::default()
            },
        )
        .expect("a resolved question");

        prop_assert_eq!(request(&from_file), request(&from_line));
    }
}
