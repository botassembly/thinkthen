//! The convenience path against the explicit engine, each failure cause, and the deadline.
//!
//! The convenience functions build their engine from the environment, so that
//! half runs in a child process with a cleared environment, a loopback
//! address, and a fake key. No test changes its own environment.

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use conformance_backend::Backend;
use serde_json::{Value, json};

use crate::cases::{Checked, Verbatim, engine, said, same};
use thinkthen::{
    Annotated, CallOptions, CancelToken, ChooseQuestion, Engine, Error, ErrorKind, FailureCause,
    Question, QuestionSet,
};

const CHILD: &str = "CONSUMER_CONVENIENCE_BASE";
const KEY: &str = "sk-consumer-loopback";

thinkthen::choices! { enum Team { Billing => "billing", Other => "other" } }

fn folder(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("consumer-{name}-{}", std::process::id()));
    let _absent = std::fs::remove_dir_all(&folder);
    folder
}

#[test]
fn the_convenience_path_equals_the_explicit_engine() {
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let output = crate::run::output(
        Command::new(std::env::current_exe().expect("this test binary"))
            .args(["--exact", "--ignored", "paths::convenience_child"])
            .args(["--test-threads", "1"])
            .env_clear()
            .env(CHILD, &base)
            .env("THINKTHEN_BASE_URL", &base)
            .env("THINKTHEN_API_KEY", KEY)
            .env("THINKTHEN_CACHE", folder("convenience-cache")),
    )
    .expect("the child ran");
    let said = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{said}");
    assert!(said.contains("1 passed"), "the child ran its test: {said}");
    assert_eq!(
        backend.count(),
        12,
        "both paths' six distinct first-round requests sent"
    );
}

/// The child half. It does nothing unless the parent named a base.
#[test]
#[ignore = "the child half; its parent runs it with --ignored"]
#[expect(
    clippy::too_many_lines,
    reason = "one child compares both engine paths and their cached second round"
)]
fn convenience_child() {
    let Ok(base) = std::env::var(CHILD) else {
        return;
    };
    let explicit = Engine::builder()
        .base_url(&base)
        .and_then(|builder| builder.api_key(KEY))
        .and_then(|builder| builder.cache_at(folder("explicit-cache")))
        .and_then(thinkthen::EngineBuilder::build)
        .expect("the explicit engine");
    let (decide, team, set) = questions();
    let score = Question::score("How severe is this?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("a score question");
    let rank = Question::rank("Which is most urgent?").expect("text");
    let records = ["one note", "two notes", "three notes"];
    let token = CancelToken::new();
    token.cancel();
    // Twice, so the second round comes from each path's own cache.
    let mut previous_observations = None;
    for _ in 0..2 {
        assert_eq!(
            explicit
                .decide(&decide, "a note")
                .ok()
                .map(thinkthen::Call::into_value),
            Some(thinkthen::Answer::Yes)
        );
        assert_eq!(
            thinkthen::decide(&decide, "a note")
                .ok()
                .map(thinkthen::Call::into_value),
            Some(thinkthen::Answer::Yes)
        );
        let one = explicit
            .details(&score, "a note")
            .expect("the engine details")
            .into_value();
        let other = thinkthen::details(&score, "a note")
            .expect("the convenience details")
            .into_value();
        assert_eq!(one.to_json(), other.to_json());
        assert_eq!(one.to_scalar_json(), other.to_scalar_json());
        assert_eq!(one.value(), other.value());
        assert_eq!(one.probabilities(), other.probabilities());
        assert_eq!(one.reported_usage(), other.reported_usage());
        let observations = (one.observations().to_vec(), other.observations().to_vec());
        if let Some(previous) = previous_observations {
            assert_eq!(
                observations, previous,
                "each cache retains its own observation"
            );
        } else {
            assert_ne!(
                observations.0, observations.1,
                "separate live answers have separate observations"
            );
        }
        previous_observations = Some(observations);
        assert_eq!(
            explicit
                .choose(&team, "a note")
                .expect("the engine chooses")
                .into_value(),
            thinkthen::choose(&team, "a note")
                .expect("the convenience chooses")
                .into_value()
        );
        assert_eq!(
            rows(explicit.filter(&decide, records), |kept| kept),
            rows(thinkthen::filter(&decide, records), |kept| kept)
        );
        let parts = thinkthen::Row::into_parts;
        assert_eq!(
            rows(explicit.decide_many(&decide, records), parts),
            rows(thinkthen::decide_many(&decide, records), parts)
        );
        assert_eq!(
            explicit
                .rank(&rank, records)
                .expect("the engine ranks")
                .into_value(),
            thinkthen::rank(&rank, records)
                .expect("the convenience ranks")
                .into_value()
        );
        let values = |record: thinkthen::AnnotatedRecord<&str>| record.values().to_vec();
        assert_eq!(
            rows(explicit.annotate(&set, records), values),
            rows(thinkthen::annotate(&set, records), values)
        );
        for (one, other) in [
            (
                explicit.decide(&decide, "  "),
                thinkthen::decide(&decide, "  "),
            ),
            (
                explicit.decide_with(&decide, "a note", CallOptions::new().cancel(&token)),
                thinkthen::decide_with(&decide, "a note", CallOptions::new().cancel(&token)),
            ),
        ] {
            let (one, other) = (one.expect_err("refused"), other.expect_err("refused"));
            assert_eq!(
                (one.kind(), one.retryable(), one.to_string()),
                (other.kind(), other.retryable(), other.to_string())
            );
        }
    }
    assert_eq!(
        explicit.usage(),
        thinkthen::usage().expect("the process engine")
    );
    assert_eq!(
        explicit.usage().requests_sent(),
        6,
        "six distinct requests per path"
    );
    assert!(
        explicit.usage().cache_answers() > 0,
        "the second round came from the cache"
    );
    let (first, again) = (thinkthen::default_engine(), thinkthen::default_engine());
    assert!(
        std::ptr::eq(first.expect("built"), again.expect("built")),
        "one lazy engine"
    );
}

/// A decide question, a choose question, and a set that asks both, the choose last.
fn questions() -> (Question, ChooseQuestion<Team>, QuestionSet) {
    let decide = Question::decide("Is this urgent?").expect("text").cut();
    let team = Question::choose::<Team>("Which team owns this?")
        .and_then(|builder| builder.option(Team::Billing, None))
        .and_then(|builder| builder.option(Team::Other, None))
        .and_then(thinkthen::ChooseBuilder::build)
        .expect("a choose question");
    let set = QuestionSet::builder()
        .question("urgent", decide.clone())
        .and_then(|builder| builder.choose("team", team.clone()))
        .and_then(thinkthen::QuestionSetBuilder::build)
        .expect("a question set");
    (decide, team, set)
}

/// Every row of a batch, each mapped or `None` for an error.
fn rows<T, U>(batch: thinkthen::Batch<'_, T>, keep: impl Fn(T) -> U) -> Vec<U> {
    batch
        .map(|row| keep(row.expect("each row answers")))
        .collect()
}

#[test]
fn each_malformed_reply_fails_the_last_question_with_its_cause() {
    let backend = Backend::start().expect("backend");
    let (_, _, set) = questions();
    for (arm, cause) in [
        ("missing_answer", FailureCause::MissingAnswer),
        ("wrong_kind", FailureCause::WrongKind),
        ("missing_probability", FailureCause::MissingProbability),
        ("invalid_probability", FailureCause::InvalidProbability),
        ("invalid_distribution", FailureCause::InvalidDistribution),
        (
            "unexpected_probability",
            FailureCause::UnexpectedProbability,
        ),
    ] {
        let engine = engine(&format!("{}/arm/malformed/{arm}/v1", backend.origin())).expect(arm);
        let record = engine
            .annotate(&set, ["a note"])
            .next()
            .expect("one record")
            .expect(arm);
        let values: Vec<_> = record
            .values()
            .iter()
            .map(thinkthen::NamedAnnotation::value)
            .collect();
        assert!(
            matches!(values[0], Annotated::Decision(thinkthen::Answer::Yes)),
            "{arm}: {values:?}"
        );
        let Annotated::Failed(failed) = values[1] else {
            panic!("{arm}: the last question did not fail: {values:?}");
        };
        assert_eq!(
            (failed.kind(), failed.cause()),
            (ErrorKind::Backend, cause),
            "{arm}"
        );
    }
}

#[test]
fn a_held_reply_ends_the_call_at_its_deadline() {
    let backend = Backend::start().expect("backend");
    let engine = engine(&format!("{}/arm/held/v1", backend.origin())).expect("a held engine");
    let decide = Question::decide("Is this urgent?").expect("text").cut();
    let started = Instant::now();
    let options = CallOptions::new()
        .deadline_after(Duration::from_millis(300))
        .expect("a budget");
    let error = engine
        .decide_with(&decide, "a note", options)
        .expect_err("the reply is held");
    let waited = started.elapsed();
    backend.release();
    assert_eq!(
        (error.kind(), backend.count()),
        (ErrorKind::Deadline, 1),
        "{error}"
    );
    assert!(
        waited >= Duration::from_millis(300) && waited < Duration::from_secs(10),
        "{waited:?}"
    );
}

/// Each error case through its public boundary. Only the refusal arm may see a request.
pub(crate) fn refused(backend: &Backend, case: &Value, verbatim: &Verbatim, kind: &str) -> Checked {
    let id = case["id"].as_str().unwrap_or_default();
    let text = case["question"]["decide"].as_str().unwrap_or_default();
    let generic = format!("{}/generic/v1", backend.origin());
    let before = backend.count();
    let decide = |engine: Checked<Engine>,
                  options: CallOptions<'_>,
                  evidence: &str|
     -> Checked<Result<(), Error>> {
        let asked = Question::decide(text).map_err(said)?.cut();
        Ok(engine?.decide_with(&asked, evidence, options).map(drop))
    };
    let token = CancelToken::new();
    token.cancel();
    let file = std::env::temp_dir().join(format!("consumer-{id}-{}", std::process::id()));
    let result = match id {
        "20-usage-fault" => decide(engine(&generic), CallOptions::new(), "   ")?,
        "21-backend-fault" => decide(
            engine(&format!("{}/arm/refuse/v1", backend.origin())),
            CallOptions::new(),
            "Is this urgent?",
        )?,
        "22-local-fault" => {
            std::fs::write(&file, "not a folder").map_err(|error| error.to_string())?;
            let built = Engine::builder()
                .base_url(&generic)
                .and_then(|b| b.api_key("sk-consumer-loopback"))
                .and_then(|b| b.cache_at(&file))
                .and_then(thinkthen::EngineBuilder::build);
            match built {
                Ok(engine) => decide(Ok(engine), CallOptions::new(), "Is this urgent?")?,
                Err(error) => Err(error),
            }
        }
        "23-cancelled-fault" => decide(
            engine(&generic),
            CallOptions::new().cancel(&token),
            "Is this urgent?",
        )?,
        "24-deadline-fault" => {
            let past = Instant::now()
                .checked_sub(Duration::from_secs(1))
                .ok_or("no past instant")?;
            decide(
                engine(&generic),
                CallOptions::new().deadline_at(past),
                "Is this urgent?",
            )?
        }
        "30-local-question-file" => {
            std::fs::write(
                &file,
                verbatim.question.as_ref().map_or("", |raw| raw.get()),
            )
            .map_err(|error| error.to_string())?;
            Question::load(&file).map(drop)
        }
        "29-usage-json-text" => Question::decide(text)
            .and_then(|builder| {
                builder.cut_at(case["question"]["threshold"].as_f64().unwrap_or_default())
            })
            .map(drop),
        "31-usage-rank-blank-question" => Question::rank(text).map(drop),
        other => return Err(format!("no public boundary is written for {other}")),
    };
    let _removed = std::fs::remove_file(&file);
    let Err(error) = result else {
        return Err("the case succeeded".to_owned());
    };
    let sent = backend.count() - before;
    same(
        "sent",
        &json!(sent),
        &json!(usize::from(id == "21-backend-fault")),
    )?;
    same(
        "kind",
        &json!(format!("{:?}", error.kind()).to_lowercase()),
        &json!(kind),
    )?;
    same("retryable", &json!(error.retryable()), &json!(false))?;
    if error.kind() == ErrorKind::Usage && error.to_string().trim().is_empty() {
        return Err("a usage error names nothing".to_owned());
    }
    Ok(())
}
