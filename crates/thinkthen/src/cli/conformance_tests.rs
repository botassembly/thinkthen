//! The shared compatibility cases, checked through the production pure core.

#[path = "conformance_tests/support/conformance.rs"]
mod conformance_support;
#[path = "conformance_tests/outcomes/outcomes.rs"]
mod outcomes;
#[path = "conformance_tests/profile_cases.rs"]
mod profile_cases;

use crate::core::adapters::systemone;
use crate::core::recording::Exchange as Recorded;
use crate::core::{
    AnswerOutcome, Cutting, Evidence, Find, ModelName, Plan, QuestionFile, QuestionSet,
    QuestionText, Threshold, Typed, Url, Value, Verb, question_sha256, ranking, resolve,
};
use conformance_support::{Case, Document, Exchange, ExpectedAnswer, Success};
use serde::Deserialize;
use std::collections::BTreeSet;

#[path = "conformance_tests/runner.rs"]
mod runner;

const CASES: &str = include_str!("../../../../conformance/cases.json");
const CAPTURED_REFUND: &str = include_str!(
    "../../../../demos/01-refund-gate/recording/e8b7d68fe0567786d9905df174191873ff0876e0c56efc008ff7a07a4de45d3e.json"
);
const VERBS: [&str; 8] = [
    "annotate", "choose", "decide", "filter", "find", "rank", "score", "tag",
];
const ERRORS: [&str; 6] = [
    "backend",
    "cancelled",
    "deadline",
    "defect",
    "local",
    "usage",
];

type CheckedAnswers = (Vec<(Value, f64)>, String, usize);

struct Asked {
    plan: Plan,
    names: Vec<String>,
    thresholds: Vec<Option<Threshold>>,
    digests: Vec<String>,
}

fn word(word: &str) -> Result<Verb, String> {
    match word {
        "decide" | "filter" | "rank" => Ok(Verb::Decide),
        "choose" => Ok(Verb::Choose),
        "tag" => Ok(Verb::Tag),
        "score" => Ok(Verb::Score),
        _ => Err(format!("unknown verb `{word}`")),
    }
}

fn asked(case: &Case, place: usize, exchange: &Exchange) -> Result<Asked, String> {
    if case.verb == "annotate" {
        return annotate(case, place, exchange);
    }
    let raw = case
        .question
        .as_ref()
        .ok_or_else(|| format!("{} has no question", case.id))?;
    let file = QuestionFile::parse(raw.get()).map_err(|error| error.to_string())?;
    let cutting = match case.verb.as_str() {
        "filter" => Cutting::OneCut,
        "rank" => Cutting::NoRule,
        _ => Cutting::AsTheVerbAllows,
    };
    let resolved = resolve(
        word(&case.verb)?,
        None,
        Some(&file),
        &Typed {
            cutting,
            ..Typed::default()
        },
    )
    .map_err(|error| error.to_string())?;
    let question = resolved
        .question()
        .cloned()
        .ok_or_else(|| format!("{} resolved no question", case.id))?;
    let threshold = resolved.threshold();
    let digest = question_sha256(&question, threshold).map_err(|error| error.to_string())?;
    let plan = Plan::new(
        Evidence::new(&exchange.evidence).map_err(|error| error.to_string())?,
        resolved.model().clone(),
        vec![question],
    )
    .map_err(|error| error.to_string())?;
    Ok(Asked {
        plan,
        names: vec!["q1".to_owned()],
        thresholds: vec![threshold],
        digests: vec![digest],
    })
}

fn annotate(case: &Case, place: usize, exchange: &Exchange) -> Result<Asked, String> {
    let raw = case
        .question_set
        .as_ref()
        .ok_or_else(|| format!("{} has no question set", case.id))?;
    let set = QuestionSet::parse(raw.get()).map_err(|error| error.to_string())?;
    let mut questions = Vec::new();
    let mut names = Vec::new();
    let mut thresholds = Vec::new();
    let mut digests = Vec::new();
    let group = set
        .groups()
        .get(place)
        .cloned()
        .ok_or_else(|| format!("{} has no annotate group {place}", case.id))?;
    for question_place in group {
        let named = set
            .questions()
            .get(question_place)
            .ok_or_else(|| format!("{} group {place} points outside its set", case.id))?;
        questions.push(named.question().clone());
        names.push(named.name().to_owned());
        thresholds.push(named.threshold());
        digests.push(
            question_sha256(named.question(), named.threshold())
                .map_err(|error| error.to_string())?,
        );
    }
    let plan = Plan::new(
        Evidence::new(&exchange.evidence).map_err(|error| error.to_string())?,
        ModelName::new("jev-latest").map_err(|error| error.to_string())?,
        questions,
    )
    .map_err(|error| error.to_string())?;
    Ok(Asked {
        plan,
        names,
        thresholds,
        digests,
    })
}

#[derive(Deserialize)]
struct FindSpec {
    find: String,
    none: bool,
    units: Vec<String>,
}

fn finding(case: &Case) -> Result<Find, String> {
    let raw = case
        .question
        .as_ref()
        .ok_or_else(|| format!("{} has no find question", case.id))?;
    let spec: FindSpec = serde_json::from_str(raw.get()).map_err(|error| error.to_string())?;
    let units = spec
        .units
        .into_iter()
        .map(Evidence::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Find::new(
        QuestionText::new(spec.find).map_err(|error| error.to_string())?,
        &units,
        ModelName::new("jev-latest").map_err(|error| error.to_string())?,
        spec.none,
    )
    .map_err(|error| error.to_string())
}

fn find_asked(case: &Case) -> Result<Asked, String> {
    let find = finding(case)?;
    Ok(Asked {
        plan: find.plan().clone(),
        names: vec!["q1".to_owned()],
        thresholds: vec![None],
        digests: vec![find.question_sha256().map_err(|error| error.to_string())?],
    })
}

fn expected<'a>(
    success: &'a Success,
    exchange: usize,
    name: &str,
) -> Result<&'a ExpectedAnswer, String> {
    success
        .answers
        .iter()
        .find(|answer| answer.exchange == exchange && answer.name == name)
        .ok_or_else(|| format!("exchange {exchange} has no expected answer `{name}`"))
}

#[allow(
    clippy::disallowed_types,
    reason = "fixture-only structural comparisons must ignore JSON formatting"
)]
fn same_json(one: &str, other: &str) -> Result<bool, String> {
    let one: serde_json::Value = serde_json::from_str(one).map_err(|error| error.to_string())?;
    let other: serde_json::Value =
        serde_json::from_str(other).map_err(|error| error.to_string())?;
    Ok(one == other)
}

fn validate_exchange(
    case: &Case,
    success: &Success,
    place: usize,
    exchange: &Exchange,
    backend_url: &Url,
    requests: &[String],
) -> Result<CheckedAnswers, String> {
    exchange.provenance.validate(exchange)?;
    let asked = if case.verb == "find" {
        find_asked(case)?
    } else {
        asked(case, place, exchange)?
    };
    let request = systemone::encode(&asked.plan).map_err(|error| error.to_string())?;
    if request != exchange.request.as_bytes() {
        return Err(format!(
            "{} exchange {place} has wrong request bytes",
            case.id
        ));
    }
    let reply = systemone::decode(&asked.plan, exchange.response.get().as_bytes())
        .map_err(|error| error.to_string())?;
    if reply.outcomes().len() != asked.names.len() {
        return Err(format!(
            "{} exchange {place} has wrong answer count",
            case.id
        ));
    }
    let mut values = Vec::new();
    let mut failed_questions = 0;
    for (answer_place, outcome) in reply.outcomes().iter().enumerate() {
        let name = asked
            .names
            .get(answer_place)
            .ok_or_else(|| "answer name is absent".to_owned())?;
        let held = expected(success, place, name)?;
        let threshold = asked
            .thresholds
            .get(answer_place)
            .copied()
            .ok_or_else(|| "answer threshold is absent".to_owned())?;
        if let Some(value) = outcomes::check(&case.id, name, outcome, held, threshold)? {
            values.push(value);
        } else {
            failed_questions += 1;
        }
        let digest = asked
            .digests
            .get(answer_place)
            .ok_or_else(|| "answer digest is absent".to_owned())?;
        if digest != &held.details.question_sha256 || reply.model().as_str() != held.details.model {
            return Err(format!("{} answer `{name}` has wrong metadata", case.id));
        }
        let exchange_request = Recorded::new(backend_url, exchange.request.as_bytes())
            .digest()
            .as_str()
            .to_owned();
        let expected_requests = if case.verb == "annotate" {
            requests
        } else {
            std::slice::from_ref(&exchange_request)
        };
        if held.details.requests != expected_requests {
            return Err(format!("{} answer `{name}` has wrong requests", case.id));
        }
    }
    Ok((values, reply.model().as_str().to_owned(), failed_questions))
}

fn validate_operation(
    case: &Case,
    success: &Success,
    odds: &[f64],
    values: &[Value],
) -> Result<(), String> {
    let Some(raw) = &success.operation else {
        return Ok(());
    };
    match case.verb.as_str() {
        "filter" => conformance_support::filter(raw, values),
        "rank" => conformance_support::rank(raw, odds, &ranking(odds, None)),
        "find" => {
            let exchange = case
                .exchanges
                .first()
                .ok_or_else(|| "find has no exchange".to_owned())?;
            let asked = find_asked(case)?;
            let reply = systemone::decode(&asked.plan, exchange.response.get().as_bytes())
                .map_err(|error| error.to_string())?;
            let [AnswerOutcome::Answered(answer)] = reply.outcomes() else {
                return Err("find has no answer".to_owned());
            };
            conformance_support::find(
                raw,
                &finding(case)?
                    .select(answer)
                    .map_err(|error| error.to_string())?,
                answer,
            )
        }
        _ => Err(format!("{} carries an unexpected operation", case.id)),
    }
}

fn validate_fault(case: &Case, kind: &str) -> Result<(), String> {
    let raw = case
        .question
        .as_ref()
        .ok_or_else(|| format!("{} has no question", case.id))?;
    let file = QuestionFile::parse(raw.get()).map_err(|error| error.to_string())?;
    resolve(word(&case.verb)?, None, Some(&file), &Typed::default())
        .map_err(|error| error.to_string())?;
    let injection = &case
        .operation
        .as_ref()
        .ok_or_else(|| format!("{} has no injection", case.id))?
        .injection;
    let accepted = match kind {
        "usage" => "invalid_arguments",
        "backend" => "response_refusal",
        "local" => "recording_read_failure",
        "cancelled" => "cancel_token",
        "deadline" => "expired_deadline",
        "defect" => "internal_invariant_failure",
        _ => return Err(format!("unknown error kind `{kind}`")),
    };
    (injection == accepted)
        .then_some(())
        .ok_or_else(|| format!("{kind} has the wrong injection"))
}

fn validate(text: &str) -> Result<(), String> {
    conformance_support::privacy(text)?;
    let document: Document = serde_json::from_str(text).map_err(|error| error.to_string())?;
    document.validate_header(&VERBS, &ERRORS)?;
    let backend_url = Url::new(&document.backend_url).map_err(|error| error.to_string())?;
    let mut ids = BTreeSet::new();
    let mut verbs = BTreeSet::new();
    let mut errors = BTreeSet::new();
    for case in &document.cases {
        if !ids.insert(case.id.as_str()) {
            return Err(format!("duplicate case id `{}`", case.id));
        }
        if !VERBS.contains(&case.verb.as_str()) {
            return Err(format!("unknown verb `{}`", case.verb));
        }
        verbs.insert(case.verb.as_str());
        case.validate_shape()?;
        if let Some(error) = &case.expect.error {
            validate_fault(case, &error.kind)?;
            errors.insert(error.kind.as_str());
            continue;
        }
        let success = case
            .expect
            .success
            .as_ref()
            .ok_or_else(|| format!("{} has no outcome", case.id))?;
        let mut odds = Vec::new();
        let mut values = Vec::new();
        let mut model: Option<String> = None;
        let mut failed_questions = 0;
        let requests = case
            .exchanges
            .iter()
            .map(|exchange| {
                Recorded::new(&backend_url, exchange.request.as_bytes())
                    .digest()
                    .as_str()
                    .to_owned()
            })
            .collect::<Vec<_>>();
        for (place, exchange) in case.exchanges.iter().enumerate() {
            let (answers, reported, failed) =
                validate_exchange(case, success, place, exchange, &backend_url, &requests)?;
            if model.as_ref().is_some_and(|held| held != &reported) {
                return Err(format!("{} has replies from different models", case.id));
            }
            model = Some(reported);
            failed_questions += failed;
            for (value, probability) in answers {
                values.push(value);
                odds.push(probability);
            }
        }
        if success.answers.len() != values.len() + success.failed_questions {
            return Err(format!("{} has an unmatched expected answer", case.id));
        }
        if failed_questions != success.failed_questions {
            return Err(format!("{} has the wrong failed question count", case.id));
        }
        validate_operation(case, success, &odds, &values)?;
    }
    if verbs != BTreeSet::from(VERBS) || errors != BTreeSet::from(ERRORS) {
        return Err("verb or error-kind coverage is incomplete".to_owned());
    }
    Ok(())
}

#[test]
fn shared_cases_match_the_production_core() {
    validate(CASES).expect("the shared cases match the core");
}

#[test]
fn focused_mutations_are_refused() {
    let duplicate_backend_fault = CASES
        .replacen(
            "\"injection\": \"recording_read_failure\"",
            "\"injection\": \"response_refusal\"",
            1,
        )
        .replacen("\"kind\": \"local\"", "\"kind\": \"backend\"", 1);
    let mutations = [
        CASES.replacen("\"bare\": true", "\"bare\": false", 1),
        CASES.replacen(
            "https://api.typesafe.ai/v1/systemone",
            "https://wrong.example/v1/systemone",
            1,
        ),
        CASES.replacen(
            "e8b7d68fe0567786d9905df174191873ff0876e0c56efc008ff7a07a4de45d3e",
            "08b7d68fe0567786d9905df174191873ff0876e0c56efc008ff7a07a4de45d3e",
            1,
        ),
        CASES.replacen("{\\\"state\\\":\\\"From:", "{\\\"state\\\":\\\"XFrom:", 1),
        CASES.replacen(
            "\"id\": \"02-decide-no\"",
            "\"id\": \"01-decide-yes-captured\"",
            1,
        ),
        CASES.replacen("\"verb\": \"decide\"", "\"verb\": \"guess\"", 1),
        duplicate_backend_fault,
        CASES.replacen('{', "{\"authorization\":\"secret\",", 1),
        CASES.replacen('{', "{\"api_key\":\"secret\",", 1),
        CASES.replacen('{', "{\"x-api-key\":\"secret\",", 1),
        CASES.replacen("\"kind\": \"filter\"", "\"kind\": \"single\"", 1),
        CASES.replacen(
            "\"operation\": {\n            \"indexes\"",
            "\"ignored\": {\n            \"indexes\"",
            1,
        ),
        CASES.replacen(
            "\"kind\": \"single\",\n          \"answers\"",
            "\"kind\": \"single\",\n          \"operation\": {},\n          \"answers\"",
            1,
        ),
        CASES.replacen(
            "      \"exchanges\": [",
            "      \"operation\": {\"injection\":\"cancel_token\"},\n      \"exchanges\": [",
            1,
        ),
        CASES.replacen(
            "\"refund\": {\n            \"decide\":",
            "\"refund\": {\n            \"choose\":",
            1,
        ),
    ];
    for (place, mutation) in mutations.into_iter().enumerate() {
        assert!(validate(&mutation).is_err(), "mutation {place} passed");
    }
}

impl conformance_support::Provenance {
    fn validate(&self, exchange: &Exchange) -> Result<(), String> {
        match self {
            Self::SyntheticContract => Ok(()),
            Self::Captured { path } => {
                if path
                    != "demos/01-refund-gate/recording/e8b7d68fe0567786d9905df174191873ff0876e0c56efc008ff7a07a4de45d3e.json"
                {
                    return Err(format!("unknown captured recording `{path}`"));
                }
                let recorded: conformance_support::Recording =
                    serde_json::from_str(CAPTURED_REFUND).map_err(|error| error.to_string())?;
                if !same_json(recorded.request.get(), &exchange.request)?
                    || !same_json(recorded.response.get(), exchange.response.get())?
                {
                    return Err("captured exchange differs from its recording".to_owned());
                }
                Ok(())
            }
        }
    }
}
