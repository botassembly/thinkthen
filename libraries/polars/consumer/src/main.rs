//! Actual Rust Polars consumer using shared native fixture framing and projection.
#![allow(
    clippy::print_stdout,
    reason = "the conformance child writes its actual native projection"
)]
#[expect(
    dead_code,
    reason = "the shared fixture also contains the Rust scalar writer; this consumer writes typed Polars records"
)]
#[path = "../../../rust/consumer/src/fixture.rs"]
mod fixture;
use fixture::{Fixture, Original};
mod columns;
use std::io::Write;
use thinkthen::RequestDefinition as Asked;
use thinkthen::polars::prelude::{NamedFrom, Series};

use thinkthen::{CallOptions, CancelToken, CompleteRecord, Error, LoadedQuestion};

fn streamed<R>(
    mut batch: thinkthen::Batch<'_, CompleteRecord<Original, R>>,
    check: impl Fn(&R),
    probe: bool,
) -> Result<String, Error>
where
    CompleteRecord<Original, R>: serde::Serialize,
{
    if probe {
        println!("ready");
        std::io::stdout()
            .flush()
            .map_err(|_| fixture::usage("fixture output failed"))?;
        let mut ready = String::new();
        std::io::stdin()
            .read_line(&mut ready)
            .map_err(|_| fixture::usage("fixture input failed"))?;
    }
    let mut results = Vec::new();
    let mut failure = None;
    for row in batch.by_ref() {
        match row {
            Ok(row) => {
                check(row.result());
                results.push(row);
            }
            Err(error) => {
                failure = Some(error);
                break;
            }
        }
    }
    let facts = batch.facts().and_then(thinkthen::Facts::complete);
    let prefix = fixture::packet(&results, facts)?;
    if let Some(error) = failure {
        let snapshot = fixture::failure(&error)?;
        return Ok(format!(
            "{{\"error\":{snapshot},\"facts\":{},\"completed\":{prefix}}}",
            serde_json::to_string(&error.facts().and_then(thinkthen::Facts::complete))
                .map_err(|_| fixture::usage("invalid native facts"))?
        ));
    }
    Ok(prefix)
}
#[expect(
    clippy::too_many_lines,
    reason = "the consumer compiles all ten concrete native APIs and their known fields in one dispatch"
)]
fn run(f: Fixture) -> Result<String, Error> {
    let engine = fixture::engine(f.settings.clone())?;
    let token = CancelToken::new();
    if f.cancel {
        token.cancel();
    }
    let held = token.clone();
    if f.held_cancel {
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(150));
            held.cancel();
        });
    }
    let options = CallOptions::new().cancel(&token).attempts(true);
    let options = if let Some(ms) = f.deadline_ms {
        options.deadline_ms(ms)?
    } else {
        options
    };
    let request = f.request()?;
    let admitted = fixture::admit(request.clone())?;
    let asked = admitted.resolve_question()?;
    let asked = match asked {
        Asked::Atomic(LoadedQuestion::Question(question)) if f.verb == "find" && f.options.none => {
            Asked::from(question.offering_none()?)
        }
        Asked::Find(find) if f.options.none => {
            let (question, reading) = find.into_parts();
            Asked::from((question.offering_none()?, reading))
        }
        asked => asked,
    };
    let options = if let Some(context) = f.shared_context.as_deref() {
        options.context(context)
    } else {
        options
    };
    let inputs = fixture::inputs(&admitted)?.collect::<Result<Vec<_>, _>>()?;
    let column = if f.incremental {
        Series::new("presentation".into(), vec!["present"; inputs.len()])
    } else {
        columns::originals(
            &inputs
                .iter()
                .map(|input| {
                    serde_json::to_value(&input.original)
                        .map_err(|_| fixture::usage("invalid original frame value"))
                })
                .collect::<Result<Vec<_>, Error>>()?,
        )?
    };
    let inputs = inputs.into_iter().map(Some).collect();
    if f.incremental {
        return match (f.verb.as_str(), asked) {
            ("decide", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine
                    .decide_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("decide", Asked::Atomic(LoadedQuestion::Banded(q))) => streamed(
                engine
                    .decide_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("choose", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine
                    .choose_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("choose", Asked::DynamicChoose(q)) => streamed(
                engine
                    .choose_dynamic_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("tag", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine
                    .tag_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("score", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine
                    .score_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("filter", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine
                    .filter_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("annotate", Asked::Annotate(q)) => streamed(
                engine
                    .annotate_input_column_batch(&q, &column, inputs, options)?
                    .0,
                |r| {
                    let _ = r.members();
                },
                f.batch_probe,
            ),
            _ => Err(fixture::usage(
                "batch question does not match the named function",
            )),
        };
    }
    let function = f.verb.parse()?;
    let call = engine.records_column(function, &asked, &column, inputs, options)?;
    let records = call
        .value()
        .column("records")
        .map_err(|_| fixture::usage("missing records column"))?;
    let _ = records
        .struct_()
        .map_err(|_| fixture::usage("records are not typed structs"))?;
    let _ = call.probability().dtype();
    fixture::packet(call.results(), call.facts().complete())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let result = serde_json::from_str(&input)
        .map_err(|error| fixture::usage(&error.to_string()))
        .and_then(run);
    match result {
        Ok(packet) => println!("{packet}"),
        Err(error) => {
            let snapshot = fixture::failure(&error)?;
            let facts = error.facts().and_then(thinkthen::Facts::complete);
            println!(
                "{{\"error\":{snapshot},\"facts\":{}}}",
                serde_json::to_string(&facts)?
            );
        }
    }
    Ok(())
}
