//! One record to one request, and one reply to one row.
//!
//! `judge.rs` decides what a run keeps and what view it prints in.

use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use crate::core::{
    Backend, BackendProfile, DecisionResult, Framing, Meta, Outcome, Plan, PlanDocument, Pointer,
    Question, QuestionText, Reading, Record, RecordValue, RequestMeta, Resolved, Sources,
    Threshold, Value, json_line, question_sha256_with_profile,
};

use crate::args::Common;
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::Client;
use crate::judge::{Asked, Keeping, View};
use crate::prepared_request::PreparedRequest;
use crate::profile::{self, Mismatch};
use crate::recorder::Recorder;
use crate::schedule::{self, Judged, Output};
use crate::table::{Kind as TableKind, Rows as TableRows};

mod folders;
mod request;

pub(crate) use folders::Folders;
pub(crate) use request::{ask, ask_prepared};

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
    input: impl Read + Send + 'static,
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
    let reading = read_by(common, settled)?;
    if keeping.streams_only() && !reading.streams() {
        return Err(Failure::NoFraming(keeping.verb()));
    }
    if view.quiet && reading.streams() {
        return Err(Failure::QuietOverRecords);
    }
    let jobs = schedule::jobs_of(common.jobs, reading.streams())?;
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

    if let Some(kind) = table_kind(common) {
        return over_table(configuration, &reading, source, kind, jobs, output);
    }

    let mut chunks = edge::Chunks::new(source, reading.streams());

    if common.dry_run {
        return plan(
            &configuration.backend,
            configuration.profile.as_ref(),
            &configuration.mismatch,
            &reading,
            &Planning {
                asks: &configuration.asks,
                sources: configuration.sources,
            },
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
        &|bytes: &Vec<u8>| judging.row(&reading, bytes),
        chunks,
        jobs,
        judging.recorder.reported(),
        judging.environment.cancel(),
        output,
    )
}

fn over_table(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    source: Box<dyn std::io::BufRead + Send>,
    kind: TableKind,
    jobs: usize,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let mut rows = TableRows::new(source, kind)?;
    if configuration.common.dry_run {
        return plan_record(
            &configuration.backend,
            configuration.profile.as_ref(),
            &configuration.mismatch,
            reading,
            &Planning {
                asks: &configuration.asks,
                sources: configuration.sources,
            },
            rows.next().transpose()?,
            output.writer(),
        );
    }
    let judging = Judging::new(configuration)?;
    schedule::over_records(
        &|record| judging.typed_row(reading, record),
        rows,
        jobs,
        judging.recorder.reported(),
        judging.environment.cancel(),
        output,
    )
}

/// Read the framing the command line asked for, over the settled pointers.
fn read_by(common: &Common, settled: &Resolved) -> Result<Reading, Failure> {
    let framing = if common.lines {
        Framing::Lines
    } else if common.jsonl {
        Framing::Jsonl
    } else if common.csv {
        Framing::Csv
    } else if common.tsv {
        Framing::Tsv
    } else {
        Framing::Document
    };
    Ok(Reading::new(framing, settled.on().to_vec())?)
}

fn table_kind(common: &Common) -> Option<TableKind> {
    common
        .csv
        .then_some(TableKind::Csv)
        .or_else(|| common.tsv.then_some(TableKind::Tsv))
}

/// What a plan shows beyond the request: the question and where it came from.
struct Planning<'a> {
    asks: &'a Asks,
    sources: Option<Sources>,
}

/// Print the plan for the first record, and read no further than that record.
#[expect(
    clippy::too_many_arguments,
    reason = "byte and table inputs share one plan path with explicit profile state"
)]
fn plan(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    first: Option<Vec<u8>>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Some(bytes) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let record = reading
        .record(&bytes)
        .map_err(|error| Failure::record(error, reading.streams()))?;
    plan_record(
        backend,
        profile,
        mismatch,
        reading,
        planning,
        Some(record),
        writer,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "the plan path receives each resolved concern without a second configuration type"
)]
fn plan_record(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    mismatch: &Mismatch,
    reading: &Reading,
    planning: &Planning<'_>,
    first: Option<Record>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Some(record) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let sending = asked_of(reading, record, backend, planning.asks)?;
    let _prepared = PreparedRequest::with_profile(backend, &sending.plan, profile)?;
    mismatch.print_once()?;
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
    record: Record,
    backend: &Backend,
    asks: &Asks,
) -> Result<Sending, Failure> {
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
    profile: Option<BackendProfile>,
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
            common,
            environment,
            client: Client::new(Duration::from_secs(common.timeout), backend.is_secure()),
            recorder: Recorder::of_private(
                folders.record.as_deref(),
                folders.replay.as_deref(),
                folders.private_default,
                folders.cache_answers,
            )?,
            backend,
            asks,
            threshold,
            view,
            keeping,
            streams,
            profile,
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
        let sending = asked_of(reading, record, &self.backend, &self.asks)?;
        let answered = ask(
            &self.backend,
            &sending.plan,
            self.common,
            self.environment,
            &self.recorder,
            &self.client,
            self.profile.as_ref(),
        )?;
        let [crate::core::AnswerOutcome::Answered(answer)] = answered.reply.outcomes() else {
            return Err(Failure::Defect("the adapter answered no question"));
        };
        let answer = answer.clone();
        let (value, outcome) = answer.read(self.threshold);
        let probability = answer.yes();
        let printed = if self.view.details
            && (self.keeping != Keeping::Passing || outcome == Outcome::Yes)
        {
            let meta = Meta::new(
                env!("CARGO_PKG_VERSION"),
                question_sha256_with_profile(
                    &sending.question,
                    self.threshold,
                    self.calibrated_profile(),
                )?,
                self.backend.url().clone(),
                answered.reply.model().clone(),
                answered.reply.usage(),
                RequestMeta::new(
                    answered.replayed,
                    answered.requests_sent,
                    vec![answered.request.as_str().to_owned()],
                )
                .with_profile_warning(self.mismatch.warning()),
            );
            // `rank` orders and never selects, so a ranked row carries no
            // value. A value here would be a cut at 0.5 that nobody named.
            let shown = if self.keeping == Keeping::Ordered {
                Value::YesNo(None)
            } else {
                value
            };
            let row = DecisionResult::new(shown, sending.question, answer, self.threshold, meta);
            let row = if self.streams {
                row.with_input(sending.record)
            } else {
                row
            };
            Some(json_line(&row)?)
        } else if self.keeping == Keeping::Passing && outcome != Outcome::Yes {
            None
        } else if self.keeping.streams_only() {
            Some(match arrived {
                Some(bytes) => reading.as_it_arrived(bytes)?.to_owned(),
                None => json_line(&sending.record)?,
            })
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
        } else if self.streams {
            Some(json_line(&RecordValue::new(sending.record, value))?)
        } else {
            Some(json_line(&value)?)
        };
        Ok(Judged {
            printed,
            outcome,
            replayed: answered.replayed,
            probability,
            partial_failure: false,
            profile_mismatch: self.mismatch.notice(),
        })
    }

    fn calibrated_profile(&self) -> Option<&crate::core::ProfileName> {
        self.mismatch.calibrated()
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
