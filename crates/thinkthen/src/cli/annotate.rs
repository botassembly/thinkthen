//! Apply one saved question set to documents or record streams.

use std::fs;
use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use crate::core::adapters::built_in;
use crate::core::{
    Backend, BackendProfile, Framing, ModelName, Plan, Pointer, QuestionSet, Reading, Record,
};

use crate::args::{AnnotateArguments, Common};
use crate::asking::{Folders, ask_prepared};
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::Client;
use crate::prepared_request::{PreparedRequest, PreparedRequests};
use crate::profile::{self, Mismatch};
use crate::recorder::Recorder;
use crate::schedule::{Judged, Output};
use crate::table::{Kind as TableKind, Rows as TableRows};

mod aggregation;
mod plan;

pub(crate) use aggregation::{GroupAnswer, check_model};
use plan::{dry_run, dry_run_record};

#[expect(
    clippy::too_many_lines,
    reason = "the command edge keeps validation and mode selection in their observable order"
)]
pub(crate) fn run(
    arguments: &AnnotateArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    refuse_views(arguments)?;
    let text = fs::read_to_string(&arguments.questions).map_err(Failure::OpenQuestionSet)?;
    let set = match QuestionSet::parse(&text) {
        Ok(set) => set,
        Err(_) if input_looks_like_set(arguments) => {
            return Err(Failure::Usage(
                "the question set and input appear to be swapped; put the question set after `annotate` and the evidence after `--input`",
            ));
        }
        Err(error) => return Err(error.into()),
    };
    let folders = Folders::of(&arguments.common, environment)?;
    if arguments.common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let model = arguments
        .common
        .model
        .as_deref()
        .or_else(|| environment.model())
        .unwrap_or(built_in::DEFAULT_MODEL);
    let backend = Backend::resolve(
        arguments.common.url.as_deref(),
        environment.base_url(),
        model,
    )?;
    let profile = profile::read(&arguments.common)?;
    let mismatch = Mismatch::new(set.profile(), profile.as_ref());
    let reading = reading(&arguments.common)?;
    let source = edge::source(arguments.common.input.as_deref(), input)?;
    if let Some(kind) = table_kind(&arguments.common) {
        let mut rows = TableRows::new(source, kind)?;
        if arguments.common.dry_run {
            return dry_run_record(
                &set,
                &backend,
                &reading,
                profile.as_ref(),
                &mismatch,
                rows.next().transpose()?,
                &mut writer,
            );
        }
        let judging = Judging {
            common: &arguments.common,
            environment,
            recorder: Recorder::of_private(
                folders.record.as_deref(),
                folders.replay.as_deref(),
                folders.private_default,
                folders.cache_answers,
            )?,
            client: Client::new(
                Duration::from_secs(arguments.common.timeout),
                backend.is_secure(),
            ),
            backend,
            set,
            profile,
            mismatch,
            streams: reading.streams(),
            recording_reported: folders.reported(),
        };
        let jobs = arguments.common.jobs.map_or(4, usize::from);
        return crate::annotate_schedule::run(
            &judging,
            &reading,
            rows.map(|row| row.map(crate::annotate_schedule::Input::Record)),
            jobs,
            environment.cancel(),
            &mut Output::Streaming(&mut writer),
        );
    }
    let mut chunks = edge::Chunks::new(source, reading.streams());
    if arguments.common.dry_run {
        return dry_run(
            &set,
            &backend,
            &reading,
            profile.as_ref(),
            &mismatch,
            chunks.next().transpose()?,
            &mut writer,
        );
    }
    let judging = Judging {
        common: &arguments.common,
        environment,
        recorder: Recorder::of_private(
            folders.record.as_deref(),
            folders.replay.as_deref(),
            folders.private_default,
            folders.cache_answers,
        )?,
        client: Client::new(
            Duration::from_secs(arguments.common.timeout),
            backend.is_secure(),
        ),
        backend,
        set,
        profile,
        mismatch,
        streams: reading.streams(),
        recording_reported: folders.reported(),
    };
    let jobs = arguments.common.jobs.map_or(4, usize::from);
    crate::annotate_schedule::run(
        &judging,
        &reading,
        chunks.map(|row| row.map(crate::annotate_schedule::Input::Bytes)),
        jobs,
        environment.cancel(),
        &mut Output::Streaming(&mut writer),
    )
}

fn refuse_views(arguments: &AnnotateArguments) -> Result<(), Failure> {
    if arguments.extra_input.is_some() {
        return Err(Failure::Usage(
            "the second path is input; write it as `--input FILE`",
        ));
    }
    if arguments.threshold.is_some() {
        return Err(Failure::Usage(
            "--threshold belongs to each question in the question set; `annotate` takes no command-level threshold",
        ));
    }
    if arguments.quiet {
        return Err(Failure::Usage(
            "--quiet hides one answer carried by an exit code; `annotate` prints every named answer",
        ));
    }
    if arguments.raw {
        return Err(Failure::Usage(
            "--raw prints one bare label; `annotate` always prints one JSON object",
        ));
    }
    Ok(())
}

fn input_looks_like_set(arguments: &AnnotateArguments) -> bool {
    arguments
        .common
        .input
        .as_deref()
        .and_then(|path| fs::read_to_string(path).ok())
        .is_some_and(|text| QuestionSet::parse(&text).is_ok())
}

fn framing(common: &Common) -> Framing {
    if common.lines {
        Framing::Lines
    } else if common.jsonl {
        Framing::Jsonl
    } else if common.csv {
        Framing::Csv
    } else if common.tsv {
        Framing::Tsv
    } else {
        Framing::Document
    }
}

fn table_kind(common: &Common) -> Option<TableKind> {
    common
        .csv
        .then_some(TableKind::Csv)
        .or_else(|| common.tsv.then_some(TableKind::Tsv))
}

fn reading(common: &Common) -> Result<Reading, Failure> {
    let fields = common
        .field
        .iter()
        .map(|typed| {
            Pointer::new(typed).map_err(|error| Failure::Pointer("--field", typed.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Reading::new(framing(common), fields)?)
}

pub(crate) struct Judging<'a> {
    common: &'a Common,
    environment: &'a Environment,
    recorder: Recorder,
    client: Client,
    backend: Backend,
    set: QuestionSet,
    profile: Option<BackendProfile>,
    mismatch: Mismatch,
    streams: bool,
    recording_reported: bool,
}

impl Judging<'_> {
    pub(crate) const fn recording_named(&self) -> bool {
        self.recording_reported
    }

    pub(crate) fn record(
        &self,
        base: &Reading,
        input: crate::annotate_schedule::Input,
    ) -> Result<Record, Failure> {
        let record = match input {
            crate::annotate_schedule::Input::Bytes(bytes) => base
                .annotation_record(&bytes)
                .map_err(|error| Failure::record(error, base.streams()))?,
            crate::annotate_schedule::Input::Record(record) => record,
        };
        collisions(&self.set, &record)?;
        Ok(record)
    }

    pub(crate) fn groups(&self) -> Vec<Vec<usize>> {
        self.set.groups()
    }

    pub(crate) const fn requested_model(&self) -> &ModelName {
        self.backend.model()
    }

    pub(crate) fn finish(
        &self,
        record: Record,
        answered: Vec<GroupAnswer>,
    ) -> Result<Judged, Failure> {
        aggregation::finish(self, record, answered)
    }

    pub(crate) fn prepare_group(
        &self,
        base: &Reading,
        record: &Record,
        places: Vec<usize>,
    ) -> Result<PreparedGroup, Failure> {
        let plan = plan_for(&self.set, &places, &self.backend, base, record)?;
        let prepared = PreparedRequests::with_profile(&self.backend, &plan, self.profile.as_ref())?;
        let mut places = places.into_iter();
        let chunks = prepared
            .into_chunks()
            .into_iter()
            .map(|chunk| PreparedGroupChunk {
                places: places.by_ref().take(chunk.plan.questions().len()).collect(),
                plan: chunk.plan,
                prepared: chunk.request,
            })
            .collect();
        Ok(PreparedGroup { chunks })
    }

    pub(crate) fn answer_group(&self, group: PreparedGroup) -> Result<GroupAnswer, Failure> {
        let mut answered = Vec::with_capacity(group.chunks.len());
        let mut model = None;
        for chunk in group.chunks {
            let result = ask_prepared(
                &self.backend,
                &chunk.plan,
                chunk.prepared,
                self.common,
                self.environment,
                &self.recorder,
                &self.client,
            )?;
            check_model(&mut model, result.reply.model(), self.backend.model())?;
            answered.push(aggregation::ChunkAnswer {
                places: chunk.places,
                reply: result.reply,
                digest: result.request.as_str().to_owned(),
                requests_sent: result.requests_sent,
                replayed: result.replayed,
            });
        }
        Ok(GroupAnswer { answered, model })
    }
}

pub(crate) struct PreparedGroup {
    chunks: Vec<PreparedGroupChunk>,
}

struct PreparedGroupChunk {
    places: Vec<usize>,
    plan: Plan,
    prepared: PreparedRequest,
}

fn collisions(set: &QuestionSet, record: &Record) -> Result<(), Failure> {
    for question in set.questions() {
        if record.has_member(question.name()) {
            return Err(Failure::AnnotationCollision(question.name().to_owned()));
        }
    }
    Ok(())
}

fn plan_for(
    set: &QuestionSet,
    group: &[usize],
    backend: &Backend,
    base: &Reading,
    record: &Record,
) -> Result<Plan, Failure> {
    let first_place = group
        .first()
        .ok_or(Failure::Defect("an annotate group is empty"))?;
    let first = set
        .questions()
        .get(*first_place)
        .ok_or(Failure::Defect("a group points outside its set"))?;
    let base_evidence = base.evidence(record)?;
    let evidence = if matches!(first.on(), [root] if root.as_str().is_empty()) {
        base_evidence
    } else {
        let nested = Reading::new(Framing::Document, first.on().to_vec())?;
        let record = nested.record(base_evidence.as_text()?.as_bytes())?;
        nested.evidence(&record)?
    };
    let questions = group
        .iter()
        .map(|place| {
            set.questions()
                .get(*place)
                .map(|named| named.question().clone())
                .ok_or(Failure::Defect("a group points outside its set"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Plan::new(evidence, backend.model().clone(), questions)
        .map_err(|_| Failure::Defect("an annotate group asks nothing"))
}
