//! Actual Rust consumer of all ten concrete native public APIs and typed readings.
#![allow(
    clippy::print_stdout,
    reason = "the conformance child writes its actual native projection"
)]
#[allow(
    dead_code,
    reason = "this Rust consumer calls concrete engine APIs; the host bridge is exercised by SDK consumers"
)]
use thinkthen_host::complete;
mod native_settings;
use complete::inputs::Original;
use complete::questions::Asked;
use serde::Deserialize;
use std::io::Write;
use thinkthen::{Call, CallOptions, CancelToken, CompleteRecord, Error, LoadedQuestion};

#[derive(Deserialize)]
struct Fixture {
    verb: String,
    question: Box<serde_json::value::RawValue>,
    input: Box<serde_json::value::RawValue>,
    settings: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    cancel: bool,
    #[serde(default)]
    held_cancel: bool,
    #[serde(default)]
    incremental: bool,
    #[serde(default)]
    batch_probe: bool,
    deadline_ms: Option<i64>,
    shared_context: Option<String>,
}
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
    let inputs = call.value().iter().map(|r| r.original().view()).collect();
    complete::rows(call, inputs)
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
            .map_err(|_| complete::usage("fixture output failed"))?;
        let mut ready = String::new();
        std::io::stdin()
            .read_line(&mut ready)
            .map_err(|_| complete::usage("fixture input failed"))?;
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
    #[derive(serde::Serialize)]
    struct Packet<'a, T> {
        results: &'a T,
        ordinals: Vec<Option<usize>>,
        inputs: Vec<complete::inputs::InputView>,
        facts: Option<thinkthen::CompleteFacts<'a>>,
    }
    let packet = Packet {
        ordinals: results.iter().map(|r| Some(r.ordinal())).collect(),
        inputs: results.iter().map(|r| r.original().view()).collect(),
        results: &results,
        facts,
    };
    let prefix = serde_json::to_string(&packet)
        .map_err(|_| complete::usage("invalid native fixture result"))?;
    if let Some(error) = failure {
        let snapshot = complete::stream::failure(&error)?;
        return Ok(format!(
            "{{\"error\":{snapshot},\"facts\":{},\"completed\":{prefix}}}",
            serde_json::to_string(&error.facts().and_then(thinkthen::Facts::complete))
                .map_err(|_| complete::usage("invalid native facts"))?
        ));
    }
    Ok(prefix)
}
#[expect(
    clippy::too_many_lines,
    reason = "the consumer compiles all ten concrete native APIs and their known fields in one dispatch"
)]
fn run(f: Fixture) -> Result<String, Error> {
    let engine = native_settings::engine(f.settings)?;
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
    // Caller fixtures are JSON. Every known response is reached through its concrete Rust type.
    #[derive(serde::Serialize)]
    struct Written<'a> {
        verb: &'a str,
        question: &'a serde_json::value::RawValue,
        input: &'a serde_json::value::RawValue,
        attempts: bool,
        context: Option<&'a str>,
    }
    let written = Written {
        verb: &f.verb,
        question: &f.question,
        input: &f.input,
        attempts: true,
        context: f.shared_context.as_deref(),
    };
    let text = serde_json::to_string(&written).map_err(|_| complete::usage("invalid fixture"))?;
    let request = complete::parse(&text)?;
    let options = if let Some(context) = f.shared_context.as_deref() {
        options.context(context)
    } else {
        options
    };
    let asked = complete::questions::load(&request.verb, request.question)?;
    if f.incremental {
        let inputs = complete::inputs::iter(
            &engine,
            request.input,
            asked.reading(),
            request.verb == "annotate",
        )?;
        return match (request.verb.as_str(), asked) {
            ("decide", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine.try_decide_records_complete_with(&q, inputs, options),
                |r| {
                    let _: thinkthen::Probabilities = r.probabilities();
                },
                f.batch_probe,
            ),
            ("decide", Asked::Atomic(LoadedQuestion::Banded(q))) => streamed(
                engine.try_decide_records_complete_with(&q, inputs, options),
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("choose", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine.try_choose_records_complete_with(&q, inputs, options),
                |r| {
                    let _ = r.value();
                },
                f.batch_probe,
            ),
            ("choose", Asked::Dynamic(q)) => streamed(
                engine.try_choose_dynamic_records_complete_with(&q, inputs, options),
                |r| {
                    let _: thinkthen::Probabilities = r.probabilities();
                },
                f.batch_probe,
            ),
            ("tag", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine.try_tag_records_complete_with(&q, inputs, options),
                |r| {
                    let _ = r.probabilities();
                },
                f.batch_probe,
            ),
            ("score", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine.try_score_records_complete_with(&q, inputs, options),
                |r| {
                    let _: f64 = r.value();
                },
                f.batch_probe,
            ),
            ("filter", Asked::Atomic(LoadedQuestion::Question(q))) => streamed(
                engine.try_filter_records_complete_with(&q, inputs, options),
                |r| {
                    let _: bool = r.value();
                },
                f.batch_probe,
            ),
            ("annotate", Asked::Set(q)) => streamed(
                engine.try_annotate_records_complete_with(&q, inputs, options),
                |r| {
                    let _ = r.members();
                },
                f.batch_probe,
            ),
            _ => Err(complete::usage(
                "batch question does not match the named function",
            )),
        };
    }
    let inputs = complete::inputs::read(
        &engine,
        request.input,
        asked.reading(),
        request.verb == "annotate",
    )?;
    let result = match (request.verb.as_str(), asked) {
        ("decide", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine.decide_records_complete_with(&q, inputs, options)?,
            |r| {
                let _: thinkthen::Probabilities = r.probabilities();
                let _ = r.answer_id();
            },
        ),
        ("decide", Asked::Atomic(LoadedQuestion::Banded(q))) => records(
            engine.decide_records_complete_with(&q, inputs, options)?,
            |r| {
                let _: thinkthen::Probabilities = r.probabilities();
            },
        ),
        ("choose", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine.choose_records_complete_with(&q, inputs, options)?,
            |r| {
                let _ = r.probabilities();
                let _ = r.question();
            },
        ),
        ("choose", Asked::Dynamic(q)) => records(
            engine.choose_dynamic_records_complete_with(&q, inputs, options)?,
            |r| {
                let _ = r.probabilities();
            },
        ),
        ("tag", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine.tag_records_complete_with(&q, inputs, options)?,
            |r| {
                let _ = r.probabilities();
            },
        ),
        ("score", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine.score_records_complete_with(&q, inputs, options)?,
            |r| {
                let _: f64 = r.value();
                let _ = r.probabilities();
            },
        ),
        ("filter", Asked::Atomic(LoadedQuestion::Question(q))) => records(
            engine.filter_records_complete_with(&q, inputs, options)?,
            |r| {
                let _: bool = r.value();
            },
        ),
        ("rank", Asked::Rank(q)) => records(
            engine.rank_records_complete_with(&q, inputs, options)?,
            |r| {
                let _: usize = r.value();
                let _ = r.probabilities();
            },
        ),
        ("rank", Asked::SetRank(q)) => records(
            engine.rank_set_records_complete_with(&q, inputs, options)?,
            |r| {
                let _: usize = r.value();
                for member in r.members() {
                    let _: &str = member.name();
                    let child: &thinkthen::CompleteRank = member.result();
                    let _: usize = child.value();
                    let _: &str = child.answer_id().as_str();
                    let _ = child.question().name();
                    let _ = child.question().wording_version();
                    let _ = child.probabilities();
                    let _ = child.meta().context_sha256();
                    let _ = child.identity().question_sources();
                    let _ = child.reported_usage();
                }
            },
        ),
        ("annotate", Asked::Set(q)) => records(
            engine.annotate_records_complete_with(&q, inputs, options)?,
            |r| {
                for member in r.members() {
                    let _ = member.value();
                    let _ = member.failure();
                }
            },
        ),
        ("find", Asked::Find(q, _)) => {
            let views = inputs.iter().map(|i| i.original.view()).collect();
            let call = engine.find_records_complete_with(&q, inputs, options)?;
            let _: &thinkthen::AnswerId = call.value().answer_id();
            let at = match call.value().selection() {
                thinkthen::FindSelection::None => None,
                thinkthen::FindSelection::Unit(at) => Some(at),
            };
            complete::written(call, vec![at], views)
        }
        ("recognize", Asked::Recognize(q, _)) => records(
            engine.recognize_records_complete_with(&q, inputs, options)?,
            |r| {
                for entity in r.value().entities() {
                    let _ = entity.start();
                    let _ = entity.end();
                }
            },
        ),
        ("relate", Asked::Relate(q)) => {
            let views = inputs.iter().map(|i| i.original.view()).collect();
            let call = engine.relate_records_complete_with(&q, inputs, options)?;
            for member in call.value().result().members() {
                let _ = member.source();
                let _ = member.target();
                let _ = member.probabilities();
            }
            complete::written(call, vec![None], views)
        }
        _ => {
            return Err(complete::usage(
                "question does not match the named function",
            ));
        }
    }?;
    let _: Option<&thinkthen::CallId> = result.1.call_id();
    let _: u64 = result.1.requests_sent();
    Ok(result.0)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let fixture = serde_json::from_str(&input)?;
    match run(fixture) {
        Ok(packet) => println!("{packet}"),
        Err(error) => {
            let snapshot = complete::stream::failure(&error)?;
            let facts = error.facts().and_then(thinkthen::Facts::complete);
            println!(
                "{{\"error\":{snapshot},\"facts\":{}}}",
                serde_json::to_string(&facts)?
            );
        }
    }
    Ok(())
}
