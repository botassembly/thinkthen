//! What a judging run asks, and the one row each record's answers make.
//! `judged.rs` sends every record through the question pipeline.
//!
//! `judge.rs` decides what a run keeps and what view it prints in.

use std::io::Read;
use std::num::NonZeroUsize;
use std::process::ExitCode;

use crate::{BatchSetting as Setting, InputDeclaration, QuestionKind, RequestFraming as Framing, ResolvedSettings};
use crate::cli::asked::Resolved;
use crate::cli::reading::Reading;
use crate::schedule::Outcome;
use crate::plan_document::Sources;

use crate::args::Common;
use crate::edge::Environment;
use crate::failure::Failure;
use crate::judge::{Asked, Keeping, View};
use crate::profile::{self, Mismatch};
use crate::schedule::{self, Output};

pub(crate) mod context;
mod folders;
mod judged;
mod native;
pub(crate) mod native_reader;
mod plan;
mod reading;

use context::Context;
pub(crate) use folders::Folders;
use reading::read_by;

/// Where one record's question comes from.
///
/// Every verb but `choose --options` asks the same question of every record.
/// `--options` names a pointer, and each record holds its own candidate list
/// there, so the question is built again for each one.
#[derive(Clone, Debug)]
pub(crate) enum Asks { Fixed(QuestionKind), Set, FromRecord }
impl Asks {
    const fn verb(&self) -> &'static str {
        match self {
            Self::Fixed(QuestionKind::Tag) => "tag", Self::Fixed(QuestionKind::Score) => "score",
            Self::Fixed(QuestionKind::Choose) | Self::FromRecord => "choose", _ => "decide",
        }
    }
}
pub(crate) fn fixed(settled: &Resolved) -> Result<Asks, Failure> {
    if settled.dynamic { return Err(Failure::Defect("a verb with no list left its question open")); }
    Ok(Asks::Fixed(settled.kind))
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
        admitted,
    } = asked;
    common.check_plan_name()?;
    if common.media.as_deref() == Some("image") && !settled.on.is_empty() {
        return Err(Failure::Usage("images cannot accompany saved on pointers"));
    }
    let view = view.checked()?;
    let asked = settled.model.as_deref();
    let per_document = matches!(
        asks,
        Asks::Fixed(QuestionKind::Choose | QuestionKind::Tag | QuestionKind::Score) | Asks::FromRecord
    );
    let request_size = batch
        .as_ref()
        .filter(|tiers| {
            if common.images() {
                environment.request_size_selected(tiers.request_size)
            } else {
                !per_document || common.framing() != Framing::Document
            }
        })
        .map(|tiers| environment.request_size(tiers.request_size))
        .transpose()?;
    let backend = environment.resolve(common.backend.as_deref(), common.url.as_deref(), asked)?;
    if let Some(size) = request_size { environment.set_request_size(size)?; }
    // The configuration's model applies only on the unnamed path.
    let configured_model = asked.is_none() && backend.backend_name.is_none()
        && backend.model_source == "configuration";
    let folders = Folders::of(common, environment)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    if let Some(size) = request_size { environment.warn_request_size(&backend, size)?; }
    profile::read(common, environment)?;
    crate::cli::intake::window(common, !settled.on.is_empty())?;
    let reading = read_by(common, settled, keeping)?;
    output.validate_display(common)?;
    let documents = !reading.streams() && (common.input.len() > 1 || common.located());
    if documents && (view.raw || view.quiet) {
        return Err(Failure::Usage(
            "multiple documents cannot accompany --raw or --quiet",
        ));
    }
    if view.quiet && reading.streams() {
        return Err(Failure::QuietOverRecords);
    }
    schedule::jobs_of(common.jobs, reading.streams(), environment)?;
    let context = context::for_run(
        batch.as_ref().and_then(|tiers| tiers.context),
        settled.literal_question,
    )?;
    let context_field = batch
        .as_ref()
        .and_then(|tiers| tiers.context_field)
        .map(str::to_owned);
    let tuned_for = batch
        .as_ref()
        .filter(|tiers| tiers.tuned)
        .map(|tiers| tiers.file.unwrap_or(Setting::Records(NonZeroUsize::MIN)));
    let batch = batch.map_or(Ok(None), |tiers| {
        tiers.setting(environment, reading.streams())
    })?;
    let mismatch = match batch {
        Some(running) => {
            let running =
                if matches!(asks, Asks::Set) || settled.literal_question {
                    running
                } else {
                    Setting::Records(NonZeroUsize::MIN)
                };
            Mismatch::new(settled.profile.as_deref(), None).with_batch(tuned_for, running)
        }
        None => Mismatch::new(settled.profile.as_deref(), None),
    };
    let (source, snapshot) = crate::cli::intake::Intake::prepare(
        common,
        &reading,
        input,
        !settled.on.is_empty(),
        output.neighbors(),
    )?;
    output.snapshot(snapshot);
    let configuration = JudgingInput {
        admitted,
        context_schema: settled.context_schema.clone(),
        common,
        environment,
        folders,
        backend,
        asks,
        view,
        keeping,
        streams: reading.streams(),
        documents,
        text_view: output.text_view(),
        profile: settled.profile.clone(),
        mismatch,
        context,
        context_field,
        sources: (settled.saved || configured_model).then(|| {
            let sources = settled.sources.clone();
            if configured_model { sources.setting("model", "configuration") } else { sources }
        }),
    };

    judged::run(configuration, &reading, source, batch, output)
}

struct JudgingInput<'a> {
    admitted: Option<crate::AdmittedRequest>,
    context_schema: Option<InputDeclaration>,
    common: &'a Common,
    environment: &'a Environment,
    folders: Folders,
    backend: ResolvedSettings,
    asks: Asks,
    view: View,
    keeping: Keeping,
    streams: bool,
    documents: bool,
    text_view: bool,
    sources: Option<Sources>,
    profile: Option<String>,
    mismatch: Mismatch,
    context: Option<Context>,
    context_field: Option<String>,
}

/// Turn the outcome into the exit code `specification/channels.md` fixes.
fn exit_code(outcome: Outcome) -> ExitCode {
    match outcome {
        Outcome::Yes => ExitCode::from(0),
        Outcome::No => ExitCode::from(1),
        Outcome::Unresolved => ExitCode::from(3),
    }
}
