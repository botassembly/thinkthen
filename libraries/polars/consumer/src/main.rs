//! Actual Rust Polars consumer using shared native fixture framing and projection.
#![allow(
    clippy::print_stdout,
    reason = "the conformance child writes its actual native projection"
)]
#[path = "../../../rust/consumer/src/fixture.rs"]
mod fixture;
use fixture::{Fixture, Original};
use std::io::Write;
use thinkthen::RequestDefinition as Asked;
use thinkthen::polars::prelude::{NamedFrom, Series};
use thinkthen::{Call, CallOptions, CancelToken, CompleteRecord, Error, LoadedQuestion};

fn records<R>(
    call: Call<Vec<CompleteRecord<Original, R>>>,
    check: impl Fn(&R),
) -> Result<(String, thinkthen::Facts), Error>
where
    CompleteRecord<Original, R>: serde::Serialize,
{
    for row in call.value() {
        check(row.result());
    }
    fixture::written(call)
}
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
    let options = if let Some(context) = f.shared_context.as_deref() {
        options.context(context)
    } else {
        options
    };
    let inputs = fixture::inputs(&admitted)?.collect::<Result<Vec<_>, _>>()?;
    let column = Series::new(
        "original".into(),
        inputs
            .iter()
            .map(|i| {
                serde_json::to_string(&i.original)
                    .map_err(|_| fixture::usage("invalid original frame value"))
            })
            .collect::<Result<Vec<_>, Error>>()?,
    );
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
    let result = match (f.verb.as_str(), asked) {
        ("decide", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine
                .decide_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _: thinkthen::Probabilities = r.probabilities();
                let _ = r.answer_id();
            },
        ),
        ("decide", Asked::Atomic(LoadedQuestion::Banded(q))) => records(
            engine
                .decide_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _: thinkthen::Probabilities = r.probabilities();
            },
        ),
        ("choose", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine
                .choose_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _ = r.probabilities();
                let _ = r.question();
            },
        ),
        ("choose", Asked::DynamicChoose(q)) => records(
            engine
                .choose_dynamic_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _ = r.probabilities();
            },
        ),
        ("tag", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine
                .tag_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _ = r.probabilities();
            },
        ),
        ("score", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine
                .score_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _: f64 = r.value();
                let _ = r.probabilities();
            },
        ),
        ("filter", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine
                .filter_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _: bool = r.value();
            },
        ),
        ("rank", Asked::Rank(q)) => records(
            engine
                .rank_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _: usize = r.value();
                let _ = r.probabilities();
            },
        ),
        ("rank", Asked::RankSet(q)) => records(
            engine
                .rank_set_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                let _: usize = r.value();
                for member in r.members() {
                    let _: &str = member.name();
                    let result: &thinkthen::CompleteRank = member.result();
                    let _ = result.answer_id();
                    let _ = result.identity();
                    let _: usize = result.value();
                    let _ = result.probabilities();
                }
            },
        ),
        ("annotate", Asked::Annotate(q)) => records(
            engine
                .annotate_input_column_complete(&q, &column, inputs, options)?
                .0,
            |r| {
                for member in r.members() {
                    let _ = member.value();
                    let _ = member.failure();
                }
            },
        ),
        ("find", Asked::Find(q)) => {
            let question = if f.options.none {
                q.question().clone().offering_none()?
            } else {
                q.question().clone()
            };
            let call = engine
                .find_input_column_complete(&question, &column, inputs, options)?
                .0;
            let _: &thinkthen::AnswerId = call.value().answer_id();
            fixture::written(call)
        }
        ("recognize", Asked::Recognize(q)) => records(
            engine
                .recognize_input_column_complete(q.question(), &column, inputs, options)?
                .0,
            |r| {
                for entity in r.value().entities() {
                    let _ = entity.start();
                    let _ = entity.end();
                }
            },
        ),
        ("relate", Asked::Relate(q)) => {
            let call = engine
                .relate_input_column_complete(&q, &column, inputs, options)?
                .0;
            for member in call.value().result().members() {
                let _ = member.source();
                let _ = member.target();
                let _ = member.probabilities();
            }
            fixture::written(call)
        }
        _ => {
            return Err(fixture::usage("question does not match the named function"));
        }
    }?;
    let _: Option<&thinkthen::CallId> = result.1.call_id();
    let _: u64 = result.1.requests_sent();
    Ok(result.0)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let result = serde_json::from_str(&input)
        .map_err(|_| fixture::usage("invalid canonical request"))
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
