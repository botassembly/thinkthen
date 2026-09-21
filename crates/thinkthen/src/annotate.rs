//! Apply one saved question set to documents or record streams.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use thinkthen_core::adapters::built_in;
use thinkthen_core::recording::Exchange as Recorded;
use thinkthen_core::{
    AnnotateMeta, AnnotateResult, AnnotatedAnswer, Backend, Framing, ModelName, Outcome, Plan,
    PlanDocument, Pointer, QuestionSet, Reading, Record, Reply, Usage, Value, json_line,
};

use crate::args::{AnnotateArguments, Common};
use crate::asking::ask;
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::Client;
use crate::recorder::Recorder;
use crate::schedule::{Judged, Output};
use crate::table::{Kind as TableKind, Rows as TableRows};

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
    let folders = folders(&arguments.common)?;
    if arguments.common.dry_run && (folders.0.is_some() || folders.1.is_some()) {
        return Err(Failure::DryRunWithRecording);
    }
    let model = arguments
        .common
        .model
        .as_deref()
        .unwrap_or(built_in::DEFAULT_MODEL);
    let backend = Backend::resolve(
        arguments.common.url.as_deref(),
        environment.base_url(),
        model,
    )?;
    let reading = reading(&arguments.common)?;
    let source = edge::source(arguments.common.input.as_deref(), input)?;
    if let Some(kind) = table_kind(&arguments.common) {
        let mut rows = TableRows::new(source, kind)?;
        if arguments.common.dry_run {
            return dry_run_record(
                &set,
                &backend,
                &reading,
                rows.next().transpose()?,
                &mut writer,
            );
        }
        let judging = Judging {
            common: &arguments.common,
            environment,
            recorder: Recorder::of(folders.0, folders.1)?,
            client: Client::new(
                Duration::from_secs(arguments.common.timeout),
                backend.is_secure(),
            ),
            backend,
            set,
        };
        let mut output = Output::Streaming(&mut writer);
        let jobs = arguments.common.jobs.map_or(4, usize::from);
        return crate::annotate_schedule::run(
            &judging,
            &reading,
            rows.map(|row| row.map(crate::annotate_schedule::Input::Record)),
            jobs,
            &mut output,
        );
    }
    let mut chunks = edge::Chunks::new(source, reading.streams());
    if arguments.common.dry_run {
        return dry_run(
            &set,
            &backend,
            &reading,
            chunks.next().transpose()?,
            &mut writer,
        );
    }
    let judging = Judging {
        common: &arguments.common,
        environment,
        recorder: Recorder::of(folders.0, folders.1)?,
        client: Client::new(
            Duration::from_secs(arguments.common.timeout),
            backend.is_secure(),
        ),
        backend,
        set,
    };
    let mut output = Output::Streaming(&mut writer);
    let jobs = arguments.common.jobs.map_or(4, usize::from);
    crate::annotate_schedule::run(
        &judging,
        &reading,
        chunks.map(|row| row.map(crate::annotate_schedule::Input::Bytes)),
        jobs,
        &mut output,
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

type FolderPair<'a> = (Option<&'a Path>, Option<&'a Path>);

fn folders(common: &Common) -> Result<FolderPair<'_>, Failure> {
    if let Some(cache) = common.cache.as_deref() {
        if common.record.is_some() || common.replay.is_some() {
            return Err(Failure::CacheWithRecording);
        }
        return Ok((Some(cache), Some(cache)));
    }
    Ok((common.record.as_deref(), common.replay.as_deref()))
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

fn dry_run(
    set: &QuestionSet,
    backend: &Backend,
    base: &Reading,
    first: Option<Vec<u8>>,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let Some(bytes) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let record = base
        .annotation_record(&bytes)
        .map_err(|error| Failure::record(error, base.streams()))?;
    dry_run_record(set, backend, base, Some(record), writer)
}

fn dry_run_record(
    set: &QuestionSet,
    backend: &Backend,
    base: &Reading,
    first: Option<Record>,
    writer: &mut dyn Write,
) -> Result<ExitCode, Failure> {
    let Some(record) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    collisions(set, &record)?;
    let group = set
        .groups()
        .into_iter()
        .next()
        .ok_or(Failure::Defect("a set has no group"))?;
    let plan = plan_for(set, &group, backend, base, &record)?;
    let on = set
        .questions()
        .iter()
        .map(|question| (question.name().to_owned(), question.on().to_vec()))
        .collect();
    let document = PlanDocument::of(backend, &plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?
        .questions_on(on);
    let document = if base.streams() {
        document.reading(base)
    } else {
        document
    };
    edge::write_line(writer, &json_line(&document)?)?;
    Ok(ExitCode::SUCCESS)
}

pub(crate) struct Judging<'a> {
    common: &'a Common,
    environment: &'a Environment,
    recorder: Recorder,
    client: Client,
    backend: Backend,
    set: QuestionSet,
}

impl Judging<'_> {
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
        let mut values: Vec<Option<Value>> = vec![None; self.set.questions().len()];
        let mut details: Vec<Option<AnnotatedAnswer>> = vec![None; self.set.questions().len()];
        let mut model: Option<ModelName> = None;
        let mut usage: Option<Usage> = Some(Usage::new(0, 0));
        let mut replayed = true;
        for answered in answered {
            let GroupAnswer {
                places,
                reply,
                digest,
                replayed: was_replayed,
            } = answered;
            check_model(&mut model, reply.model(), self.backend.model())?;
            usage = match (usage, reply.usage()) {
                (Some(total), Some(next)) => {
                    Some(total.checked_plus(next).ok_or(Failure::UsageOverflow)?)
                }
                _ => None,
            };
            replayed &= was_replayed;
            take_answers(
                &self.set,
                &places,
                &reply,
                &digest,
                &mut values,
                &mut details,
            )?;
        }
        let named_values = pair_values(&self.set, values)?;
        let printed = if self.common.details {
            let named_details = pair_details(&self.set, details)?;
            let meta = AnnotateMeta::new(
                env!("CARGO_PKG_VERSION"),
                self.set.sha256()?,
                self.backend.url().clone(),
                model.ok_or(Failure::Defect("no group reported a model"))?,
                usage,
                replayed,
            );
            json_line(&AnnotateResult::new(
                record,
                named_values,
                named_details,
                meta,
            ))?
        } else {
            json_line(&record.annotated(named_values))?
        };
        Ok(Judged {
            printed: Some(printed),
            outcome: Outcome::Yes,
            replayed,
            probability: None,
        })
    }

    pub(crate) fn answer_group(
        &self,
        base: &Reading,
        record: &Record,
        places: Vec<usize>,
    ) -> Result<GroupAnswer, Failure> {
        let plan = plan_for(&self.set, &places, &self.backend, base, record)?;
        let request = built_in::encode(&plan)
            .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
        let digest = Recorded::new(self.backend.url(), &request)
            .digest()
            .as_str()
            .to_owned();
        let (reply, replayed) = ask(
            &self.backend,
            &plan,
            self.common,
            self.environment,
            &self.recorder,
            &self.client,
        )?;
        Ok(GroupAnswer {
            places,
            reply,
            digest,
            replayed,
        })
    }
}

pub(crate) struct GroupAnswer {
    pub(crate) places: Vec<usize>,
    pub(crate) reply: Reply,
    pub(crate) digest: String,
    pub(crate) replayed: bool,
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
        let nested_record = nested.record(base_evidence.as_str().as_bytes())?;
        nested.evidence(&nested_record)?
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

fn take_answers(
    set: &QuestionSet,
    group: &[usize],
    reply: &Reply,
    digest: &str,
    values: &mut [Option<Value>],
    details: &mut [Option<AnnotatedAnswer>],
) -> Result<(), Failure> {
    if group.len() != reply.answers().len() {
        return Err(Failure::Defect(
            "the adapter answered the wrong number of questions",
        ));
    }
    for (place, answer) in group.iter().zip(reply.answers()) {
        let question = set
            .questions()
            .get(*place)
            .ok_or(Failure::Defect("a group points outside its set"))?;
        let (value, _) = answer.read(question.threshold());
        let value_slot = values
            .get_mut(*place)
            .ok_or(Failure::Defect("an answer points outside its set"))?;
        *value_slot = Some(value.clone());
        let detail_slot = details
            .get_mut(*place)
            .ok_or(Failure::Defect("an answer points outside its set"))?;
        *detail_slot = Some(AnnotatedAnswer::new(
            value,
            question.question().clone(),
            answer.clone(),
            question.threshold(),
            digest.to_owned(),
        ));
    }
    Ok(())
}

fn pair_values(
    set: &QuestionSet,
    values: Vec<Option<Value>>,
) -> Result<Vec<(String, Value)>, Failure> {
    set.questions()
        .iter()
        .zip(values)
        .map(|(question, value)| {
            Ok((
                question.name().to_owned(),
                value.ok_or(Failure::Defect("a question has no value"))?,
            ))
        })
        .collect()
}

fn pair_details(
    set: &QuestionSet,
    values: Vec<Option<AnnotatedAnswer>>,
) -> Result<Vec<(String, AnnotatedAnswer)>, Failure> {
    set.questions()
        .iter()
        .zip(values)
        .map(|(question, value)| {
            Ok((
                question.name().to_owned(),
                value.ok_or(Failure::Defect("a question has no detailed answer"))?,
            ))
        })
        .collect()
}

pub(crate) fn check_model(
    first: &mut Option<ModelName>,
    next: &ModelName,
    requested: &ModelName,
) -> Result<(), Failure> {
    let Some(held) = first else {
        *first = Some(next.clone());
        return Ok(());
    };
    if held == next {
        return Ok(());
    }
    let safe = built_in::diagnostic_model(held.as_str(), requested.as_str())
        .then(|| held.as_str().to_owned())
        .zip(
            built_in::diagnostic_model(next.as_str(), requested.as_str())
                .then(|| next.as_str().to_owned()),
        )
        .map(|(first, second)| {
            if first <= second {
                (first, second)
            } else {
                (second, first)
            }
        });
    Err(Failure::ModelsDiffer(safe))
}
