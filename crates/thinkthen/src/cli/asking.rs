//! One record to one request, and one reply to one row. `batched.rs` sends
//! `decide`, `filter`, `rank` and `choose` over a stream in batches.
//!
//! `judge.rs` decides what a run keeps and what view it prints in.

use std::io::Read;
use std::io::{self, Write as _};
use std::num::NonZeroUsize;
use std::process::ExitCode;
use std::time::Duration;

use crate::core::{
    Backend, BackendProfile, BatchMeta, Evidence, Framing, Outcome, Pointer, Question,
    QuestionText, Reading, Record, Resolved, Setting, Sources, Threshold,
};

use crate::args::Common;
use crate::edge::{self, Environment};
use crate::engine::Width;
use crate::engine::facade::{Engine, Settings, Storage};
use crate::failure::Failure;
use crate::judge::{Asked, Keeping, View};
use crate::profile::{self, Mismatch};
use crate::schedule::{self, Judged, Output};
use crate::table::{Kind as TableKind, Rows as TableRows};

mod batch_meta;
mod batched;
mod context;
mod folders;
mod plan;
mod reading;
mod row;

use context::Context;
pub(crate) use folders::Folders;
use plan::{plan, plan_record, print_plan};
use reading::read_by;

struct RowContext<'a> {
    arrived: Option<&'a [u8]>,
    batch: Option<BatchMeta>,
}

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
    let roots = environment.roots()?;
    if !common.dry_run && folders.writable_by_another() {
        writeln!(
            io::stderr().lock(),
            "thinkthen: warning: another user may change this named cache or recording folder; its writers decide the answers read from it"
        )
        .map_err(Failure::Output)?;
    }
    if folders.record.is_some()
        && folders.replay.is_some()
        && (folders.refresh_cache
            || crate::core::adapters::built_in::is_mutable_alias(backend.model()))
    {
        writeln!(
            io::stderr().lock(),
            "thinkthen: warning: a mutable model alias or --refresh-cache sends each planned cache request live and may incur a charge"
        )
        .map_err(Failure::Output)?;
    }
    Ok(Engine::with_roots(
        Settings {
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
                refresh_cache: folders.refresh_cache,
            },
            key: environment.key_reader(),
            usage: environment.counters(),
        },
        roots,
    )?)
}

/// Where one record's question comes from.
///
/// Every verb but `choose --options` asks the same question of every record.
/// `--options` names a pointer, and each record holds its own candidate list
/// there, so the question is built again for each one.
#[derive(Clone, Debug)]
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
#[expect(
    clippy::too_many_lines,
    reason = "the command edge keeps mode, setting, and output decisions in their observable order"
)]
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
    common.check_plan_name()?;
    let threshold = settled.threshold();
    let view = view.checked()?;
    let configured_model = settled
        .sources()
        .model_is_default()
        .then(|| environment.model())
        .flatten();
    let per_document = matches!(
        asks,
        Asks::Fixed(Question::Choose { .. } | Question::Tag { .. } | Question::Score { .. })
            | Asks::FromRecord { .. }
    );
    let request_size = batch
        .as_ref()
        .filter(|_| !per_document || common.framing() != Framing::Document)
        .map(|tiers| environment.request_size(tiers.request_size))
        .transpose()?;
    let backend = Backend::resolve(
        common.url.as_deref(),
        environment.base_url(),
        configured_model.unwrap_or_else(|| settled.model().as_str()),
    )?
    .with_request_size(request_size.unwrap_or(Backend::DEFAULT_REQUEST_SIZE));
    environment.check_key(&backend)?;
    let folders = Folders::of(common, environment)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    if request_size.is_some() {
        environment.warn_request_size(&backend)?;
    }
    let profile = profile::read(common)?;
    let reading = read_by(common, settled, keeping)?;
    if view.quiet && reading.streams() {
        return Err(Failure::QuietOverRecords);
    }
    schedule::jobs_of(common.jobs, reading.streams())?;
    let context = context::for_run(
        batch.as_ref().and_then(|tiers| tiers.context),
        reading.streams(),
        settled.text().as_json().as_str().is_some(),
    )?;
    let tuned_for = batch
        .as_ref()
        .filter(|tiers| tiers.tuned)
        .and_then(|tiers| match tiers.file.as_ref() {
            Some(value) => Setting::of_json(value),
            None => Some(Setting::Records(NonZeroUsize::MIN)),
        });
    let batch = batch.map_or(Ok(None), |tiers| {
        tiers.setting(environment, reading.streams())
    })?;
    let mismatch = match batch {
        Some(running) => {
            let running = if settled.text().as_json().as_str().is_some() {
                running
            } else {
                Setting::Records(NonZeroUsize::MIN)
            };
            Mismatch::new(settled.profile(), profile.as_ref()).with_batch(tuned_for, running)
        }
        None => Mismatch::new(settled.profile(), profile.as_ref()),
    };
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
        context,
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

    let mut chunks = edge::numbered(edge::Chunks::new(source, reading.streams()), &reading);

    if common.dry_run {
        return plan(
            &configuration.backend,
            configuration.profile.as_ref(),
            &configuration.mismatch,
            &reading,
            &configuration.planning(),
            chunks.map(|(_, row)| row),
            output.writer(),
        );
    }

    let judging = Judging::new(configuration)?;
    if !judging.streams {
        let bytes = first(&mut chunks)?.unwrap_or_default();
        let judged = judging.row(&reading, &bytes)?;
        let outcome = judged.outcome;
        output.take(judged)?;
        return Ok(exit_code(outcome));
    }
    schedule::over_records(
        &judging.engine,
        &|bytes: &Vec<u8>| judging.row(&reading, bytes),
        chunks.map(place),
        judging.environment.cancel(),
        output,
    )
}

fn first(
    chunks: &mut impl Iterator<Item = (usize, Result<Vec<u8>, Failure>)>,
) -> Result<Option<Vec<u8>>, Failure> {
    chunks.next().map(|(_, row)| row).transpose()
}

fn place(
    (at, row): (usize, Result<Vec<u8>, Failure>),
) -> Result<(usize, Vec<u8>), schedule::Placed> {
    row.map(|bytes| (at, bytes))
        .map_err(|error| schedule::Placed::at(error, at))
}

fn over_table(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    source: Box<dyn std::io::BufRead + Send>,
    kind: TableKind,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let rows = TableRows::new(source, kind)?;
    if configuration.common.dry_run {
        return plan_record(
            &configuration.backend,
            configuration.profile.as_ref(),
            &configuration.mismatch,
            reading,
            &configuration.planning(),
            rows,
            output.writer(),
        );
    }
    let judging = Judging::new(configuration)?;
    schedule::over_records(
        &judging.engine,
        &|record| judging.typed_row(reading, record),
        rows.enumerate().map(|(place, row)| {
            row.map(|record| (place + 1, record))
                .map_err(|error| schedule::Placed::at(error, place + 1))
        }),
        judging.environment.cancel(),
        output,
    )
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
    context: Option<Context>,
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
    context: Option<Context>,
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
            context,
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
            context,
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
}

/// Turn the outcome into the exit code `specification/channels.md` fixes.
fn exit_code(outcome: Outcome) -> ExitCode {
    match outcome {
        Outcome::Yes => ExitCode::from(0),
        Outcome::No => ExitCode::from(1),
        Outcome::Unresolved => ExitCode::from(3),
    }
}
