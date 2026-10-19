//! Native carrier secrecy assertions moved from the command failure tests.
use crate::core::recording::Exchange as Recorded;
use crate::core::{QuestionSetError, Url};
use crate::engine::http::Client;
use std::time::Duration;
const KEY: &str = "sk-marker-2f9d41c6";
const EVIDENCE: &str = "marker-evidence-7b3ac5";

#[test]
fn no_debug_line_shows_the_key_or_the_evidence() {
    let key = crate::engine::http::Key::of(KEY);
    let body = format!(r#"{{"state":"{EVIDENCE}"}}"#);
    let url = Url::new("http://127.0.0.1:1/v1/systemone").expect("an address");
    let recorded = Recorded::new(&url, body.as_bytes());
    let exchange = crate::engine::http::Exchange {
        headers: &[],
        url: url.as_str(),
        body: body.as_bytes(),
        key: &key,
        max_retries: 2,
        retry_wait: Duration::from_secs(1),
    };
    let client = Client::new(
        Duration::from_secs(1),
        false,
        &crate::engine::limits::process().widths,
    );
    // `relate` reads its evidence as entity names and kinds.
    let entity = crate::core::RelationEntity::new(EVIDENCE, EVIDENCE).expect("an entity");
    let question_set = QuestionSetError::Duplicate(format!("{KEY}.{EVIDENCE}"));
    let shown = format!("{key:?} {exchange:?} {client:?} {recorded:?} {entity:?} {question_set:?}");

    assert!(!shown.contains(KEY), "{shown}");
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert!(shown.contains("withheld"), "{shown}");
}

#[test]
fn no_record_label_request_or_answer_debug_line_shows_the_evidence() {
    use crate::core::Question;
    let record = crate::core::Reading::new(crate::core::Framing::Jsonl, Vec::new())
        .expect("a JSON reading")
        .record(format!(r#"{{"labels":["{EVIDENCE}","other"]}}"#).as_bytes())
        .expect("a JSON record");
    let labels = record
        .choices(&crate::core::Pointer::new("/labels").expect("a pointer"))
        .expect("two labels");
    let text = crate::core::QuestionText::new("Which one fits?").expect("a question");
    let questions = [
        Question::Choose {
            text: text.clone(),
            options: labels.clone(),
        },
        Question::Tag {
            text: text.clone(),
            labels: labels.clone(),
        },
        Question::Score {
            text,
            levels: labels,
        },
    ];
    let answers = [
        format!(r#""q1":{{"type":"choice","probabilities":{{"{EVIDENCE}":0.75,"other":0.25}}}}"#),
        r#""q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}"#.to_owned(),
        r#""q1":{"type":"score","probabilities":{"0":0.75,"1":0.25}}"#.to_owned(),
    ];
    let (listener, backend, engine) = loopback_engine(&answers);
    let evidence = crate::core::Evidence::new(EVIDENCE).expect("evidence");
    let plan = crate::core::Plan::authored(
        evidence.clone(),
        backend.model().clone(),
        questions.to_vec(),
    )
    .expect("a plan");
    let mut shown = format!("{plan:?} {plan:#?}");
    let mut judged = Vec::new();
    for question in questions {
        let judgment = engine
            .judge(
                &question,
                None,
                evidence.clone(),
                &crate::engine::Cancel::default(),
            )
            .expect("a judgment");
        let reply = &judgment.answered.reply;
        shown.push_str(&format!("{reply:?} {reply:#?}"));
        judged.push(format!("{:?} {:?}", judgment.answer, judgment.value));
    }
    assert_eq!(listener.requests().len(), 3);
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert_eq!(
        judged,
        [
            "Answer(Choice { pick: <22 bytes withheld>, probabilities: \
             Distribution { labels: <27 bytes withheld>, probabilities: [0.75, 0.25] }, \
             confidence: None }) Choice(Some(<22 bytes withheld>))",
            "Answer(Tag { probabilities: TagProbabilities { labels: <27 bytes withheld>, \
             probabilities: [0.9, 0.1] } }) Tag([<22 bytes withheld>])",
            "Answer(Score { level: <22 bytes withheld>, probabilities: \
             Distribution { labels: <27 bytes withheld>, probabilities: [0.75, 0.25] }, \
             confidence: None }) Score(0.25)",
        ]
    );
}

/// An engine over a loopback listener that answers each request with the next `answers`.
fn loopback_engine(
    answers: &[String],
) -> (
    conformance_backend::Listener,
    crate::core::Backend,
    crate::engine::facade::Engine,
) {
    let listener = conformance_backend::Listener::serving(
        answers
            .iter()
            .map(|answers| {
                conformance_backend::Canned::ok(&format!(
                    r#"{{"model":"jev-latest","answers":{{{answers}}}}}"#
                ))
            })
            .collect(),
    )
    .expect("a loopback listener");
    let backend =
        crate::core::Backend::resolve(Some(listener.base()), None, "jev-latest").expect("backend");
    let engine = crate::engine::facade::Engine::new(crate::engine::facade::Settings {
        backend: backend.clone(),
        profile: None,
        timeout: Duration::from_secs(30),
        max_retries: 0,
        retry_wait: Duration::from_millis(10),
        width: None,
        per_minute: None,
        storage: crate::engine::facade::Storage::default(),
        key: std::sync::Arc::new(|| Ok(crate::engine::facade::Key::new(KEY.to_owned()))),
        usage: std::sync::Arc::default(),
    })
    .expect("an engine");
    (listener, backend, engine)
}

/// `recognize` reads its evidence as names and edge labels, in plain and pretty `Debug`.
#[test]
fn no_recognize_debug_line_shows_the_evidence() {
    let name = crate::core::RecognizedName {
        text: EVIDENCE.to_owned(),
        start: 0,
        end: 1,
        length: 1,
        kind: "person".to_owned(),
        strength: 0.9,
    };
    let edge = crate::core::RelationEdge {
        relation: "works_for".to_owned(),
        source: name.clone(),
        target: name.clone(),
        probability: 0.8,
        either: false,
    };
    let edges = crate::core::Odds(vec![(EVIDENCE.to_owned(), 0.9)]);
    let lines = crate::core::Reading::new(crate::core::Framing::Lines, Vec::new())
        .expect("a text reading")
        .record(format!("\u{e9}{EVIDENCE}").as_bytes())
        .expect("a text record");
    let object =
        crate::core::Record::string_fields(vec![(EVIDENCE.to_owned(), EVIDENCE.to_owned())]);
    let shown = format!(
        "{name:?} {name:#?} {edge:?} {edge:#?} {edges:?} {edges:#?} \
         {lines:?} {lines:#?} {object:?} {object:#?}"
    );
    assert!(!shown.contains(EVIDENCE), "{shown}");
    assert_eq!(shown.matches("withheld").count(), 12, "{shown}");
    assert_eq!(format!("{edges:?}"), "Odds([(<22 bytes withheld>, 0.9)])");
    assert_eq!(format!("{lines:?}"), "Record(text, <24 bytes withheld>)");
    assert_eq!(format!("{object:?}"), "Record(json, <51 bytes withheld>)");
    assert!(
        shown.contains(r#"start: 0, end: 1, length: 1, kind: "person", strength: 0.9"#),
        "{shown}"
    );
}

