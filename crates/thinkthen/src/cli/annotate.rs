//! Apply one saved question set to documents or record streams.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;

use crate::core::adapters::built_in;
use crate::core::{
    Backend, Framing, ModelName, PartError, Plan, Pointer, QuestionSet, QuestionSetError, Reading,
    ReadingError, Record, RecordError, Setting,
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
mod batching;
pub(crate) mod error_row;
mod plan;

pub(crate) use crate::engine::facade::{GroupAnswer, PreparedGroup, check_model};
use plan::{dry_run, dry_run_record};

pub(crate) enum PrepareError {
    MissingOn(String),
    Other(Failure),
}

impl PrepareError {
    pub(crate) fn into_failure(self) -> Failure {
        match self {
            Self::MissingOn(pointer) => Failure::Record(RecordError::Missed(pointer)),
            Self::Other(error) => error,
        }
    }
}

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
    environment.check_key(&backend)?;
    let folders = Folders::of(&arguments.common, environment)?;
    if arguments.common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let profile = profile::read(&arguments.common)?;
    let mismatch = Mismatch::new(set.profile(), profile.as_ref());
    let reading = reading(&arguments.common)?;
    let setting = batch_setting(arguments, environment, &set, reading.streams())?;
    let request_size = if reading.streams() {
        Some(environment.request_size(arguments.batching.max_request_bytes.as_deref())?)
    } else {
        None
    };
    let backend = backend.with_request_size(request_size.unwrap_or(Backend::DEFAULT_REQUEST_SIZE));
    if request_size.is_some_and(|size| size > Backend::DEFAULT_REQUEST_SIZE) {
        environment.warn_request_size(&backend)?;
    }
    if let (Framing::Lines, Some(name)) = (arguments.common.framing(), set.first_part()) {
        return Err(Failure::Reading(ReadingError::LinesPart(name.to_owned())));
    }
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
                arguments.common.details,
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
        let inputs = rows.enumerate().map(|(place, row)| {
            row.map(|record| crate::annotate_schedule::Input::Record(place + 1, record))
                .map_err(|error| crate::schedule::Placed::at(error, place + 1))
        });
        let mut output = Output::streaming(&mut writer, environment.usage());
        return if setting
            .is_some_and(|setting| setting != Setting::Records(std::num::NonZeroUsize::MIN))
        {
            batching::run(
                &judging,
                &reading,
                inputs,
                setting.unwrap_or(Setting::Max),
                environment.cancel(),
                &mut output,
            )
        } else {
            crate::annotate_schedule::run(
                &judging,
                &reading,
                inputs,
                environment.cancel(),
                &mut output,
            )
        };
    }
    let mut chunks = edge::numbered(edge::Chunks::new(source, reading.streams()), &reading);
    if arguments.common.dry_run {
        return dry_run(
            &set,
            &backend,
            &reading,
            profile.as_ref(),
            &mismatch,
            chunks.next().map(|(_, row)| row).transpose()?,
            arguments.common.details,
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
    let inputs = chunks.map(|(at, row)| {
        row.map(|bytes| crate::annotate_schedule::Input::Bytes(at, bytes))
            .map_err(|error| crate::schedule::Placed::at(error, at))
    });
    let mut output = Output::streaming(&mut writer, environment.usage());
    if setting.is_some_and(|setting| setting != Setting::Records(std::num::NonZeroUsize::MIN)) {
        batching::run(
            &judging,
            &reading,
            inputs,
            setting.unwrap_or(Setting::Max),
            environment.cancel(),
            &mut output,
        )
    } else {
        crate::annotate_schedule::run(
            &judging,
            &reading,
            inputs,
            environment.cancel(),
            &mut output,
        )
    }
}

fn refuse_views(arguments: &AnnotateArguments) -> Result<(), Failure> {
    if let Some(policy) = arguments.on_error.as_deref() {
        if policy != "continue" {
            return Err(Failure::Usage("--on-error takes continue"));
        }
        if !arguments.common.jsonl
            || !arguments.common.details
            || arguments.batching.batch.as_deref() != Some("1")
            || arguments.common.dry_run
        {
            return Err(Failure::Usage(
                "--on-error continue needs --jsonl --details --batch 1 and cannot accompany --dry-run",
            ));
        }
    }
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
    if arguments.batching.context.is_some() {
        return Err(Failure::Usage(
            "--context does not act on annotate; each question's `on` selects its evidence",
        ));
    }
    Ok(())
}

fn batch_setting(
    arguments: &AnnotateArguments,
    environment: &Environment,
    set: &QuestionSet,
    streams: bool,
) -> Result<Option<Setting>, Failure> {
    if !streams {
        return if arguments.batching.batch.is_some() {
            Err(Failure::Usage(
                "--batch groups the records of a stream, and a single text is one record",
            ))
        } else {
            Ok(None)
        };
    }
    let flag = arguments.batching.batch.as_deref();
    let selected = flag.or_else(|| environment.batch());
    if let Some(text) = selected {
        return Setting::parse(text)
            .map(Some)
            .ok_or(Failure::Usage(if flag.is_some() {
                "--batch takes max or a whole number of at least 1"
            } else {
                "THINKTHEN_BATCH takes max or a whole number of at least 1"
            }));
    }
    set.batch().map_or(Ok(Some(Setting::Max)), |value| {
        Setting::of_json(value)
            .map(Some)
            .ok_or(Failure::QuestionSet(QuestionSetError::Shape {
                path: "batch".to_owned(),
                wanted: "takes max or a whole number of at least 1",
            }))
    })
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
    continue_missing: bool,
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
            continue_missing: arguments.on_error.is_some(),
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

    pub(crate) const fn set(&self) -> &QuestionSet {
        &self.set
    }

    pub(crate) fn cancel(&self) -> &crate::engine::Cancel<'static> {
        self.environment.cancel()
    }

    pub(crate) fn record(
        &self,
        base: &Reading,
        input: crate::annotate_schedule::Input,
    ) -> Result<Record, Failure> {
        let record = match input {
            crate::annotate_schedule::Input::Bytes(_, bytes) => base
                .annotation_record(&bytes)
                .map_err(|error| Failure::record(error, base.streams()))?,
            crate::annotate_schedule::Input::Record(_, record) => record,
        };
        if !self.common.details {
            collisions(&self.set, &record)?;
        }
        Ok(record)
    }

    pub(crate) fn groups(&self) -> Vec<Vec<usize>> {
        self.set.groups()
    }

    pub(crate) const fn continue_missing(&self) -> bool {
        self.continue_missing
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
    ) -> Result<PreparedGroup, PrepareError> {
        let plan = plan_for(&self.set, &places, self.engine.backend(), base, record)?;
        self.engine
            .prepare_group(&plan, places)
            .map_err(|error| PrepareError::Other(error.into()))
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
) -> Result<Plan, PrepareError> {
    if group.is_empty() {
        return Err(PrepareError::Other(Failure::Defect(
            "an annotate group is empty",
        )));
    }
    let record = base
        .batch_record(record)
        .map_err(|error| PrepareError::Other(error.into()))?;
    let evidence = set
        .group_evidence(group, &record)
        .map_err(|error| match error {
            PartError::Reading(error) => PrepareError::Other(error.into()),
            PartError::Record(RecordError::Missed(pointer)) => PrepareError::MissingOn(pointer),
            PartError::Record(error) => PrepareError::Other(error.into()),
        })?;
    let questions = group
        .iter()
        .map(|place| {
            set.questions()
                .get(*place)
                .map(|named| named.question().clone())
                .ok_or(PrepareError::Other(Failure::Defect(
                    "a group points outside its set",
                )))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Plan::new(evidence, backend.model().clone(), questions)
        .map_err(|_| PrepareError::Other(Failure::Defect("an annotate group asks nothing")))
}
