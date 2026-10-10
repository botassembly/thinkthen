//! Installed Rust caller of admitted Requests and Rust-owned typed results.
#![allow(
    clippy::print_stdout,
    reason = "the fixture writes its actual native result"
)]
mod fixture;
use fixture::Fixture;
use std::io::Write;
use thinkthen::{
    CallOptions, CancelToken, Error, RequestEnvironment, RequestOutcome, RequestValue,
};

fn run(f: Fixture) -> Result<String, Error> {
    let request = f.request()?;
    let admitted = fixture::admit(request)?;
    let engine = fixture::engine(f.settings.clone())?;
    let stop = CancelToken::new();
    if f.cancel {
        stop.cancel();
    }
    let worker = f.held_cancel.then(|| {
        let held = stop.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(150));
            held.cancel();
        })
    });
    let options = CallOptions::new().cancel(&stop);
    if f.batch_probe {
        println!("ready");
        std::io::stdout()
            .flush()
            .map_err(|_| fixture::usage("fixture output failed"))?;
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .map_err(|_| fixture::usage("fixture input failed"))?;
    }
    let streamed;
    let records = if f.incremental {
        Some(fixture::inputs(&admitted)?)
    } else {
        None
    };
    let (execution, feed) = if let Some(records) = records {
        streamed = f
            .request_input(thinkthen::RequestInput::Feed {
                name: "fixture".into(),
                framing: thinkthen::RequestFraming::Document,
                reading: thinkthen::ReaderOptions::default(),
                images: vec![],
            })?
            .admit()?;
        let feed = thinkthen::RequestFeed::from_records("fixture", records);
        let feed = if f.verb == "filter" {
            feed.with_all_filter_results()
        } else {
            feed
        };
        (&streamed, Some(feed))
    } else {
        (&admitted, None)
    };
    let outcome = engine.execute_request(
        execution,
        RequestEnvironment {
            controls: options,
            feed,
        },
    );
    if let Some(worker) = worker {
        worker
            .join()
            .map_err(|_| fixture::usage("fixture cancellation failed"))?;
    }
    match outcome? {
        RequestOutcome::Complete(call) => {
            check(call.value());
            Ok(fixture::written(call)?.0)
        }
        RequestOutcome::Failed { completed, error } => {
            check(&completed);
            let facts = error.facts().and_then(thinkthen::Facts::complete);
            let facts_json =
                serde_json::to_string(&facts).map_err(|_| fixture::usage("invalid facts"))?;
            let prefix = fixture::packet(&completed, facts)?;
            Ok(format!(
                "{{\"error\":{},\"facts\":{},\"completed\":{prefix}}}",
                fixture::failure(&error)?,
                facts_json
            ))
        }
    }
}
fn check(value: &RequestValue) {
    match value {
        RequestValue::Decisions(rows) => {
            for row in rows {
                let _: &fixture::Original = row.original();
                let _ = (
                    row.original(),
                    row.result().value(),
                    row.result().probabilities(),
                );
            }
        }
        RequestValue::Choices(rows) => {
            for row in rows {
                let _ = (
                    row.original(),
                    row.result().value(),
                    row.result().probabilities(),
                );
            }
        }
        RequestValue::Tags(rows) => {
            for row in rows {
                let _ = (
                    row.original(),
                    row.result().value(),
                    row.result().probabilities(),
                );
            }
        }
        RequestValue::Scores(rows) => {
            for row in rows {
                let _ = (
                    row.original(),
                    row.result().value(),
                    row.result().probabilities(),
                );
            }
        }
        RequestValue::Filtered(rows) => {
            for row in rows {
                let _ = (row.original(), row.result().value());
            }
        }
        RequestValue::Ranked(rows) => {
            for row in rows {
                let _ = (row.ordinal(), row.original(), row.result().probabilities());
            }
        }
        RequestValue::SetRanked(rows) => {
            for row in rows {
                let _ = (row.ordinal(), row.original(), row.result().members());
            }
        }
        RequestValue::Found(found) => {
            let _ = (found.selection(), found.candidates(), found.answer_id());
        }
        RequestValue::Annotations(rows) => {
            for row in rows {
                let _ = (row.original(), row.result().members());
            }
        }
        RequestValue::Recognized(rows) => {
            for row in rows {
                let _ = (row.original(), row.result().value().entities());
            }
        }
        RequestValue::Related(row) => {
            let _ = (row.original(), row.result().members());
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let result = serde_json::from_str(&input)
        .map_err(|_| fixture::usage("invalid canonical request"))
        .and_then(run);
    match result {
        Ok(packet) => println!("{packet}"),
        Err(error) => println!(
            "{{\"error\":{},\"facts\":{}}}",
            fixture::failure(&error)?,
            serde_json::to_string(&error.facts().and_then(thinkthen::Facts::complete))?
        ),
    }
    Ok(())
}
