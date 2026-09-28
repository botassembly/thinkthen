//! The 0095 members a binding reads, through the public API alone.
//!
//! Runtime labels build the question their file builds and ask what the
//! typed call asks. The spec readers keep the parser's rules and split a
//! broken rule into `Usage` from text and `Local` from a file. Kinds read in
//! set order, a bulk row carries the yes probability `details` reads, and each
//! error kind names its word in the shared cases.

#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use std::path::PathBuf;

use conformance_backend::{Backend, Canned, Listener};
use thinkthen::{
    Engine, ErrorKind, Judgment, LoadedQuestion, Probabilities, Question, QuestionKind,
    QuestionSet, Recognize, Relate, RelationRule,
};

const CASES: &str = include_str!("../../../conformance/cases.json");

thinkthen::choices! {
    /// The teams the runtime labels name.
    enum Team { Billing => "billing", Outage => "outage" }
}

fn engine(base: &str) -> Engine {
    Engine::builder()
        .base_url(base)
        .and_then(|builder| builder.api_key("sk-public-members"))
        .map(thinkthen::EngineBuilder::no_cache)
        .and_then(thinkthen::EngineBuilder::build)
        .expect("engine")
}

fn loaded(text: &str) -> Question {
    match Question::from_json(text).expect("question file") {
        LoadedQuestion::Question(question) => Some(question),
        LoadedQuestion::Banded(_) => None,
    }
    .expect("a file with no band")
}

#[test]
fn a_saved_calibration_name_reaches_the_public_digest_and_warning() {
    let case: serde_json::Value =
        serde_json::from_str(include_str!("../../../conformance/calibration.json"))
            .expect("shared calibration case");
    const ANSWER: &str = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}}}"#;
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let question = loaded(&case["question"].to_string());
    let refused = QuestionSet::builder()
        .question("answer", question.clone())
        .expect_err("a member cannot silently lose its saved profile");
    assert_eq!(refused.kind(), ErrorKind::Usage);
    assert_eq!(
        refused.to_string(),
        "a question set member takes no profile; name it on the set"
    );
    assert_eq!(listener.count(), 0);
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-profile-local")
        .expect("key")
        .profile_json(&case["runtime_profile"].to_string())
        .expect("running profile")
        .no_cache()
        .build()
        .expect("engine");
    let details = engine
        .details(&question, case["evidence"].as_str().expect("evidence"))
        .expect("details");
    assert_eq!(listener.count(), 1);
    assert_eq!(details.value().question_sha256(), case["question_sha256"]);
    assert_eq!(details.value().profile_warning(), Some(("old", "new")));
    let row: serde_json::Value =
        serde_json::from_str(&details.value().to_json()).expect("result JSON");
    assert_eq!(row["meta"]["question_sha256"], case["question_sha256"]);
    assert_eq!(row["meta"]["profile_warning"], case["warning"]);
    assert_eq!(row["meta"]["model"], case["model"]);
}

#[test]
fn a_runtime_profile_limit_refuses_the_saved_question_before_sending() {
    let listener = Listener::answering(|_| Canned::ok("{}")).expect("listener");
    let question = loaded(r#"{"decide":"Is this a request?","profile":"old"}"#);
    let engine = Engine::builder()
        .base_url(listener.base())
        .expect("base")
        .api_key("sk-profile-local")
        .expect("key")
        .profile_json(
            r#"{"schema":"thinkthen.backend-profile/1","name":"new","max_evidence_bytes":1}"#,
        )
        .expect("running profile")
        .no_cache()
        .build()
        .expect("engine");
    let error = engine.details(&question, "two").expect_err("profile limit");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(
        error
            .to_string()
            .contains("allows at most 1 evidence bytes")
    );
    assert_eq!(listener.count(), 0);
}

fn message<T: std::fmt::Debug>(result: Result<T, thinkthen::Error>) -> (ErrorKind, String) {
    let error = result.expect_err("a refusal");
    (error.kind(), error.to_string())
}

#[test]
fn runtime_labels_build_the_files_question_and_refuse_a_repeat_at_its_step() {
    let described = thinkthen::Description::text("Money owed.").expect("description");
    let chosen = Question::choose_labels("Which team?")
        .and_then(|labels| labels.label("billing", Some(described)))
        .and_then(|labels| labels.label("outage", None))
        .and_then(thinkthen::LabelBuilder::build)
        .expect("choose");
    let file = r#"{"choose":"Which team?","options":{"billing":"Money owed.","outage":null}}"#;
    assert_eq!(chosen, loaded(file));
    let tagged = Question::tag_labels("Which apply?")
        .and_then(|labels| labels.label("urgent", None))
        .and_then(|labels| labels.cut_at(0.7))
        .expect("tag");
    let file = r#"{"tag":"Which apply?","labels":["urgent"],"threshold":0.7}"#;
    assert_eq!(tagged, loaded(file));
    let default_cut = Question::tag_labels("Which apply?")
        .and_then(|labels| labels.label("urgent", None))
        .and_then(thinkthen::LabelBuilder::build);
    let file = r#"{"tag":"Which apply?","labels":["urgent"]}"#;
    assert_eq!(default_cut.expect("tag"), loaded(file));
    for (start, repeated) in [
        (
            Question::choose_labels("Which team?"),
            "a list holds each option once",
        ),
        (
            Question::tag_labels("Which apply?"),
            "a list holds each label once",
        ),
    ] {
        let twice = start
            .and_then(|labels| labels.label("billing", None))
            .and_then(|labels| labels.label("billing", None));
        assert_eq!(message(twice), (ErrorKind::Usage, repeated.to_owned()));
    }
}

#[test]
fn details_over_runtime_labels_is_the_typed_call_with_one_send_each() {
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/generic/v1", backend.origin()));
    let runtime = Question::choose_labels("Which team owns this?")
        .and_then(|labels| labels.label("billing", None))
        .and_then(|labels| labels.label("outage", None))
        .and_then(thinkthen::LabelBuilder::build)
        .expect("runtime");
    let typed = runtime.clone().into_choose::<Team>().expect("bound");
    let details = engine
        .details(&runtime, "The invoice is wrong.")
        .expect("details");
    assert_eq!(
        details.value().value(),
        &Judgment::Choice(Some("billing".to_owned()))
    );
    assert_eq!(backend.count(), 1);
    let picked = engine
        .choose(&typed, "The invoice is wrong.")
        .expect("typed");
    assert_eq!(picked.into_value(), Some(Team::Billing));
    assert_eq!(backend.count(), 2);
    let bound = engine
        .details(&typed, "The invoice is wrong.")
        .expect("typed details");
    assert_eq!(bound.value().requests(), details.value().requests());
}

#[test]
fn kinds_read_in_set_order_and_a_band_reads_decide() {
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"urgent":{"decide":"Urgent?","threshold":"0.2:0.8"},"team":{"choose":"Which team?","options":["billing","outage"]},"severity":{"score":"How severe?","levels":["low","high"]},"topics":{"tag":"Which apply?","labels":["urgent"]}}}"#,
    )
    .expect("set");
    let members: Vec<(&str, QuestionKind)> = set.members().collect();
    assert_eq!(
        members,
        [
            ("urgent", QuestionKind::Decide),
            ("team", QuestionKind::Choose),
            ("severity", QuestionKind::Score),
            ("topics", QuestionKind::Tag),
        ]
    );
    assert_eq!(set.members().len(), 4);
    let asked = [
        (
            Question::rank("Relevant?").expect("rank").kind(),
            QuestionKind::Rank,
        ),
        (
            Question::find("Which one?").expect("find").kind(),
            QuestionKind::Find,
        ),
        (
            loaded(r#"{"decide":"Urgent?"}"#).kind(),
            QuestionKind::Decide,
        ),
        (
            loaded(r#"{"score":"How?","levels":["a","b"]}"#).kind(),
            QuestionKind::Score,
        ),
    ];
    for (kind, expected) in asked {
        assert_eq!(kind, expected);
    }
}

#[test]
fn spec_readers_keep_the_parsers_rules_and_split_usage_from_local() {
    let either = r#"{"version":1,"relate":{"relations":[{"name":"works_with","source":"person","target":"organization","either":true}]}}"#;
    let built = Relate::builder()
        .relation(RelationRule::both_ways("works_with", "person", "organization").expect("rule"))
        .and_then(thinkthen::RelateBuilder::build)
        .expect("built");
    // The digest reads only the compared spec, so equal specs have equal digests.
    assert_eq!(Relate::from_json(either).expect("relate file"), built);
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("public-members-specs");
    std::fs::create_dir_all(&folder).expect("folder");
    let broken_relate = r#"{"version":1,"relate":{"relations":[]}}"#;
    let broken_recognize = r#"{"version":1,"recognize":{"kinds":{"Entity":null}}}"#;
    let relate_file = folder.join("relate.json");
    let recognize_file = folder.join("recognize.json");
    std::fs::write(&relate_file, broken_relate).expect("relate file");
    std::fs::write(&recognize_file, broken_recognize).expect("recognize file");
    let (kind, relate_sentence) = message(Relate::from_json(broken_relate));
    assert_eq!(kind, ErrorKind::Usage);
    assert_eq!(
        message(Relate::load(&relate_file)),
        (ErrorKind::Local, relate_sentence)
    );
    let recognize_sentence =
        "recognize reserves the kind names none of these, ENTITY and ANY in any ASCII case"
            .to_owned();
    assert_eq!(
        message(Recognize::from_json(broken_recognize)),
        (ErrorKind::Usage, recognize_sentence.clone())
    );
    assert_eq!(
        message(Recognize::load(&recognize_file)),
        (ErrorKind::Local, recognize_sentence)
    );
    let pointed = r#"{"version":1,"relate":{"fields":{"name":"/who","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization"}]}}"#;
    assert_eq!(
        message(Relate::from_json(pointed)),
        (
            ErrorKind::Usage,
            "a library relate reads each entity's name and kind, so `fields` keeps /name and /kind"
                .to_owned()
        )
    );
    let on = r#"{"version":1,"recognize":{"kinds":{"person":"A person."}},"on":"/body"}"#;
    assert_eq!(
        message(Recognize::from_json(on)),
        (
            ErrorKind::Usage,
            "a library recognize reads its evidence whole, so it takes no `on`".to_owned()
        )
    );
}

#[test]
fn a_bulk_row_carries_the_yes_probability_details_reads() {
    let listener = Listener::answering(|body| {
        let yes = if String::from_utf8_lossy(body).contains("low") {
            "0.3"
        } else {
            "0.8"
        };
        Canned::ok(&format!(
            r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{yes}}}}}}}"#
        ))
    })
    .expect("listener");
    let engine = engine(listener.base());
    let question = Question::decide("Refund?").expect("question").cut();
    let rows = engine
        .decide_many(&question, ["a low one", "a high one"])
        .collect::<Result<Vec<_>, _>>()
        .expect("rows");
    let read: Vec<f64> = rows.iter().map(thinkthen::Row::probability).collect();
    assert_eq!(read, [0.3, 0.8]);
    for row in &rows {
        let details = engine.details(&question, row.input()).expect("details");
        let Probabilities::YesNo { yes } = details.value().probabilities() else {
            panic!("a yes or no answer");
        };
        assert_eq!(*yes, row.probability());
    }
}

#[test]
fn each_error_kind_names_its_word_in_the_shared_cases() {
    let cases: serde_json::Value = serde_json::from_str(CASES).expect("the shared cases");
    let named: Vec<&str> = [
        ErrorKind::Usage,
        ErrorKind::Backend,
        ErrorKind::Local,
        ErrorKind::Cancelled,
        ErrorKind::Deadline,
        ErrorKind::Defect,
    ]
    .map(ErrorKind::name)
    .to_vec();
    assert_eq!(serde_json::json!(named), cases["error_kinds"]);
}

#[test]
fn a_builders_debug_line_withholds_its_question_labels_and_descriptions() {
    let said = || thinkthen::Description::text("sentinel-said").expect("description");
    let asked = "sentinel-asked";
    let lines = [
        shown(Question::decide(asked).and_then(|b| b.yes(said()))),
        shown(Question::choose::<Team>(asked).and_then(|b| b.option(Team::Billing, Some(said())))),
        shown(Question::tag::<Team>(asked).and_then(|b| b.label(Team::Billing, Some(said())))),
        shown(Question::score(asked).and_then(|b| b.level("sentinel-level", Some(said())))),
        shown(Question::choose_labels(asked).and_then(|b| b.label("sentinel-label", Some(said())))),
        shown(thinkthen::Description::builder().what("sentinel-what")),
    ];
    for line in lines.iter().flatten() {
        assert!(!line.contains("sentinel"), "{line}");
    }
}

/// A builder's plain and pretty `Debug` lines.
fn shown<T: std::fmt::Debug>(built: Result<T, thinkthen::Error>) -> [String; 2] {
    let built = built.expect("builder");
    [format!("{built:?}"), format!("{built:#?}")]
}

/// A relate entity's name and kind are the caller's text, so both are withheld.
#[test]
fn a_relate_entitys_debug_line_withholds_its_name_and_kind() {
    assert_eq!(
        shown(thinkthen::Entity::new("sentinel-name", "sentinel-kind-x")),
        [
            "Entity { name: <13 bytes withheld>, kind: <15 bytes withheld> }",
            "Entity {\n    name: <13 bytes withheld>,\n    kind: <15 bytes withheld>,\n}",
        ]
    );
}
