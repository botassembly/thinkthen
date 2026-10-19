//! Preview admitted native requests before any function check sends.

use super::{RELATE, TEXT, choice, decision, score, tags};
use thinkthen::{Engine, PlanEstimate};
use crate::cli::plan_counts::Counts;
use crate::failure::Failure;
use crate::{RequestEnvironment as Environment, RequestInput as Input};

/// One line per function and the bound before retries and refusal splits.
pub(in crate::cli::check) fn prepare(engine: &Engine) -> Result<(Vec<String>, usize), Failure> {
    let decide = decision().map_err(invalid)?;
    let definitions = [
        (
            "decide",
            crate::RequestFunction::Decide,
            decide.clone().into(),
            false,
        ),
        (
            "choose",
            crate::RequestFunction::Choose,
            choice().map_err(invalid)?.into(),
            false,
        ),
        (
            "tag",
            crate::RequestFunction::Tag,
            tags().map_err(invalid)?.into(),
            false,
        ),
        (
            "score",
            crate::RequestFunction::Score,
            score().map_err(invalid)?.into(),
            false,
        ),
        (
            "filter",
            crate::RequestFunction::Filter,
            decide.clone().into(),
            false,
        ),
        (
            "rank",
            crate::RequestFunction::Rank,
            crate::Question::rank("Did the parcel arrive undamaged?")
                .map_err(invalid)?
                .into(),
            true,
        ),
    ];
    let mut summaries = Vec::new();
    for (name, call, definition, native_error) in definitions {
        let request = admitted(call, definition, text(TEXT))?;
        let summary = engine
            .plan_request(&request, Environment::default())
            .map_err(|error| {
                if native_error {
                    Failure::from(error)
                } else {
                    Failure::Usage("a backend setup cannot fit a minimal function check")
                }
            })?;
        summaries.push((name, summary));
    }
    staged(engine, decide, &mut summaries)?;
    let mut requests = 0;
    let lines = summaries
        .into_iter()
        .map(|(name, summary)| {
            let mut counts = Counts::from(&summary);
            // These fixed whole-set checks historically count one probe input,
            // independently of its candidate/entity count. Native preparation
            // still supplies every body byte and accounting estimate.
            if matches!(name, "find" | "relate") {
                counts.records = 1;
                counts.upper_bound = false;
            }
            if name == "recognize" {
                // The fixed check presents one possible kind request after its
                // native boundary bodies, as specification/check.md defines.
                counts.requests = summary.prepared_requests().len().checked_add(1).ok_or_else(|| invalid(()))?;
                counts.upper_bound = true;
            }
            requests += counts.requests;
            Ok(format!(
                "function-plan {name} {}",
                crate::render::json_line(&counts)?
            ))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    Ok((lines, requests))
}

fn staged(
    engine: &Engine,
    decide: crate::Question,
    summaries: &mut Vec<(&'static str, PlanEstimate)>,
) -> Result<(), Failure> {
    let find = admitted(
        crate::RequestFunction::Find,
        crate::Question::find("Which text says when the parcel arrived?")
            .map_err(invalid)?
            .into(),
        Input::Units {
            items: vec![item(TEXT), item("The box was intact.")],
        },
    )?;
    let summary = engine.plan_request(&find, Environment::default()).map_err(Failure::from)?;
    summaries.push(("find", summary));
    let annotation = admitted(
        crate::RequestFunction::Annotate,
        crate::QuestionSet::builder()
            .question("intact", decide)
            .map_err(invalid)?
            .build()
            .map_err(invalid)?
            .into(),
        text(TEXT),
    )?;
    let summary = engine
        .plan_request(&annotation, Environment::default())
        .map_err(Failure::from)?;
    summaries.push(("annotate", summary));
    let recognition = admitted(
        crate::RequestFunction::Recognize,
        crate::Recognize::builder()
            .kind(crate::Kind::new("person", None).map_err(invalid)?)
            .map_err(invalid)?
            .build()
            .map_err(invalid)?
            .into(),
        text("Ada"),
    )?;
    let summary = engine.plan_request(&recognition, Environment::default()).map_err(Failure::from)?;
    summaries.push(("recognize", summary));
    let relation = admitted(
        crate::RequestFunction::Relate,
        crate::Relate::from_json(RELATE).map_err(invalid)?.into(),
        Input::Entities {
            items: vec![entity("Ada")?, entity("Bo")?],
        },
    )?;
    let summary = engine.plan_request(&relation, Environment::default()).map_err(Failure::from)?;
    summaries.push(("relate", summary));
    Ok(())
}

fn admitted(
    call: crate::RequestFunction,
    definition: crate::RequestDefinition,
    input: crate::RequestInput,
) -> Result<crate::AdmittedRequest, Failure> {
    crate::Request::for_function(
        call,
        crate::RequestArguments {
            question: crate::RequestQuestion::Definition { value: definition },
            input,
            options: crate::RequestOptions::default(),
        },
    )
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

fn invalid<T>(_error: T) -> Failure {
    Failure::Defect("a fixed backend-check input no longer plans")
}
