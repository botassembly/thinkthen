//! Preview admitted native requests before any function check sends.

use super::{RELATE, TEXT, choice, decision, score, tags};
use crate::core::{self, PlanSummary};
use crate::engine::facade::Engine;
use crate::failure::Failure;
use crate::{RequestCall as Call, RequestEnvironment as Environment, RequestInput as Input};

/// One line per function and the bound before retries and refusal splits.
pub(in crate::cli::check) fn prepare(inner: &Engine) -> Result<(Vec<String>, usize), Failure> {
    let engine = crate::public::Engine::for_check(inner);
    let decide = decision().map_err(invalid)?;
    let definitions = [
        ("decide", Call::Decide as fn(_) -> _, decide.clone().into()),
        ("choose", Call::Choose, choice().map_err(invalid)?.0.into()),
        ("tag", Call::Tag, tags().map_err(invalid)?.0.into()),
        ("score", Call::Score, score().map_err(invalid)?.into()),
        ("filter", Call::Filter, decide.clone().into()),
        (
            "rank",
            Call::Rank,
            crate::Question::rank("Did the parcel arrive undamaged?")
                .map_err(invalid)?
                .into(),
        ),
    ];
    let mut summaries = Vec::new();
    for (name, call, definition) in definitions {
        let request = admitted(call, definition, text(TEXT))?;
        let (summary, _) = engine
            .plan_request_summary(&request, Environment::default())
            .map_err(|error| {
                if name == "rank" {
                    Failure::from(error)
                } else {
                    Failure::Usage("a backend setup cannot fit a minimal function check")
                }
            })?;
        summaries.push((name, summary));
    }
    staged(inner, &engine, decide, &mut summaries)?;
    let mut requests = 0;
    let lines = summaries
        .into_iter()
        .map(|(name, summary)| {
            let counts = summary.counts().map_err(|_| invalid(()))?;
            requests += counts.requests;
            Ok(format!(
                "function-plan {name} {}",
                core::json_line(&counts)?
            ))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    Ok((lines, requests))
}

fn staged(
    inner: &Engine,
    engine: &crate::Engine,
    decide: crate::Question,
    summaries: &mut Vec<(&'static str, PlanSummary)>,
) -> Result<(), Failure> {
    let find = admitted(
        Call::Find,
        crate::Question::find("Which text says when the parcel arrived?")
            .map_err(invalid)?
            .into(),
        Input::Units {
            items: vec![item(TEXT), item("The box was intact.")],
        },
    )?;
    let (_, summary) = find
        .plan_find(engine, Environment::default())
        .map_err(Failure::from)?;
    summaries.push((
        "find",
        probe_summary(inner.backend(), summary.first_body().into_iter(), 0)?,
    ));
    let annotation = admitted(
        Call::Annotate,
        crate::QuestionSet::builder()
            .question("intact", decide)
            .map_err(invalid)?
            .build()
            .map_err(invalid)?
            .into(),
        text(TEXT),
    )?;
    let (summary, _, _) = engine
        .plan_annotation_request(&annotation, Environment::default())
        .map_err(Failure::from)?;
    summaries.push(("annotate", summary));
    let recognition = admitted(
        Call::Recognize,
        crate::Recognize::builder()
            .kind(crate::Kind::new("person", None).map_err(invalid)?)
            .map_err(invalid)?
            .build()
            .map_err(invalid)?
            .into(),
        text("Ada"),
    )?;
    let preview = recognition
        .plan_recognition(
            inner.backend(),
            inner.profile(),
            Environment::default(),
            crate::core::MAX_RECORD_BYTES,
        )
        .map_err(Failure::from)?;
    let first = preview
        .first
        .ok_or(Failure::Defect("a check preview has no first request"))?;
    // specification/check.md counts one fixed probe and one possible kind request
    // for its single-token name; native preparation owns the admitted body bytes.
    let summary = probe_summary(
        inner.backend(),
        first.requests.iter().map(|request| request.body.as_slice()),
        1,
    )?;
    summaries.push(("recognize", summary));
    let relation = admitted(
        Call::Relate,
        crate::Relate::from_json(RELATE).map_err(invalid)?.into(),
        Input::Entities {
            items: vec![entity("Ada")?, entity("Bo")?],
        },
    )?;
    let preview = relation
        .plan_relations(inner.backend(), inner.profile(), Environment::default())
        .map_err(Failure::from)?;
    let summary = probe_summary(
        inner.backend(),
        preview
            .requests
            .iter()
            .map(|request| request.body.as_slice()),
        0,
    )?;
    summaries.push(("relate", summary));
    Ok(())
}

fn admitted(
    call: fn(crate::RequestArguments) -> crate::RequestCall,
    definition: crate::RequestDefinition,
    input: crate::RequestInput,
) -> Result<crate::AdmittedRequest, Failure> {
    crate::Request::new(call(crate::RequestArguments {
        question: crate::RequestQuestion::Definition { value: definition },
        input,
        options: crate::RequestOptions::default(),
    }))
    .admit()
    .map_err(invalid)
}

fn text(text: &str) -> Input {
    Input::Records {
        items: vec![item(text)],
    }
}

fn item(text: &str) -> crate::RequestItem {
    crate::RequestItem {
        original: Some(crate::RequestOriginal::Text {
            text: text.to_owned(),
        }),
        context: None,
        options: None,
        examples: None,
        seed_spans: None,
        images: Vec::new(),
    }
}

fn entity(name: &str) -> Result<crate::RequestItem, Failure> {
    let mut item = item(name);
    item.original = Some(crate::RequestOriginal::Json {
        value: crate::RawRecord::json(&format!(r#"{{"name":"{name}","kind":"person"}}"#))
            .map_err(invalid)?,
    });
    Ok(item)
}

fn probe_summary<'a>(
    backend: &core::Backend,
    bodies: impl Iterator<Item = &'a [u8]>,
    adaptive: usize,
) -> Result<PlanSummary, Failure> {
    let mut summary = PlanSummary::new(adaptive > 0).with_accounting(backend.accounting());
    summary.record().map_err(|_| invalid(()))?;
    for body in bodies {
        summary.request(body).map_err(|_| invalid(()))?;
    }
    if adaptive > 0 {
        summary
            .possible_requests(adaptive)
            .map_err(|_| invalid(()))?;
    }
    Ok(summary)
}

fn invalid<T>(_error: T) -> Failure {
    Failure::Defect("a fixed backend-check input no longer plans")
}
