//! One record to one request, and one reply to one row. `batched.rs` sends
//! `decide`, `filter` and `rank` over a stream in batches.
//!
//! `judge.rs` decides what a run keeps and what view it prints in.

use std::io::Read;
use std::process::ExitCode;
use std::time::Duration;

use crate::core::{
    Backend, BackendProfile, Evidence, Framing, Outcome, Pointer, Question, QuestionText, Reading,
    Record, RecordValue, Resolved, Sources, Threshold, Value, json_line,
};

use crate::args::Common;
use crate::edge::{self, Environment};
use crate::engine::Width;
use crate::engine::facade::{Engine, Judgment, Settings, Storage};
use crate::failure::Failure;
use crate::judge::{Asked, Keeping, View};
use crate::profile::{self, Mismatch};
use crate::result_json::{Run, decision};
use crate::schedule::{self, Judged, Output};
use crate::table::{Kind as TableKind, Rows as TableRows};

mod batched;
mod folders;
mod plan;

pub(crate) use folders::Folders;
use plan::{plan, plan_record, print_plan};

/// Build the one engine a command calls, from what the command resolved.
///
/// `width` is the `--jobs` a record command registered, or `None`.
pub(crate) fn engine(
    common: &Common,
    environment: &Environment,
    folders: Folders,
    backend: Backend,
    profile: Option<BackendProfile>,
    width: Option<u8>,
) -> Result<Engine, Failure> {
    let width = width.map(|jobs| Width::new(u64::from(jobs))).transpose()?;
    Ok(Engine::new(Settings {
        backend,
        profile,
        timeout: Duration::from_secs(common.timeout),
        max_retries: common.max_retries,
        retry_wait: environment.retry_wait(),
        width,
        storage: Storage {
            record: folders.record,
            replay: folders.replay,
            private_default: folders.private_default,
            cache_answers: folders.cache_answers,
        },
        key: std::sync::Arc::new(edge::key),
        usage: environment.counters(),
    })?)
}

/// Where one record's question comes from.
///
/// Every verb but `choose --options` asks the same question of every record.
/// `--options` names a pointer, and each record holds its own candidate list
/// there, so the question is built again for each one.
#[derive(Debug)]
pub(crate) enum Asks {
    /// One question, asked of every record.
    Fixed(Question),
    /// A pick whose options each record carries at this pointer.
    FromRecord {
        /// The question the model receives.
        text: QuestionText,
        /// Where in the record the candidate list sits.
        pointer: Pointer,
    },
}

impl Asks {
    /// Build the question this record is asked.
    fn of(&self, record: &Record) -> Result<Question, Failure> {
        match self {
            Self::Fixed(question) => Ok(question.clone()),
            Self::FromRecord { text, pointer } => Ok(Question::Choose {
                text: text.clone(),
                options: record.choices(pointer)?,
            }),
        }
    }
}

/// One record, the question it was asked, and the evidence it is asked of.
#[derive(Debug)]
struct Sending {
    record: Record,
    question: Question,
    evidence: Evidence,
}

/// The one question every record is asked, which every verb but one has.
///
/// # Errors
///
/// Returns [`Failure::Defect`] when a verb that settles its own list left the
/// question open, which no verb but `choose --options` can do.
pub(crate) fn fixed(settled: &Resolved) -> Result<Asks, Failure> {
    settled
        .question()
        .cloned()
        .map(Asks::Fixed)
        .ok_or(Failure::Defect(
            "a verb with no list left its question open",
        ))
}

/// Send the question over every record, and print one answer for each.
///
/// # Errors
///
/// Returns [`Failure`] for every outcome `channels.md` gives a code above 3,
/// and for every option that cannot act in the mode the run is in.
pub(crate) fn run(
    asked: Asked<'_>,
    environment: &Environment,
    input: impl Read + Send + 'static,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let Asked {
        common,
        asks,
        settled,
        view,
        keeping,
        batch,
    } = asked;
    let threshold = settled.threshold();
    let view = view.checked()?;
    let folders = Folders::of(common, environment)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let configured_model = settled
        .sources()
        .model_is_default()
        .then(|| environment.model())
        .flatten();
    let backend = Backend::resolve(
        common.url.as_deref(),
        environment.base_url(),
        configured_model.unwrap_or_else(|| settled.model().as_str()),
    )?;
    let profile = profile::read(common)?;
    let mismatch = Mismatch::new(settled.profile(), profile.as_ref());
    let reading = read_by(common, settled, keeping)?;
    if view.quiet && reading.streams() {
        return Err(Failure::QuietOverRecords);
    }
    schedule::jobs_of(common.jobs, reading.streams())?;
    let batch = batch.map_or(Ok(None), |tiers| {
        tiers.setting(environment, reading.streams())
    })?;
    let source = edge::source(common.input.as_deref(), input)?;
    let configuration = JudgingInput {
        common,
        environment,
        folders,
        backend,
        asks,
        threshold,
        view,
        keeping,
        streams: reading.streams(),
        profile,
        mismatch,
        sources: (settled.sources().question_is_from_file() || configured_model.is_some()).then(
            || {
                if configured_model.is_some() {
                    settled.sources().with_configuration_model()
                } else {
                    *settled.sources()
                }
            },
        ),
    };

    if let Some(setting) = batch {
        return batched::run(configuration, &reading, source, setting, output);
    }

    if let Some(kind) = table_kind(common) {
        return over_table(configuration, &reading, source, kind, output);
    }

    let mut chunks = edge::Chunks::new(source, reading.streams());

    if common.dry_run {
        return plan(
            &configuration.backend,
            configuration.profile.as_ref(),
            &configuration.mismatch,
            &reading,
            &configuration.planning(),
            chunks.next().transpose()?,
            output.writer(),
        );
    }

    let judging = Judging::new(configuration)?;
    if !judging.streams {
        let bytes = chunks.next().transpose()?.unwrap_or_default();
        let judged = judging.row(&reading, &bytes)?;
        let outcome = judged.outcome;
        output.take(judged)?;
        return Ok(exit_code(outcome));
    }
    schedule::over_records(
        &judging.engine,
        &|bytes: &Vec<u8>| judging.row(&reading, bytes),
        chunks,
        judging.environment.cancel(),
        output,
    )
}

fn over_table(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    source: Box<dyn std::io::BufRead + Send>,
    kind: TableKind,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let mut rows = TableRows::new(source, kind)?;
    if configuration.common.dry_run {
        return plan_record(
            &configuration.backend,
            configuration.profile.as_ref(),
            &configuration.mismatch,
            reading,
            &configuration.planning(),
            rows.next().transpose()?,
            output.writer(),
        );
    }
    let judging = Judging::new(configuration)?;
    schedule::over_records(
        &judging.engine,
        &|record| judging.typed_row(reading, record),
        rows,
        judging.environment.cancel(),
        output,
    )
}

/// Read the framing the command line asked for, over the settled pointers.
/// With no flag, `filter` and `rank` read lines, or JSON Lines under a pointer.
fn read_by(common: &Common, settled: &Resolved, keeping: Keeping) -> Result<Reading, Failure> {
    let on = settled.on().to_vec();
    let asked = common.framing();
    if asked != Framing::Document || !keeping.streams_only() {
        return Ok(Reading::new(asked, on)?);
    }
    let framing = if on.is_empty() {
        Framing::Lines
    } else {
        Framing::Jsonl
    };
    Ok(Reading::new(framing, on)?.by_default())
}

fn table_kind(common: &Common) -> Option<TableKind> {
    common
        .csv
        .then_some(TableKind::Csv)
        .or_else(|| common.tsv.then_some(TableKind::Tsv))
}

/// Read one record and the question it is asked, which both paths do.
fn asked_of(reading: &Reading, record: Record, asks: &Asks) -> Result<Sending, Failure> {
    Ok(Sending {
        question: asks.of(&record)?,
        evidence: reading.evidence(&record)?,
        record,
    })
}

/// One question over one engine, asked of every record in turn.
struct Judging<'a> {
    environment: &'a Environment,
    engine: Engine,
    asks: Asks,
    threshold: Option<Threshold>,
    view: View,
    keeping: Keeping,
    streams: bool,
    mismatch: Mismatch,
}

struct JudgingInput<'a> {
    common: &'a Common,
    environment: &'a Environment,
    folders: Folders,
    backend: Backend,
    asks: Asks,
    threshold: Option<Threshold>,
    view: View,
    keeping: Keeping,
    streams: bool,
    sources: Option<Sources>,
    profile: Option<BackendProfile>,
    mismatch: Mismatch,
}

impl Judging<'_> {
    fn new(input: JudgingInput<'_>) -> Result<Judging<'_>, Failure> {
        let JudgingInput {
            common,
            environment,
            folders,
            backend,
            asks,
            threshold,
            view,
            keeping,
            streams,
            sources: _,
            profile,
            mismatch,
        } = input;
        Ok(Judging {
            environment,
            engine: engine(common, environment, folders, backend, profile, common.jobs)?,
            asks,
            threshold,
            view,
            keeping,
            streams,
            mismatch,
        })
    }

    /// Ask one record and build the line its answer prints.
    ///
    /// Nothing here touches the writer, so a worker thread may call it and the
    /// one thread that owns standard output prints the lines in input order.
    fn row(&self, reading: &Reading, bytes: &[u8]) -> Result<Judged, Failure> {
        let record = reading
            .record(bytes)
            .map_err(|error| Failure::record(error, reading.streams()))?;
        self.finish_row(reading, record, Some(bytes))
    }

    fn typed_row(&self, reading: &Reading, record: &Record) -> Result<Judged, Failure> {
        self.finish_row(reading, record.clone(), None)
    }

    fn finish_row(
        &self,
        reading: &Reading,
        record: Record,
        arrived: Option<&[u8]>,
    ) -> Result<Judged, Failure> {
        let sending = asked_of(reading, record, &self.asks)?;
        let judged = self.engine.judge(
            &sending.question,
            self.threshold,
            sending.evidence,
            self.environment.cancel(),
        )?;
        self.row_of(reading, sending.record, sending.question, &judged, arrived)
    }

    /// Build the line one answered record prints. Both paths share it.
    fn row_of(
        &self,
        reading: &Reading,
        record: Record,
        question: Question,
        judged: &Judgment,
        arrived: Option<&[u8]>,
    ) -> Result<Judged, Failure> {
        let (outcome, replayed) = (judged.outcome, judged.answered.replayed);
        let probability = judged.answer.yes();
        let printed =
            if self.view.details && (self.keeping != Keeping::Passing || outcome == Outcome::Yes) {
                // `rank` orders and never selects, so a ranked row carries no
                // value. A value here would be a cut at 0.5 that nobody named.
                let shown = if self.keeping == Keeping::Ordered {
                    Value::YesNo(None)
                } else {
                    judged.value.clone()
                };
                let run = Run {
                    backend: self.engine.backend(),
                    tuned_for: self.tuned_for_profile(),
                    warning: self.mismatch.warning(),
                };
                let input = self.streams.then_some(record);
                Some(decision(
                    run,
                    judged,
                    question,
                    self.threshold,
                    shown,
                    input,
                )?)
            } else if self.keeping == Keeping::Passing && outcome != Outcome::Yes {
                None
            } else if self.keeping.streams_only() {
                Some(match arrived {
                    Some(bytes) => reading.as_it_arrived(bytes)?.to_owned(),
                    None => json_line(&record)?,
                })
            } else if self.view.raw {
                // One line stands for one record, so an unresolved record prints
                // an empty line. On one document it prints nothing at all.
                match judged.value.label() {
                    Some(label) => Some(label.to_owned()),
                    None if self.streams => Some(String::new()),
                    None => None,
                }
            } else if self.view.quiet {
                None
            } else if self.streams {
                Some(json_line(&RecordValue::new(record, judged.value.clone()))?)
            } else {
                Some(json_line(&judged.value)?)
            };
        Ok(Judged {
            printed,
            outcome,
            replayed,
            probability,
            partial_failure: false,
            profile_mismatch: self.mismatch.notice(),
        })
    }

    fn tuned_for_profile(&self) -> Option<&crate::core::ProfileName> {
        self.mismatch.tuned_for()
    }
}

/// Turn the outcome into the exit code `specification/channels.md` fixes.
fn exit_code(outcome: Outcome) -> ExitCode {
    match outcome {
        Outcome::Yes => ExitCode::from(0),
        Outcome::No => ExitCode::from(1),
        Outcome::Unresolved => ExitCode::from(3),
    }
}
