#![cfg(test)]
//! A Rust caller gets each answer in one step, and not sure or a failure never reads as no.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
mod support;

use loopback::{KEY, Loopback};
use serde_json::json;
use thinkthen::{
    AnnotatableQuestion, Annotate, AnnotationEntry, BackendName, Choose, Decide, Engine, Filter,
    Key, Name, NoAnswer, Outcome, Probability, Question, Rank, Record, RecordId, Relation, Score,
    Settings, Span, Status, Tag, Threshold,
};

fn stub() -> Engine {
    Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap()
}

fn answering(server: &Loopback) -> Engine {
    Settings::default()
        .url(server.base())
        .api_key(Key::new(KEY).unwrap())
        .engine(|_| None)
        .unwrap()
}

#[test]
fn into_answer_gives_the_answer_and_refuses_not_sure_and_failed_rows() {
    let banded = Decide::new("Does the customer ask for a refund?")
        .unwrap()
        .threshold(Threshold::band(0.2, 0.8).unwrap());
    let sure = Loopback::noul(0.9).unwrap();
    let is_refund: bool = answering(&sure)
        .run(&banded, "Please refund my order.")
        .unwrap()
        .into_answer()
        .unwrap();
    assert!(is_refund);

    let middle = Loopback::noul(0.5).unwrap();
    let row = answering(&middle)
        .run(&banded, "Maybe send it back?")
        .unwrap();
    // Every status is named: the set is closed, so no catch-all arm is needed.
    match row.status() {
        Status::Answered | Status::Failed => panic!("expected not sure, got {row:?}"),
        Status::Unsure => {}
    }
    assert_eq!(
        row.answer().unwrap_err(),
        row.clone().into_answer().unwrap_err()
    );
    match row.into_answer() {
        Err(NoAnswer::Unsure { probability }) => assert_eq!(probability.get(), 0.5),
        Ok(answer) => panic!("not sure became {answer}"),
        Err(NoAnswer::Failed(error)) => panic!("not sure became {error}"),
    }

    let failed = stub()
        .rows(&banded, [Record::missing()])
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    match failed.into_answer() {
        Err(NoAnswer::Failed(error)) => assert_eq!(error.code().as_str(), "record.missing"),
        other => panic!("a failed record became {other:?}"),
    }
}

#[test]
fn rows_compare_and_print_as_their_answer() {
    let engine = stub();
    let team = Choose::with_strings("Which team owns this?", ["billing", "shipping"]).unwrap();
    let row = engine.run(&team, "My parcel never came.").unwrap();
    assert_eq!(row, engine.run(&team, "My parcel never came.").unwrap());
    let owner: String = row.answer().unwrap().to_owned();
    assert_eq!(row.to_string(), owner);
    assert!(!format!("{row:?}").contains("PhantomData"), "{row:?}");

    let middle = Loopback::noul(0.5).unwrap();
    let banded = Decide::new("Ready?")
        .unwrap()
        .threshold(Threshold::band(0.2, 0.8).unwrap());
    let unsure = answering(&middle).run(&banded, "Perhaps.").unwrap();
    assert_eq!(unsure.to_string(), "unsure");
    let failed = engine
        .rows(&banded, [Record::missing()])
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(
        failed.to_string(),
        "failed: record.missing: the record is missing"
    );
    assert_ne!(unsure, failed);
}

#[test]
fn run_takes_text_and_json_as_they_are() {
    let engine = stub();
    let refund = Decide::new("Does the customer ask for a refund?").unwrap();
    let text = "Please refund my order.";
    let expected = engine.run(&refund, Record::text(text)).unwrap();
    assert_eq!(engine.run(&refund, text).unwrap(), expected);
    assert_eq!(engine.run(&refund, text.to_owned()).unwrap(), expected);
    let from_json = engine.run(&refund, json!({"body": text})).unwrap();
    let from_record = engine
        .run(
            &refund,
            Record::from_json(&json!({"body": text}).to_string()),
        )
        .unwrap();
    assert_eq!(from_json, from_record);
    let rows: Vec<_> = engine
        .rows(&refund, ["Refund me.", "Thanks!"])
        .unwrap()
        .map(Result::unwrap)
        .collect();
    assert_eq!(rows.len(), 2);
}

#[test]
fn filter_yields_kept_items_before_one_failure_with_all_failed_items() {
    let server = Loopback::noul(0.9).unwrap();
    let engine = answering(&server);
    let complaint = Filter::new("Is this review a complaint?").unwrap();
    let reviews: Vec<serde_json::Value> = (0..40)
        .map(|n| {
            (n % 7 != 3)
                .then(|| format!("Review number {n}."))
                .map_or(serde_json::Value::Null, serde_json::Value::String)
        })
        .collect();
    let mut expected = Vec::new();
    let mut failed = Vec::new();
    for review in &reviews {
        if review.is_null() {
            failed.push(review.clone());
        } else {
            expected.push(review.clone());
        }
    }
    let mut results = engine.filter(&complaint, reviews).unwrap();
    let mut actual = Vec::new();
    for result in results.by_ref() {
        match result {
            Ok(review) => actual.push(review),
            Err(failure) => {
                assert_eq!(failure.error().code().as_str(), "record.missing");
                assert_eq!(
                    failure
                        .rows()
                        .iter()
                        .map(|row| row.item().clone())
                        .collect::<Vec<_>>(),
                    failed
                );
                assert!(results.next().is_none());
                break;
            }
        }
    }
    assert_eq!(actual, expected);
}

#[test]
fn typed_calls_carry_the_engine_execution_name() {
    let engine = stub().named("refund-triage").unwrap();
    let refund = Decide::new("Does the customer ask for a refund?").unwrap();
    let row = engine.run(&refund, "Refund me.").unwrap();
    assert_eq!(row.source().name(), Some("refund-triage"));
    for row in engine.rows(&refund, ["One.", "Two."]).unwrap() {
        assert_eq!(row.unwrap().source().name(), Some("refund-triage"));
    }
    assert_eq!(
        stub().run(&refund, "Refund me.").unwrap().source().name(),
        None
    );
    let error = stub().named(" ").unwrap_err();
    assert_eq!(error.code().as_str(), "usage.name");
}

#[test]
fn annotate_takes_pairs_and_refuses_a_repeated_name() {
    let refund = Decide::new("Does the customer ask for a refund?").unwrap();
    let team = Choose::with_strings("Which team owns this?", ["billing", "shipping"]).unwrap();
    let form = Annotate::new([
        ("refund", AnnotatableQuestion::from(refund.clone())),
        ("team", AnnotatableQuestion::from(team)),
    ])
    .unwrap();
    assert_eq!(
        form.questions().keys().collect::<Vec<_>>(),
        ["refund", "team"]
    );
    let triage = stub().run(&form, "Refund my lost parcel.").unwrap();
    let printed = triage.to_string();
    assert!(printed.starts_with("refund="), "{printed}");
    assert!(printed.contains("\tteam="), "{printed}");
    let error = Annotate::new([("refund", refund.clone()), ("refund", refund)]).unwrap_err();
    assert_eq!(error.code().as_str(), "usage.question");
}

#[test]
fn annotate_answers_read_one_question_in_its_own_type() {
    let refund = Decide::new("Does the customer ask for a refund?").unwrap();
    let team = Choose::with_strings("Which team owns this?", ["billing", "shipping"]).unwrap();
    let form = Annotate::new([
        ("refund", AnnotatableQuestion::from(refund)),
        ("team", team.into()),
    ])
    .unwrap();
    let answers = stub()
        .run(&form, "Refund my lost parcel.")
        .unwrap()
        .into_answer()
        .unwrap();
    let expected = match &answers["refund"] {
        AnnotationEntry::Decide(Outcome::Answered { answer, .. }) => Ok(*answer),
        AnnotationEntry::Decide(Outcome::Unsure { probability, .. }) => Err(NoAnswer::Unsure {
            probability: *probability,
        }),
        AnnotationEntry::Decide(Outcome::Failed { error }) => Err(NoAnswer::Failed(error.clone())),
        other => panic!("refund is not a decide entry: {other:?}"),
    };
    let is_refund: Result<bool, NoAnswer> = answers.decide("refund").unwrap().answer();
    assert_eq!(is_refund, expected);
    let owner: Result<String, NoAnswer> = answers.choose::<String>("team").unwrap().into_answer();
    match (&answers["team"], owner) {
        (AnnotationEntry::Choose(Outcome::Answered { answer, .. }), Ok(owner)) => {
            assert_eq!(&owner, answer);
        }
        (AnnotationEntry::Choose(Outcome::Unsure { .. }), Err(NoAnswer::Unsure { .. })) => {}
        (entry, owner) => panic!("{entry:?} read as {owner:?}"),
    }
    let mut names = Vec::new();
    for (name, _) in &answers {
        names.push(name.as_str());
    }
    assert_eq!(names, ["refund", "team"]);
    for (name, question, code) in [
        ("urgency", "decide", "usage.name"),
        ("refund", "choose", "usage.function"),
        ("team", "decide", "usage.function"),
    ] {
        let read = match question {
            "decide" => answers.decide(name).map(|_| ()),
            _ => answers.choose::<String>(name).map(|_| ()),
        };
        match read {
            Err(error) => assert_eq!(error.code().as_str(), code, "{name}"),
            other => panic!("{name} as {question} gave {other:?}"),
        }
    }
}

#[test]
fn run_takes_a_borrowed_string_and_returns_blank_text_as_a_call_error() {
    let engine = stub();
    let refund = Decide::new("Does the customer ask for a refund?").unwrap();
    let message = String::from("Please refund my order.");
    assert_eq!(
        engine.run(&refund, &message).unwrap(),
        engine.run(&refund, message.as_str()).unwrap()
    );
    let blank = String::from("   ");
    for error in [
        engine.run(&refund, "").unwrap_err(),
        engine.run(&refund, &blank).unwrap_err(),
    ] {
        assert_eq!(error.code().as_str(), "record.empty");
    }
    let rows: Vec<_> = engine
        .rows(&refund, ["Refund me.", ""])
        .unwrap()
        .map(Result::unwrap)
        .collect();
    match rows[1].answer() {
        Err(NoAnswer::Failed(error)) => assert_eq!(error.code().as_str(), "record.empty"),
        other => panic!("a blank record in rows gave {other:?}"),
    }
}

#[test]
fn record_ids_print_as_the_caller_wrote_them() {
    assert_eq!(RecordId::from(-7_i64).to_string(), "-7");
    assert_eq!(RecordId::from(u64::MAX).to_string(), u64::MAX.to_string());
    let engine = stub();
    let refund = Decide::new("Does the customer ask for a refund?")
        .unwrap()
        .field("/text".parse().unwrap());
    let ids: Vec<String> = engine
        .rows(
            &refund,
            [
                json!({"id": "a1", "text": "Refund me."}),
                json!({"id": 42, "text": "Thanks."}),
            ],
        )
        .unwrap()
        .map(|row| row.unwrap().id().unwrap().to_string())
        .collect();
    assert_eq!(ids, ["a1", "42"]);
}

#[test]
fn spans_and_probabilities_are_plain_values() {
    let found: Span =
        serde_json::from_value(json!({"text": "refund", "start": 7, "end": 13})).unwrap();
    let span: (usize, usize) = (found.start(), found.end());
    assert_eq!(span, (7, 13));
    let name: Name = serde_json::from_value(
        json!({"text": "Éva", "start": 0, "end": 4, "kind": "person", "probability": 0.9}),
    )
    .unwrap();
    let support: Probability = name.probability();
    assert_eq!((name.start(), name.end()), (0, 4));
    assert!((support.get() - 0.9).abs() < 1e-12);
    let relation: Relation = serde_json::from_value(json!({
        "source": {"text": "Éva", "start": 0, "end": 4, "kind": "person"},
        "target": {"text": "Acme", "start": 14, "end": 18, "kind": "organization"},
        "relation": "works_for",
        "probability": 0.8,
        "either": true
    }))
    .unwrap();
    let unordered: bool = relation.either();
    assert!(unordered);
    assert_eq!(relation.target().start(), 14);
    assert!((relation.probability().get() - 0.8).abs() < 1e-12);
}

#[test]
fn ranked_rows_carry_their_own_record() {
    let by_urgency =
        Rank::with_strings("How urgent is the request?", ["routine", "urgent"]).unwrap();
    let messages = ["Refund me now!", "Thanks for the help."];
    for row in stub().rank(&by_urgency, messages).unwrap() {
        let row = row.unwrap();
        assert_eq!(*row.item(), messages[row.position() as usize - 1]);
    }
}

#[test]
fn meanings_and_descriptions_reach_the_question() {
    let decide = Decide::new("Ready?")
        .unwrap()
        .yes_means("safe to proceed")
        .no_means("needs work");
    let filter = Filter::new("Ready?")
        .unwrap()
        .yes_means("safe to proceed")
        .no_means("needs work");
    for question in [Question::from(decide), Question::from(filter)] {
        let definition = serde_json::to_value(&question).unwrap();
        assert_eq!(definition["yes_means"], "safe to proceed");
        assert_eq!(definition["no_means"], "needs work");
    }
    let described = [("billing", "money owed or paid")];
    let choose = Choose::with_strings("Which team?", ["billing", "shipping"])
        .unwrap()
        .descriptions(described)
        .unwrap();
    let tag = Tag::with_strings("Which teams?", ["billing", "shipping"])
        .unwrap()
        .descriptions(described)
        .unwrap();
    let score = Score::with_strings("How urgent?", ["routine", "urgent"])
        .unwrap()
        .descriptions([("urgent", "needs an answer today")])
        .unwrap();
    let rank = Rank::with_strings("How urgent?", ["routine", "urgent"])
        .unwrap()
        .descriptions([("urgent", "needs an answer today")])
        .unwrap();
    for (question, field, expected) in [
        (
            Question::from(choose),
            "choices",
            json!([{"name": "billing", "description": "money owed or paid"}, "shipping"]),
        ),
        (
            Question::from(tag),
            "labels",
            json!([{"name": "billing", "description": "money owed or paid"}, "shipping"]),
        ),
        (
            Question::from(score),
            "levels",
            json!(["routine", {"name": "urgent", "description": "needs an answer today"}]),
        ),
        (
            Question::from(rank),
            "levels",
            json!(["routine", {"name": "urgent", "description": "needs an answer today"}]),
        ),
    ] {
        assert_eq!(serde_json::to_value(&question).unwrap()[field], expected);
    }
}

#[test]
fn an_unknown_description_name_gets_the_engine_refusal() {
    let unknown = [("sales", "selling things")];
    let refused = Choose::with_strings("Which team?", ["billing", "shipping"])
        .unwrap()
        .descriptions(unknown)
        .unwrap_err();
    let shared = Question::from_json(
        &json!({"function": "choose", "question": "Which team?", "choices": ["billing", "shipping"], "descriptions": {"sales": "selling things"}})
            .to_string(),
    )
    .unwrap_err();
    assert_eq!(refused.code(), shared.code());
    assert_eq!(refused.to_string(), shared.to_string());
    for refused in [
        Tag::with_strings("Which teams?", ["billing", "shipping"])
            .unwrap()
            .descriptions(unknown)
            .map(|_| ())
            .unwrap_err(),
        Score::with_strings("How urgent?", ["routine", "urgent"])
            .unwrap()
            .descriptions(unknown)
            .map(|_| ())
            .unwrap_err(),
        Rank::with_strings("How urgent?", ["routine", "urgent"])
            .unwrap()
            .descriptions(unknown)
            .map(|_| ())
            .unwrap_err(),
    ] {
        assert_eq!(refused.code(), shared.code());
    }
}

const CHILD: &str = "THINKTHEN_TEST_FROM_ENV_EXPECTS";

#[test]
fn engine_from_env_reads_the_backend_variable() {
    if let Some(expected) = std::env::var_os(CHILD) {
        let expected = expected.into_string().unwrap();
        match Engine::from_env() {
            Ok(engine) => assert_eq!(engine.resolved_settings().backend().as_str(), expected),
            Err(error) => assert_eq!(error.code().as_str(), expected),
        }
        return;
    }
    for (backend, expected) in [
        (Some("stub"), "stub"),
        (Some("unknown"), "usage.backend"),
        (None, "usage.key-missing"),
    ] {
        let mut child = support::isolated(std::env::current_exe().unwrap());
        child
            .args(["--exact", "engine_from_env_reads_the_backend_variable"])
            .env(CHILD, expected);
        if let Some(backend) = backend {
            child.env("THINKTHEN_BACKEND", backend);
        }
        let output = child.output().unwrap();
        let report = String::from_utf8_lossy(&output.stdout);
        assert!(output.status.success(), "{backend:?}: {report}");
        assert!(report.contains("1 passed"), "{backend:?}: {report}");
    }
}
