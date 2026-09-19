//! One record to one request, and one reply to one row.
//!
//! This module knows how a record becomes a request and how the answer becomes
//! the line that prints. It knows nothing of which verb was typed. `judge.rs`
//! decides what a run keeps and what view it prints in, and hands both here.

use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use thinkthen_core::adapters::built_in;
use thinkthen_core::recording::Exchange as Recorded;
use thinkthen_core::{
    Backend, DecisionResult, Framing, Meta, Outcome, Plan, PlanDocument, Pointer, Question,
    QuestionText, Reading, Record, Reply, Resolved, Sources, Threshold, json_line, question_sha256,
};

use crate::args::Common;
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::{Client, Exchange};
use crate::judge::{Asked, Keeping, View};
use crate::recorder::Recorder;
use crate::schedule::{self, Judged, Output};

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

/// One record, the question it was asked, and the request that carries both.
#[derive(Debug)]
struct Sending {
    record: Record,
    question: Question,
    plan: Plan,
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
    input: impl Read,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let Asked {
        common,
        asks,
        settled,
        view,
        keeping,
    } = asked;
    let threshold = settled.threshold();
    let view = view.checked()?;
    let folders = Folders::of(common)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let backend = Backend::resolve(
        common.url.as_deref(),
        environment.base_url(),
        settled.model().as_str(),
    )?;
    let reading = read_by(common, settled)?;
    if keeping.streams_only() && !reading.streams() {
        return Err(Failure::NoFraming(keeping.verb()));
    }
    if view.quiet && reading.streams() {
        return Err(Failure::QuietOverRecords);
    }
    let mut chunks = edge::Chunks::new(
        edge::source(common.input.as_deref(), input)?,
        reading.streams(),
    );

    if common.dry_run {
        return plan(
            &backend,
            &reading,
            &Planning {
                asks: &asks,
                sources: settled.sources().from_file().then(|| *settled.sources()),
            },
            chunks.next().transpose()?,
            output.writer(),
        );
    }

    let jobs = schedule::jobs_of(common.jobs, reading.streams())?;
    let judging = Judging {
        common,
        environment,
        client: Client::new(Duration::from_secs(common.timeout), backend.is_secure()),
        recorder: Recorder::of(folders.record, folders.replay)?,
        backend,
        asks,
        threshold,
        view,
        keeping,
        streams: reading.streams(),
    };
    if !judging.streams {
        let bytes = chunks.next().transpose()?.unwrap_or_default();
        let judged = judging.row(&reading, &bytes)?;
        let outcome = judged.outcome;
        output.take(judged)?;
        return Ok(exit_code(outcome));
    }
    schedule::over_records(
        &|bytes| judging.row(&reading, bytes),
        &mut chunks,
        jobs,
        output,
    )
}

/// The folders `--record`, `--replay`, and `--cache` name between them.
#[derive(Debug)]
struct Folders<'a> {
    record: Option<&'a Path>,
    replay: Option<&'a Path>,
}

impl<'a> Folders<'a> {
    /// Read the two folders, with `--cache` standing for both at once.
    ///
    /// # Errors
    ///
    /// Returns [`Failure::CacheWithRecording`] when `--cache` is given beside
    /// one of the two options it stands for.
    fn of(common: &'a Common) -> Result<Self, Failure> {
        let Some(cached) = common.cache.as_deref() else {
            return Ok(Self {
                record: common.record.as_deref(),
                replay: common.replay.as_deref(),
            });
        };
        if common.record.is_some() || common.replay.is_some() {
            return Err(Failure::CacheWithRecording);
        }
        Ok(Self {
            record: Some(cached),
            replay: Some(cached),
        })
    }

    /// True when a folder is named at all, which a plan may not name.
    const fn named(&self) -> bool {
        self.record.is_some() || self.replay.is_some()
    }
}

/// Read the framing the command line asked for, over the settled pointers.
fn read_by(common: &Common, settled: &Resolved) -> Result<Reading, Failure> {
    let framing = match (common.lines, common.jsonl) {
        (true, _) => Framing::Lines,
        (_, true) => Framing::Jsonl,
        _ => Framing::Document,
    };
    Ok(Reading::new(framing, settled.on().to_vec())?)
}

/// What a plan shows beyond the request: the question and where it came from.
struct Planning<'a> {
    asks: &'a Asks,
    sources: Option<Sources>,
}

/// Print the plan for the first record, and read no further than that record.
fn plan(
    backend: &Backend,
    reading: &Reading,
    planning: &Planning<'_>,
    first: Option<Vec<u8>>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Some(bytes) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let sending = asked_of(reading, &bytes, backend, planning.asks)?;
    let document = PlanDocument::of(backend, &sending.plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
    let document = if reading.streams() {
        document.reading(reading)
    } else {
        document
    };
    let document = match planning.sources {
        Some(sources) => document.from(sources),
        None => document,
    };
    edge::write_line(writer, &json_line(&document)?)?;
    Ok(ExitCode::SUCCESS)
}

/// Read one record and build the one request it asks, which both paths do.
fn asked_of(
    reading: &Reading,
    bytes: &[u8],
    backend: &Backend,
    asks: &Asks,
) -> Result<Sending, Failure> {
    let record = reading.record(bytes)?;
    let question = asks.of(&record)?;
    let plan = Plan::new(
        reading.evidence(&record)?,
        backend.model().clone(),
        vec![question.clone()],
    )
    .map_err(|_| Failure::Defect("a plan of one question asks nothing"))?;
    Ok(Sending {
        record,
        question,
        plan,
    })
}

/// One question over one backend, asked of every record in turn.
#[derive(Debug)]
struct Judging<'a> {
    common: &'a Common,
    environment: &'a Environment,
    recorder: Recorder,
    backend: Backend,
    client: Client,
    asks: Asks,
    threshold: Option<Threshold>,
    view: View,
    keeping: Keeping,
    streams: bool,
}

impl Judging<'_> {
    /// Ask one record and build the line its answer prints.
    ///
    /// Nothing here touches the writer, so a worker thread may call it and the
    /// one thread that owns standard output prints the lines in input order.
    fn row(&self, reading: &Reading, bytes: &[u8]) -> Result<Judged, Failure> {
        let sending = asked_of(reading, bytes, &self.backend, &self.asks)?;
        let (reply, replayed) = ask(
            &self.backend,
            &sending.plan,
            self.common,
            self.environment,
            &self.recorder,
            &self.client,
        )?;
        let answer = reply
            .answers()
            .first()
            .ok_or(Failure::Defect("the adapter answered no question"))?
            .clone();
        let (value, outcome) = answer.read(self.threshold);
        let probability = answer.yes();
        let printed = if self.view.details {
            let meta = Meta::new(
                env!("CARGO_PKG_VERSION"),
                question_sha256(&sending.question, self.threshold)?,
                self.backend.url().clone(),
                reply.model().clone(),
                reply.usage(),
                replayed,
            );
            let row = DecisionResult::new(value, sending.question, answer, self.threshold, meta);
            let row = if self.streams {
                row.with_input(sending.record)
            } else {
                row
            };
            Some(json_line(&row)?)
        } else if self.keeping == Keeping::Passing && outcome != Outcome::Yes {
            // A record that did not reach the mark prints nothing at all.
            None
        } else if self.keeping.streams_only() {
            // The record goes back as it arrived, so nothing is parsed and
            // nothing is written through a JSON encoder.
            Some(reading.as_it_arrived(bytes)?.to_owned())
        } else if self.view.raw {
            // One line stands for one record, so an unresolved record prints
            // an empty line. On one document it prints nothing at all.
            match value.label() {
                Some(label) => Some(label.to_owned()),
                None if self.streams => Some(String::new()),
                None => None,
            }
        } else if self.view.quiet {
            None
        } else {
            Some(json_line(&value)?)
        };
        Ok(Judged {
            printed,
            outcome,
            replayed,
            probability,
        })
    }
}

/// Answer the plan from the recording folder, or from the backend itself.
///
/// The recording is read before a key is, so a replay opens no connection and
/// needs no key. Only an exchange the adapter read is recorded.
fn ask(
    backend: &Backend,
    plan: &Plan,
    common: &Common,
    environment: &Environment,
    recorder: &Recorder,
    client: &Client,
) -> Result<(Reply, bool), Failure> {
    let body = built_in::encode(plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
    let recorded = Recorded::new(backend.url(), &body);
    if let Some(response) = recorder.replayed(&recorded)? {
        return Ok((built_in::decode(plan, &response)?, true));
    }
    let key = edge::key()?;
    let answered = client.post(&Exchange {
        url: backend.url().as_str(),
        body: &body,
        key: &key,
        max_retries: common.max_retries,
        retry_wait: environment.retry_wait(),
    })?;
    let reply = built_in::decode(plan, &answered)?;
    recorder.record(&recorded, &answered)?;
    Ok((reply, false))
}

/// Turn the outcome into the exit code `specification/channels.md` fixes.
fn exit_code(outcome: Outcome) -> ExitCode {
    match outcome {
        Outcome::Yes => ExitCode::from(0),
        Outcome::No => ExitCode::from(1),
        Outcome::Unresolved => ExitCode::from(3),
    }
}
