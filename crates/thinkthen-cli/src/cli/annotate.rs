//! Apply one saved question set to documents or record streams.

use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;

use crate::core::{
    Backend, Framing, Pointer, QuestionSet, QuestionSetError, Reading, ReadingError, Record,
    Setting,
};

use crate::args::{AnnotateArguments, Common};
use crate::asking::{self, Folders};
use crate::cli::question_text;
use crate::edge::Environment;
use crate::engine::facade::Engine;
use crate::failure::Failure;
use crate::profile::{self, Mismatch};
use crate::schedule::Output;

mod asker;
pub(crate) mod error_row;
mod native;
mod plan;

pub(crate) fn run(
    arguments: &AnnotateArguments,
    environment: &Environment,
    admitted: crate::AdmittedRequest,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    arguments.common.check_plan_name()?;
    refuse_views(arguments)?;
    // `@FILE` names the same file, as the other verbs' question files do.
    let path = arguments
        .questions
        .to_str()
        .and_then(|typed| typed.strip_prefix('@'));
    let text = match path {
        Some(path) => question_text::reference(Path::new(path), Failure::OpenQuestionSet),
        None => question_text::read(arguments.questions.as_path(), Failure::OpenQuestionSet),
    }?;
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
    let backend = environment.resolve(
        arguments.common.backend.as_deref(),
        arguments.common.url.as_deref(),
        arguments.common.model.as_deref(),
    )?;
    let folders = Folders::of(&arguments.common, environment)?;
    if arguments.common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let profile = profile::read(&arguments.common, environment, &backend)?;
    let mismatch = Mismatch::new(set.profile(), profile.as_ref());
    crate::cli::intake::window(&arguments.common, set.first_part().is_some())?;
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
    let inputs = crate::cli::intake::Intake::new(
        &arguments.common,
        &reading,
        input,
        set.first_part().is_some(),
    )?;
    let engine = crate::cli::construction::engine(
        &arguments.common,
        environment,
        folders,
        (backend, profile),
        arguments.common.jobs,
        false,
    )?;
    let judging = Judging::new(arguments, environment, engine, set, mismatch)?;
    if arguments.common.dry_run {
        return plan::dry_run(&judging, admitted, &reading, inputs, setting, &mut writer);
    }
    let mut output = Output::streaming(&mut writer, environment.usage());
    native::run(&judging, admitted, &reading, inputs, setting, &mut output)
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
                "--on-error continue needs --jsonl --details --batch 1 and cannot accompany --plan",
            ));
        }
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
        .first()
        .and_then(|path| crate::read_question_file(path).ok())
        .is_some_and(|text| QuestionSet::parse(&text).is_ok())
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
    context: Option<String>,
    context_field: Option<String>,
}

impl<'a> Judging<'a> {
    fn new(
        arguments: &'a AnnotateArguments,
        environment: &'a Environment,
        engine: Engine,
        set: QuestionSet,
        mismatch: Mismatch,
    ) -> Result<Self, Failure> {
        let common = &arguments.common;
        Ok(Self {
            context: asking::context::shared(arguments.batching.context.as_deref())?,
            context_field: arguments.batching.context_field.clone(),
            streams: common.framing() != Framing::Document,
            continue_missing: arguments.on_error.is_some(),
            engine,
            common,
            environment,
            set,
            mismatch,
        })
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

    pub(crate) const fn mismatch(&self) -> &Mismatch {
        &self.mismatch
    }

    pub(crate) const fn details(&self) -> bool {
        self.common.details
    }

    pub(crate) const fn streams(&self) -> bool {
        self.streams
    }

    pub(crate) const fn continue_missing(&self) -> bool {
        self.continue_missing
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
