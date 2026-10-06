//! One requested, compact run line after every human diagnostic.

use std::io::Write;
use std::time::Duration;

use serde::Serialize;

use crate::cli::args::Command;
use crate::cli::failure::facts::Stopped;
use crate::engine::usage::RunSnapshot;

#[derive(Serialize)]
struct Line {
    schema: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    call_id: Option<crate::core::CallId>,
    records: u64,
    requests_sent: u64,
    retries: u64,
    cache_answers: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    estimated_cost_usd: Option<String>,
    seconds: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stopped: Option<Stopped>,
}

pub(crate) fn enabled(command: &Command) -> bool {
    match command {
        Command::Decide(value) => value.common.facts,
        Command::Choose(value) => value.common.facts,
        Command::Tag(value) => value.common.facts,
        Command::Score(value) => value.common.facts,
        Command::Filter(value) => value.common.facts,
        Command::Rank(value) => value.common.facts,
        Command::Find(value) => value.common.facts,
        Command::Annotate(value) => value.common.facts,
        Command::Recognize(value) => value.common.facts,
        Command::Relate(value) => value.common.facts,
        Command::Cache(_)
        | Command::Status(_)
        | Command::Check(_)
        | Command::Backends(_)
        | Command::Transform(_)
        | Command::Runs(_)
        | Command::Audit(_)
        | Command::Diff(_) => false,
    }
}

pub(crate) fn write(
    mut writer: impl Write,
    snapshot: RunSnapshot,
    elapsed: Duration,
    stopped: Option<Stopped>,
    call_id: Option<crate::core::CallId>,
) {
    let line = Line {
        schema: "thinkthen.run/1",
        call_id,
        records: snapshot.records,
        requests_sent: snapshot.counts.requests_sent,
        retries: snapshot.counts.retries,
        cache_answers: snapshot.counts.cache_answers,
        input_tokens: snapshot
            .reported
            .and_then(crate::core::ReportedUsage::input_tokens),
        output_tokens: snapshot
            .reported
            .and_then(crate::core::ReportedUsage::output_tokens),
        estimated_cost_usd: snapshot.estimated_cost_usd,
        seconds: (elapsed.as_secs_f64() * 1000.0).round() / 1000.0,
        model: snapshot.model,
        stopped,
    };
    if let Ok(json) = serde_json::to_string(&line) {
        let _unwritten = writeln!(writer, "{json}").and_then(|()| writer.flush());
    }
}
