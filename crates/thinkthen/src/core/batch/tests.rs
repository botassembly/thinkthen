//! Design tests 1, 2 and 3 of the batching design, over fixture files and the
//! batches `specification/fixtures/batching/README.md` works out by hand.

use std::num::NonZeroUsize;

use super::{Batch, BatchError, BatchRecord, Batcher, Closed, Setting};
use crate::core::Pointer;
use crate::core::adapters::systemone::tests::{TEAMS, disruption_plan, team_plan, urgency_plan};
use crate::core::backend::Backend;
use crate::core::backend_profile::{BackendProfile, LimitKind};
use crate::core::find::Find;
use crate::core::json::Json;
use crate::core::plan::Plan;
use crate::core::question::{Labels, Question};
use crate::core::recording::Exchange;
use crate::core::records::{Framing, Reading, Record};
use crate::core::render::json_line;
use crate::core::text::{Evidence, Meaning, ModelName, QuestionText};

const LOOPBACK: &str = "http://127.0.0.1:9";
const BUILT_IN: &str = "https://api.typesafe.ai/v1";
const SONG: &str = "The text is the title of a song by the Beatles.";
const GROUPING: &str = include_str!("../../../../../specification/fixtures/batching/grouping.txt");

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            "../../../../../specification/fixtures/systemone/",
            $name,
            ".request.json"
        ))
    };
}

fn backend(url: &str, model: &str) -> Backend {
    Backend::resolve(Some(url), None, model).expect("backend")
}

fn profile(limits: &str) -> Option<BackendProfile> {
    let text = format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"test",{limits}}}"#);
    Some(BackendProfile::parse(&text).expect("profile"))
}

fn decide(text: &str) -> Question {
    Question::Decide {
        text: QuestionText::new(text).expect("text"),
        yes: None,
        no: None,
    }
}

fn text(record: &str) -> BatchRecord {
    let evidence = Evidence::new(record).expect("evidence");
    BatchRecord {
        value: evidence.as_json(),
        evidence,
    }
}

fn loopback() -> Backend {
    backend(LOOPBACK, "jev-latest")
}

/// A question written as JSON, which cannot take the quote prefix.
fn structured() -> Question {
    let text = Json::parse(r#"{"ask":"urgent?"}"#).expect("json");
    Question::Decide {
        text: QuestionText::structured(&text).expect("text"),
        yes: None,
        no: None,
    }
}

fn records(count: usize) -> Setting {
    Setting::Records(NonZeroUsize::new(count).expect("positive"))
}

/// The fixture's bytes written compact, keys in their order, without a newline.
fn compact(fixture: &str) -> Vec<u8> {
    json_line(&Json::parse(fixture).expect("fixture JSON"))
        .expect("compact")
        .into_bytes()
}

/// Every batch a batcher closes over these records, or the first refusal.
fn run(mut batcher: Batcher, records: Vec<BatchRecord>) -> Result<Vec<Batch>, BatchError> {
    let mut batches = Vec::new();
    for record in records {
        batches.extend(batcher.push(record)?);
    }
    batches.extend(batcher.finish()?);
    Ok(batches)
}

fn batches(
    backend: Backend,
    profile: Option<BackendProfile>,
    question: Question,
    setting: Setting,
    records: Vec<BatchRecord>,
) -> Vec<Batch> {
    let batcher = Batcher::new(backend, profile, question, setting, None).expect("batcher");
    run(batcher, records).expect("batches")
}

/// Each batch as its record count and why it closed.
fn shape(batches: &[Batch]) -> Vec<(usize, Closed)> {
    batches
        .iter()
        .map(|batch| (batch.questions.len(), batch.closed))
        .collect()
}

#[test]
fn a_batch_of_one_is_todays_request() {
    let find_evidence = [
        Evidence::new("alpha \"quote\"\\path").expect("e"),
        Evidence::new("beta").expect("e"),
    ];
    let find = Find::new(
        QuestionText::new("Which unit answers?").expect("question"),
        &find_evidence,
        ModelName::new("local-1").expect("model"),
        false,
    )
    .expect("find");
    let cases: [(Plan, &str); 4] = [
        (urgency_plan(), fixture!("decide-urgent")),
        (team_plan(), fixture!("choose-team")),
        (disruption_plan(), fixture!("score-disruption")),
        (find.plan().clone(), fixture!("find-two")),
    ];
    for (plan, fixture) in cases {
        let expected = compact(fixture);
        let backend = backend(LOOPBACK, plan.model().as_str());
        let digest = Exchange::new(backend.url(), &expected).digest();
        for setting in [records(1), Setting::Max] {
            let record = BatchRecord {
                evidence: plan.evidence().clone(),
                value: plan.evidence().as_json(),
            };
            let question = plan.questions().first().expect("question").clone();
            let found = batches(backend.clone(), None, question, setting, vec![record]);
            let [batch] = found.as_slice() else {
                panic!("one batch, not {}", found.len())
            };
            assert_eq!(
                String::from_utf8_lossy(&batch.body),
                String::from_utf8_lossy(&expected)
            );
            assert_eq!(batch.digest, digest);
            assert_eq!(batch.questions, [0]);
        }
    }
}

#[test]
fn each_batch_body_matches_its_fixture() {
    let three = ["Come Together", "Say \"hello\"\nthen leave", "Because"];
    let mails = ["My card was charged twice.", "The parcel never arrived."];
    let choose = Question::Choose {
        text: QuestionText::new("Which team owns this request?").expect("text"),
        options: Labels::options(TEAMS.iter().map(|team| (*team).to_owned()).collect())
            .expect("options"),
    };
    let meant = Question::Decide {
        text: QuestionText::new("The record names a song on Abbey Road.").expect("text"),
        yes: Some(Meaning::new("The album is Abbey Road.").expect("yes")),
        no: Some(Meaning::new("Another album.").expect("no")),
    };
    let csv = Reading::new(Framing::Csv, Vec::new()).expect("reading");
    let rows = [("Come Together", "Abbey Road"), ("Help!", "Help!")].map(|(title, album)| {
        let row = [("title", title), ("album", album)]
            .map(|(key, cell)| (key.to_owned(), cell.to_owned()));
        csv.batch_record(&Record::string_fields(row.to_vec()))
            .expect("row")
    });
    let duplicate = ["Come Together", "Because", "Come Together"];
    let context = "Come Together appears on Abbey Road.\nBecause appears on Abbey Road.\n";
    let cases: [BodyCase<'_>; 5] = [
        (
            decide(SONG),
            three.map(text).into(),
            None,
            fixture!("batch-three"),
            vec![0, 1, 2],
        ),
        (
            decide("It appears on the album Abbey Road."),
            ["Come Together", "Help!"].map(text).into(),
            Some(context),
            fixture!("batch-context"),
            vec![0, 1],
        ),
        (
            choose,
            mails.map(text).into(),
            None,
            fixture!("batch-choose"),
            vec![0, 1],
        ),
        (meant, rows.into(), None, fixture!("batch-csv"), vec![0, 1]),
        (
            decide(SONG),
            duplicate.map(text).into(),
            None,
            fixture!("batch-duplicate"),
            vec![0, 1, 0],
        ),
    ];
    for (question, records, context, fixture, questions) in cases {
        let context = context.map(|held| Evidence::new(held).expect("context"));
        let batcher =
            Batcher::new(loopback(), None, question, Setting::Max, context).expect("batcher");
        let found = run(batcher, records).expect("batches");
        let [batch] = found.as_slice() else {
            panic!("one batch, not {}", found.len())
        };
        assert_eq!(String::from_utf8_lossy(&batch.body), fixture.trim_end());
        assert_eq!(
            (batch.questions.clone(), batch.closed),
            (questions, Closed::End)
        );
        let debug = format!("{batch:?}");
        assert!(
            !debug.contains("Come") && !debug.contains("card"),
            "{debug}"
        );
    }
}

/// The grouping fixture's lines, with a line inserted after a position if given.
fn grouping(insert: Option<(usize, &str)>) -> Vec<BatchRecord> {
    let mut lines: Vec<&str> = GROUPING.lines().collect();
    if let Some((after, line)) = insert {
        lines.insert(after, line);
    }
    lines.into_iter().map(text).collect()
}

#[test]
fn batches_close_where_the_readme_says() {
    use Closed::{Content, End, Limit, Size};
    let eight = || profile(r#""max_questions":8"#);
    let plan =
        |setting, insert| batches(loopback(), eight(), decide(SONG), setting, grouping(insert));
    let original = plan(Setting::Max, None);
    assert_eq!(
        shape(&original),
        [(8, Content), (8, Limit), (1, Content), (8, End)]
    );
    let fives = plan(records(5), None);
    assert_eq!(
        shape(&fives),
        [
            (5, Size),
            (3, Content),
            (5, Size),
            (4, Content),
            (5, Size),
            (3, End)
        ]
    );
    let digests = |found: &[Batch]| {
        found
            .iter()
            .map(|batch| batch.digest.clone())
            .collect::<Vec<_>>()
    };
    let before = digests(&original);
    let plain = plan(Setting::Max, Some((3, "line 24")));
    assert_eq!(
        shape(&plain),
        [(8, Limit), (1, Content), (8, Limit), (1, Content), (8, End)]
    );
    let kept = digests(&plain);
    assert_ne!(kept.first(), before.first());
    assert_eq!(kept.get(2..), before.get(1..));
    let cut = plan(Setting::Max, Some((12, "line 10633")));
    assert_eq!(
        shape(&cut),
        [(8, Content), (5, Content), (5, Content), (8, End)]
    );
    let kept = digests(&cut);
    assert_eq!((kept.first(), kept.get(3)), (before.first(), before.get(3)));
    assert!(!kept.contains(&before[1]) && !kept.contains(&before[2]));
}

/// A body fixture's question, records, context, fixture, and each record's first question.
type BodyCase<'a> = (
    Question,
    Vec<BatchRecord>,
    Option<&'a str>,
    &'a str,
    Vec<usize>,
);

/// A limit case's backend, profile, question, records, and expected batches.
type LimitCase = (
    Backend,
    Option<BackendProfile>,
    Question,
    Vec<BatchRecord>,
    Vec<(usize, Closed)>,
);

/// Three records of `bytes` bytes each, from distinct letters.
fn sized(bytes: usize) -> Vec<BatchRecord> {
    ["a", "b", "c"]
        .map(|letter| text(&letter.repeat(bytes)))
        .into()
}

#[test]
fn limits_close_batches_by_exact_bytes_and_the_ceiling() {
    use Closed::{End, Limit};
    let three = || {
        ["Come Together", "Say \"hello\"\nthen leave", "Because"]
            .map(text)
            .into()
    };
    let body = compact(fixture!("batch-three")).len();
    let state = r#"{"records":["Come Together","Say \"hello\"\nthen leave","Because"]}"#.len();
    let urgent = || text("Help! My payouts have been failing for 3 days.");
    let today = compact(fixture!("decide-urgent")).len();
    let cases: [LimitCase; 5] = [
        (
            loopback(),
            profile(&format!(r#""max_request_bytes":{body}"#)),
            decide(SONG),
            three(),
            vec![(3, End)],
        ),
        (
            loopback(),
            profile(&format!(r#""max_request_bytes":{}"#, body - 1)),
            decide(SONG),
            three(),
            vec![(2, Limit), (1, End)],
        ),
        (
            loopback(),
            profile(&format!(r#""max_evidence_bytes":{state}"#)),
            decide(SONG),
            three(),
            vec![(3, End)],
        ),
        (
            loopback(),
            profile(&format!(r#""max_evidence_bytes":{}"#, state - 1)),
            decide(SONG),
            three(),
            vec![(2, Limit), (1, End)],
        ),
        (
            loopback(),
            profile(&format!(r#""max_request_bytes":{today}"#)),
            decide("Does this convey urgency?"),
            vec![urgent(), urgent(), urgent()],
            vec![(3, End)],
        ),
    ];
    check(cases);
}

#[test]
fn the_ceiling_closes_batches_at_the_built_in_address_only() {
    use Closed::{End, Limit};
    let built_in = || backend(BUILT_IN, "jev-latest");
    let cases: [LimitCase; 5] = [
        (
            built_in(),
            None,
            decide(SONG),
            sized(20_000),
            vec![(2, Limit), (1, End)],
        ),
        (
            built_in(),
            None,
            decide(SONG),
            sized(40_000),
            vec![(1, Limit), (1, Limit), (1, End)],
        ),
        (
            built_in(),
            None,
            decide(SONG),
            vec![text(&"a".repeat(100_000))],
            vec![(1, End)],
        ),
        (
            built_in(),
            profile(r#""max_request_bytes":200000"#),
            decide(SONG),
            sized(40_000),
            vec![(2, Limit), (1, End)],
        ),
        (
            loopback(),
            None,
            decide(SONG),
            sized(40_000),
            vec![(3, End)],
        ),
    ];
    check(cases);
}

/// Plan each case at `Max` and compare its batches' sizes and reasons.
fn check(cases: [LimitCase; 5]) {
    for (backend, profile, question, records, expected) in cases {
        assert_eq!(
            shape(&batches(backend, profile, question, Setting::Max, records)),
            expected
        );
    }
}

#[test]
fn questions_copies_and_refusals_follow_the_batch_rules() {
    use Closed::{End, Limit, Size};
    let tag = Question::Tag {
        text: QuestionText::new("Which topics?").expect("text"),
        labels: Labels::tags(["a", "b", "c"].map(|label| (label.to_owned(), None)).into())
            .expect("labels"),
    };
    let tags = batches(
        loopback(),
        profile(r#""max_questions":8"#),
        tag,
        Setting::Max,
        ["x", "y", "z"].map(text).into(),
    );
    assert_eq!(shape(&tags), [(2, Limit), (1, End)]);
    assert_eq!(
        tags.first().map(|batch| batch.questions.clone()),
        Some(vec![0, 3])
    );
    let structured = structured();
    let alone = batches(
        loopback(),
        None,
        structured,
        Setting::Max,
        ["x", "x", "y"].map(text).into(),
    );
    assert_eq!(shape(&alone), [(1, Size), (1, Size), (1, Size)]);
    let reading =
        Reading::new(Framing::Jsonl, vec![Pointer::new("/n").expect("pointer")]).expect("reading");
    let objects = [
        r#"{"n":7}"#,
        r#"{"n":{"a":1,"b":2}}"#,
        r#"{"n":{"b":2,"a":1}}"#,
    ]
    .map(|line| {
        reading
            .batch_record(&reading.record(line.as_bytes()).expect("record"))
            .expect("selected")
    });
    let found = batches(loopback(), None, decide("Q"), Setting::Max, objects.into());
    let [batch] = found.as_slice() else {
        panic!("one batch")
    };
    let questions: Vec<String> = batch
        .plan
        .questions()
        .iter()
        .map(|question| match question {
            Question::Decide { text, .. } => text.as_json().as_str().expect("text").to_owned(),
            _ => panic!("a decide question"),
        })
        .collect();
    assert_eq!(
        questions,
        [
            r#"The text is 7. Q"#,
            r#"The text is {"a":1,"b":2}. Q"#,
            r#"The text is {"b":2,"a":1}. Q"#
        ]
    );
}

#[test]
fn a_context_is_refused_without_echoing_it() {
    let beside = |profile, question| {
        let context = Evidence::new("secret context").expect("context");
        Batcher::new(loopback(), profile, question, Setting::Max, Some(context))
    };
    let refused = beside(None, structured()).err();
    assert_eq!(refused, Some(BatchError::StructuredQuestionWithContext));
    let refused = beside(profile(r#""max_evidence_bytes":5"#), decide("Q")).err();
    let over = BatchError::ContextOverLimit {
        kind: LimitKind::EvidenceBytes,
        limit: 5,
        actual: 14,
    };
    assert_eq!(refused, Some(over));
    let batcher = beside(profile(r#""max_request_bytes":200"#), decide("Q")).expect("batcher");
    let error =
        run(batcher, vec![text("short"), text(&"secret ".repeat(40))]).expect_err("late overflow");
    let limit = (LimitKind::RequestBytes, 200);
    assert!(
        matches!(error, BatchError::ContextOverLimit { kind, limit: most, .. } if (kind, most) == limit)
    );
    assert!(!format!("{error:?} {error}").contains("secret"));
}
