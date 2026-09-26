//! Apply one saved question set to documents or record streams.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;

use crate::core::adapters::built_in;
use crate::core::{
    Backend, Framing, ModelName, PartError, Plan, Pointer, QuestionSet, Reading, Record,
};

use crate::args::{AnnotateArguments, Common};
use crate::asking::{self, Folders};
use crate::edge::{self, Environment};
use crate::engine::facade::Engine;
use crate::failure::Failure;
use crate::profile::{self, Mismatch};
use crate::schedule::{Judged, Output};
use crate::table::{Kind as TableKind, Rows as TableRows};

mod aggregation;
mod plan;

pub(crate) use crate::engine::facade::{GroupAnswer, PreparedGroup, check_model};
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
    // `@FILE` names the same file, as the other verbs' question files do.
    let path = arguments
        .questions
        .to_str()
        .and_then(|typed| typed.strip_prefix('@'));
    let text = fs::read_to_string(path.map_or(arguments.questions.as_path(), Path::new))
        .map_err(Failure::OpenQuestionSet)?;
    let set = match QuestionSet::parse(&text) {
        Ok(set) => set,
        Err(_) if input_looks_like_set(arguments) => {
            return Err(Failure::Usage(
                "the question set and input appear to be swapped; put the question set after `annotate` and the evidence after `--input`",
            ));
        }
        Err(error) => return Err(error.into()),
    };
    crate::schedule::width(arguments.common.jobs)?;
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
        let judging = Judging::new(
            arguments,
            environment,
            asking::engine(
                &arguments.common,
                environment,
                folders,
                backend,
                profile,
                arguments.common.jobs,
            )?,
            set,
            mismatch,
        );
        return crate::annotate_schedule::run(
            &judging,
            &reading,
            rows.map(|row| row.map(crate::annotate_schedule::Input::Record)),
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
    let judging = Judging::new(
        arguments,
        environment,
        asking::engine(
            &arguments.common,
            environment,
            folders,
            backend,
            profile,
            arguments.common.jobs,
        )?,
        set,
        mismatch,
    );
    crate::annotate_schedule::run(
        &judging,
        &reading,
        chunks.map(|row| row.map(crate::annotate_schedule::Input::Bytes)),
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
    Ok(Reading::new(common.framing(), fields)?)
}

pub(crate) struct Judging<'a> {
    common: &'a Common,
    environment: &'a Environment,
    engine: Engine,
    set: QuestionSet,
    mismatch: Mismatch,
    streams: bool,
}

impl<'a> Judging<'a> {
    fn new(
        arguments: &'a AnnotateArguments,
        environment: &'a Environment,
        engine: Engine,
        set: QuestionSet,
        mismatch: Mismatch,
    ) -> Self {
        let common = &arguments.common;
        Self {
            streams: common.framing() != Framing::Document,
            engine,
            common,
            environment,
            set,
            mismatch,
        }
    }

    pub(crate) const fn engine(&self) -> &Engine {
        &self.engine
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
        self.engine.backend().model()
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
        let plan = plan_for(&self.set, &places, self.engine.backend(), base, record)?;
        Ok(self.engine.prepare_group(&plan, places)?)
    }

    pub(crate) fn answer_group(&self, group: PreparedGroup) -> Result<GroupAnswer, Failure> {
        Ok(self.engine.answer_group(group, self.environment.cancel())?)
    }
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
    if group.is_empty() {
        return Err(Failure::Defect("an annotate group is empty"));
    }
    let evidence = set
        .group_evidence(group, &base.evidence(record)?)
        .map_err(|error| match error {
            PartError::Reading(error) => Failure::from(error),
            PartError::Render(error) => Failure::from(error),
            PartError::Record(error) => Failure::from(error),
        })?;
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
