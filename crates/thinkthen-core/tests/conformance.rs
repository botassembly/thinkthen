//! The shared compatibility cases, checked through the production pure core.

#![allow(
    clippy::disallowed_types,
    reason = "fixture-only structural comparisons must ignore JSON formatting"
)]

#[path = "support/conformance.rs"]
mod conformance_support;

use conformance_support::{Case, Document, Exchange, ExpectedAnswer, Success};
use serde::Deserialize;
use std::collections::BTreeSet;
use thinkthen_core::adapters::systemone;
use thinkthen_core::{
    Cutting, Evidence, Find, ModelName, Plan, QuestionFile, QuestionSet, QuestionText, Threshold,
    Typed, Value, Verb, question_sha256, ranking, resolve,
};

const CASES: &str = include_str!("../../../conformance/cases.json");
const CAPTURED_REFUND: &str = include_str!(
    "../../../demos/01-refund-gate/recording/e8b7d68fe0567786d9905df174191873ff0876e0c56efc008ff7a07a4de45d3e.json"
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

type CheckedAnswers = (Vec<(Value, f64)>, String);

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

fn asked(case: &Case, exchange: &Exchange) -> Result<Asked, String> {
    if case.verb == "annotate" {
        return annotate(case, exchange);
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

fn annotate(case: &Case, exchange: &Exchange) -> Result<Asked, String> {
    let raw = case
        .question_set
        .as_ref()
        .ok_or_else(|| format!("{} has no question set", case.id))?;
    let set = QuestionSet::parse(raw.get()).map_err(|error| error.to_string())?;
    let mut questions = Vec::new();
    let mut names = Vec::new();
    let mut thresholds = Vec::new();
    let mut digests = Vec::new();
    for named in set.questions() {
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
) -> Result<CheckedAnswers, String> {
    exchange.provenance.validate(exchange)?;
    let asked = if case.verb == "find" {
        find_asked(case)?
    } else {
        asked(case, exchange)?
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
    if reply.answers().len() != asked.names.len() {
        return Err(format!(
            "{} exchange {place} has wrong answer count",
            case.id
        ));
    }
    let mut values = Vec::new();
    for (answer_place, answer) in reply.answers().iter().enumerate() {
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
        let (value, _) = answer.read(threshold);
        if !same_json(
            &serde_json::to_string(&value).map_err(|error| error.to_string())?,
            held.bare.get(),
        )? {
            return Err(format!("{} answer `{name}` has wrong bare value", case.id));
        }
        if !same_json(
            &serde_json::to_string(answer).map_err(|error| error.to_string())?,
            held.details.answer.get(),
        )? {
            return Err(format!("{} answer `{name}` has wrong details", case.id));
        }
        let digest = asked
            .digests
            .get(answer_place)
            .ok_or_else(|| "answer digest is absent".to_owned())?;
        if digest != &held.details.question_sha256 || reply.model().as_str() != held.details.model {
            return Err(format!("{} answer `{name}` has wrong metadata", case.id));
        }
        values.push((value, answer.yes().unwrap_or_default()));
    }
    Ok((values, reply.model().as_str().to_owned()))
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
            let answer = reply
                .answers()
                .first()
                .ok_or_else(|| "find has no answer".to_owned())?;
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
        for (place, exchange) in case.exchanges.iter().enumerate() {
            let (answers, reported) = validate_exchange(case, success, place, exchange)?;
            if model.as_ref().is_some_and(|held| held != &reported) {
                return Err(format!("{} has replies from different models", case.id));
            }
            model = Some(reported);
            for (value, probability) in answers {
                values.push(value);
                odds.push(probability);
            }
        }
        if success.answers.len() != values.len() {
            return Err(format!("{} has an unmatched expected answer", case.id));
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
