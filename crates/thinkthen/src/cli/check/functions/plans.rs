//! Preflight the same ordinary and staged production planners before any send.

use super::{RELATE, TEXT, choice, decision, score, tags};
use crate::core::{self, Evidence, PlanSummary, QuestionText};
use crate::engine::facade::{self, Asks, Bound, Engine};
use crate::failure::Failure;

/// One line per function and the bound on prepared requests before retries and refusal splits.
pub(in crate::cli::check) fn prepare(inner: &Engine) -> Result<(Vec<String>, usize), Failure> {
    let engine = crate::public::Engine::for_check(inner);
    let plan_error = |_| Failure::Usage("a backend setup cannot fit a minimal function check");
    let decide = decision().map_err(invalid)?;
    let choice = choice().map_err(invalid)?;
    let tag = tags().map_err(invalid)?;
    let score = score().map_err(invalid)?;
    let rank =
        crate::public::Question::rank("Did the parcel arrive undamaged?").map_err(invalid)?;
    let estimates = [
        ("decide", engine.plan(&decide, [TEXT])),
        ("choose", engine.plan(&choice, [TEXT])),
        ("tag", engine.plan(&tag, [TEXT])),
        ("score", engine.plan(&score, [TEXT])),
        ("filter", engine.plan(&decide, [TEXT])),
    ];
    let mut lines = Vec::new();
    let mut requests = 0;
    for (name, estimate) in estimates {
        let estimate = estimate.map_err(plan_error)?;
        requests += estimate.requests();
        lines.push(format!(
            "function-plan {name} {}",
            core::json_line(&estimate)?
        ));
    }
    quoted("rank", inner, rank.core, &mut lines, &mut requests)?;
    let backend = inner.backend();
    let evidence = Evidence::new(TEXT).map_err(invalid)?;
    let find = core::Find::new(
        QuestionText::new("Which text says when the parcel arrived?").map_err(invalid)?,
        &[
            evidence.clone(),
            Evidence::new("The box was intact.").map_err(invalid)?,
        ],
        backend.model().clone(),
        false,
    )
    .map_err(invalid)?;
    inner.check_plan(find.plan())?;
    let mut asks = Asks::default();
    asks.add(backend, find.plan())?;
    add(
        backend,
        "find",
        asks.requests(backend, inner.profile(), Bound::WHOLE)?,
        0,
        &mut lines,
        &mut requests,
    )?;
    quoted("annotate", inner, decide.core, &mut lines, &mut requests)?;
    let spec =
        core::RecognizeSpec::from_parts(vec![("person".to_owned(), None)], Vec::new(), None, None)
            .map_err(invalid)?;
    let (_, _, prepared) = facade::step_one(
        backend,
        inner.profile(),
        &spec,
        "Ada",
        facade::MAX_TEXT_BYTES,
    )?;
    // One token can produce at most one name. Its kind takes one request;
    // no alternate edges or relations exist in this recognition input.
    add(backend, "recognize", prepared, 1, &mut lines, &mut requests)?;
    let spec = core::RelateSpec::parse(RELATE).map_err(invalid)?;
    let entities = spec
        .admit(&[
            ("Ada".to_owned(), "person".to_owned()),
            ("Bo".to_owned(), "person".to_owned()),
        ])
        .map_err(invalid)?;
    let prepared = facade::relations(&entities, &spec, backend, inner.profile())?;
    add(
        backend,
        "relate",
        prepared.requests,
        0,
        &mut lines,
        &mut requests,
    )?;
    Ok((lines, requests))
}

fn quoted(
    name: &str,
    inner: &Engine,
    question: core::Question,
    lines: &mut Vec<String>,
    requests: &mut usize,
) -> Result<(), Failure> {
    let backend = inner.backend();
    let plan = core::quoted_plan(
        backend.asked(),
        Evidence::new(TEXT).map_err(invalid)?,
        None,
        vec![question],
        inner.profile(),
    )
    .map_err(|error| match error {
        core::BatchError::Profile(limit) => Failure::ProfileLimit(limit),
        _ => Failure::Defect("a fixed function check no longer plans"),
    })?;
    let mut asks = Asks::default();
    asks.add(backend, &plan)?;
    add(
        backend,
        name,
        asks.requests(backend, inner.profile(), Bound::WHOLE)?,
        0,
        lines,
        requests,
    )
}

fn add(
    backend: &core::Backend,
    name: &str,
    prepared: Vec<facade::Request>,
    adaptive: usize,
    lines: &mut Vec<String>,
    requests: &mut usize,
) -> Result<(), Failure> {
    let mut summary = PlanSummary::new(adaptive > 0).with_accounting(backend.accounting());
    summary
        .record()
        .map_err(|_| Failure::Defect("a check plan is too large"))?;
    for request in prepared {
        summary
            .request(&request.body)
            .map_err(|_| Failure::Defect("a check plan is too large"))?;
    }
    if adaptive > 0 {
        summary
            .possible_requests(adaptive)
            .map_err(|_| Failure::Defect("a check plan is too large"))?;
    }
    let counts = summary
        .counts()
        .map_err(|_| Failure::Defect("a check plan is too large"))?;
    *requests += counts.requests;
    lines.push(format!(
        "function-plan {name} {}",
        core::json_line(&counts)?
    ));
    Ok(())
}

fn invalid<T>(_error: T) -> Failure {
    Failure::Defect("a fixed backend-check input no longer parses")
}
