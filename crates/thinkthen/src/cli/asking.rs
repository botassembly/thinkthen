//! What a judging run asks, and the one row each record's answers make.
//! `judged.rs` sends every record through the question pipeline.
//!
//! `judge.rs` decides what a run keeps and what view it prints in.

use std::io::Read;
use std::io::{self, Write as _};
use std::num::NonZeroUsize;
use std::process::ExitCode;
use std::time::Duration;

use crate::core::{
    Backend, BackendProfile, Framing, Outcome, Pointer, Question, QuestionText, Reading, Record,
    Resolved, Setting, Sources, Threshold,
};

use crate::args::Common;
use crate::edge::Environment;
use crate::engine::Width;
use crate::engine::facade::{Engine, Settings, Storage};
use crate::failure::Failure;
use crate::judge::{Asked, Keeping, View};
use crate::profile::{self, Mismatch};
use crate::public::AttemptObservation;
use crate::schedule::{self, Output};

mod context;
mod folders;
mod judged;
mod plan;
mod rank_set;
mod reading;
mod row;

use context::Context;
pub(crate) use folders::Folders;
use reading::read_by;

/// What one row shows beside its answer: the line it arrived as, the keys
/// of its questions, and its requests' attempts.
struct RowContext<'a> {
    arrived: Option<&'a [u8]>,
    requests: Vec<String>,
    attempts: Vec<AttemptObservation>,
    position: Option<&'a crate::cli::intake::Position>,
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
            per_minute: environment.per_minute,
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
    )?
    .with_process_budget(
        common.max_requests_total,
        common
            .max_estimated_input_tokens_total
            .or(environment.estimated_total),
    ))
}

/// Where one record's question comes from.
///
/// Every verb but `choose --options` asks the same question of every record.
/// `--options` names a pointer, and each record holds its own candidate list
/// there, so the question is built again for each one.
#[derive(Clone)]
pub(crate) enum Asks {
    /// One question, asked of every record.
    Fixed(Question),
    /// Ordered decide questions, each asking about the same evidence.
    Set(crate::core::QuestionSet),
    /// A pick whose options each record carries at this pointer.
    FromRecord {
        /// The question the model receives.
        text: QuestionText,
        /// Where in the record the candidate list sits.
        pointer: Pointer,
    },
}

impl std::fmt::Debug for Asks {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set(set) => formatter
                .debug_struct("Set")
                .field("questions", &set.questions().len())
                .finish_non_exhaustive(),
            Self::Fixed(question) => formatter.debug_tuple("Fixed").field(question).finish(),
            Self::FromRecord { text, pointer } => formatter
                .debug_struct("FromRecord")
                .field("text", text)
                .field("pointer", pointer)
                .finish(),
        }
    }
}

impl Asks {
    /// The verb a replay failure over one document names.
    const fn verb(&self) -> &'static str {
        match self {
            Self::Set(_) | Self::Fixed(Question::Decide { .. }) => "decide",
            Self::Fixed(Question::Tag { .. }) => "tag",
            Self::Fixed(Question::Score { .. }) => "score",
            Self::Fixed(Question::Choose { .. }) | Self::FromRecord { .. } => "choose",
        }
    }

    /// Build the question this record is asked.
    fn of(&self, record: &Record) -> Result<Question, Failure> {
        match self {
            Self::Fixed(question) => Ok(question.clone()),
            Self::Set(_) => Err(Failure::Defect("a set is not one question")),
            Self::FromRecord { text, pointer } => Ok(Question::Choose {
                text: text.clone(),
                options: record.choices(pointer)?,
            }),
        }
    }
    fn questions(&self, record: &Record) -> Result<Vec<Question>, Failure> {
        match self {
            Self::Set(set) => Ok(set
                .questions()
                .iter()
                .map(|member| member.question().clone())
                .collect()),
            _ => self.of(record).map(|question| vec![question]),
        }
    }
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
    let asked = (!settled.sources().model_is_default()).then(|| settled.model().as_str());
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
    let backend = environment
        .resolve(common.backend.as_deref(), common.url.as_deref(), asked)?
        .with_request_size(request_size.unwrap_or(if common.image.is_empty() {
            Backend::DEFAULT_REQUEST_SIZE
        } else {
            2_800_000
        }));
    if !common.image.is_empty() && !matches!(asks, Asks::Fixed(_)) {
        return Err(Failure::Usage("--image requires one fixed scalar question"));
    }
    let image = crate::cli::image::read(common)?;
    // The configuration's model applies only on the unnamed path.
    let configured_model = (asked.is_none() && environment.named().is_none())
        .then(|| environment.model())
        .flatten();
    let folders = Folders::of(common, environment)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    if request_size.is_some() {
        environment.warn_request_size(&backend)?;
    }
    let profile = profile::read(common, environment, &backend)?;
    crate::cli::intake::window(common, !settled.on().is_empty())?;
    let reading = read_by(common, settled, keeping)?;
    output.validate_display(common)?;
    let documents = !reading.streams() && common.input.len() > 1;
    if documents && (view.raw || view.quiet) {
        return Err(Failure::Usage(
            "multiple documents cannot accompany --raw or --quiet",
        ));
    }
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
            let running =
                if matches!(asks, Asks::Set(_)) || settled.text().as_json().as_str().is_some() {
                    running
                } else {
                    Setting::Records(NonZeroUsize::MIN)
                };
            Mismatch::new(settled.profile(), profile.as_ref()).with_batch(tuned_for, running)
        }
        None => Mismatch::new(settled.profile(), profile.as_ref()),
    };
    let (source, snapshot) = crate::cli::intake::Intake::prepare(
        common,
        &reading,
        input,
        !settled.on().is_empty(),
        output.neighbors(),
    )?;
    output.snapshot(snapshot);
    let configuration = JudgingInput {
        common,
        environment,
        folders,
        backend,
        image,
        asks,
        threshold,
        view,
        keeping,
        streams: reading.streams(),
        documents,
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

    judged::run(configuration, &reading, source, batch, output)
}

/// One question over one engine, asked of every record in turn.
struct Judging<'a> {
    environment: &'a Environment,
    engine: Engine,
    asks: Asks,
    image: Option<Vec<crate::core::image::ImageInput>>,
    threshold: Option<Threshold>,
    view: View,
    keeping: Keeping,
    streams: bool,
    documents: bool,
    mismatch: Mismatch,
    context: Option<Context>,
}

struct JudgingInput<'a> {
    common: &'a Common,
    environment: &'a Environment,
    folders: Folders,
    backend: Backend,
    asks: Asks,
    image: Option<Vec<crate::core::image::ImageInput>>,
    threshold: Option<Threshold>,
    view: View,
    keeping: Keeping,
    streams: bool,
    documents: bool,
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
            image,
            threshold,
            view,
            keeping,
            streams,
            documents,
            sources: _,
            profile,
            mismatch,
            context,
        } = input;
        Ok(Judging {
            environment,
            engine: engine(common, environment, folders, backend, profile, common.jobs)?,
            asks,
            image,
            threshold,
            view,
            keeping,
            streams,
            documents,
            mismatch,
            context,
        })
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
