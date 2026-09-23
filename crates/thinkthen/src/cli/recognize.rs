//! The staged recognize command over the shared splitter and relation planner.

use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use serde::Serialize;

use crate::args::{Common, RecognizeArguments};
use crate::asking::{Folders, ask_prepared};
use crate::core::{
    Answer, AnswerOutcome, Backend, BackendProfile, Description, Meta, ModelName, Outcome, Plan,
    Reading, RecognizeSpec, RecognizedName, Record, RecordValue, RelationEdge, RequestMeta,
    TokenAnswer, Usage, assemble_edges, assemble_names, json_line, kind_questions, plan_pairs,
    plan_relation, recognition_questions, recognize_sha256, tokenize,
};
use crate::edge::{self, Environment};
use crate::engine::error::Error as EngineError;
use crate::failure::Failure;
use crate::http::Client;
use crate::prepared_request::PreparedRequests;
use crate::profile;
use crate::recorder::Recorder;
use crate::schedule;
use crate::table::Rows as TableRows;

mod config;
mod dry_run;

#[derive(Clone, Debug, PartialEq, Serialize)]
struct Recognized {
    entities: Vec<RecognizedName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relations: Option<Vec<RelationEdge>>,
}

#[derive(Debug, Serialize)]
struct Detailed<'a> {
    schema: &'static str,
    value: &'a Recognized,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Record>,
    question: &'a RecognizeSpec,
    answer: StrengthInputs<'a>,
    meta: Meta,
}

#[derive(Debug, Serialize)]
struct StrengthInputs<'a> {
    tokens: &'a [TokenInput],
}

#[derive(Debug, Serialize)]
struct TokenInput {
    token: String,
    detection_probability: f64,
    kind_probabilities: Vec<f64>,
}

#[derive(Debug)]
struct Aggregate {
    model: Option<ModelName>,
    usage: Option<Usage>,
    replayed: bool,
    requests_sent: u64,
    requests: Vec<String>,
}

impl Default for Aggregate {
    fn default() -> Self {
        Self {
            model: None,
            usage: None,
            replayed: true,
            requests_sent: 0,
            requests: Vec::new(),
        }
    }
}

struct Running<'a> {
    common: &'a Common,
    environment: &'a Environment,
    backend: Backend,
    profile: Option<BackendProfile>,
    mismatch: profile::Mismatch,
    recorder: Recorder,
    client: Client,
}

#[expect(
    clippy::too_many_lines,
    reason = "the command edge keeps validation and mode selection in their observable order"
)]
pub(crate) fn run(
    arguments: &RecognizeArguments,
    environment: &Environment,
    input: impl Read + Send + 'static,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let mut spec = config::settle(arguments)?;
    let pointers = if arguments.common.field.is_empty() {
        spec.on.clone()
    } else {
        let fields = arguments
            .common
            .field
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        crate::core::pointers(&fields, crate::core::Source::CommandLine, "field")?
    };
    if let Some(model) = arguments.common.model.as_deref() {
        spec.model = Some(
            ModelName::new(model)
                .map_err(|_| Failure::Usage("--model is text, not white space"))?,
        );
    }
    let reading = Reading::new(config::framing(&arguments.common), pointers)?;
    let jobs = schedule::jobs_of(arguments.common.jobs, reading.streams())?;
    let source = edge::source(arguments.common.input.as_deref(), input)?;
    let configured = spec
        .model
        .as_ref()
        .map(ModelName::as_str)
        .or_else(|| environment.model());
    let backend = Backend::resolve(
        arguments.common.url.as_deref(),
        environment.base_url(),
        configured.unwrap_or(crate::core::DEFAULT_MODEL),
    )?;
    let selected_profile = profile::read(&arguments.common)?;
    let mismatch = profile::Mismatch::new(spec.profile.as_ref(), selected_profile.as_ref());

    if arguments.common.dry_run {
        let folders = Folders::of(&arguments.common, environment)?;
        if folders.named() {
            return Err(Failure::DryRunWithRecording);
        }
        return dry_run::run(
            &reading,
            source,
            &backend,
            selected_profile.as_ref(),
            (
                &spec,
                arguments
                    .kinds
                    .first()
                    .is_some_and(|kind| kind.starts_with('@')),
            ),
            &mut writer,
        );
    }

    let folders = Folders::of(&arguments.common, environment)?;
    let recording = folders.reported();
    let running = Running {
        common: &arguments.common,
        environment,
        client: Client::new(
            Duration::from_secs(arguments.common.timeout),
            backend.is_secure(),
        ),
        recorder: Recorder::of_private(
            folders.record.as_deref(),
            folders.replay.as_deref(),
            folders.private_default,
            folders.cache_answers,
        )?,
        backend,
        profile: selected_profile,
        mismatch,
    };
    if let Some(kind) = config::table_kind(&arguments.common) {
        let rows = TableRows::new(source, kind)?;
        return schedule::over_records(
            &|record: &Record| judged_record(&running, &reading, &spec, record.clone(), true),
            rows,
            jobs,
            recording,
            environment.cancel(),
            &mut schedule::Output::Streaming(&mut writer),
        );
    }
    let streams = reading.streams();
    let mut chunks = edge::Chunks::new(source, streams);
    if !streams {
        let bytes = chunks.next().transpose()?.unwrap_or_default();
        let record = reading
            .record(&bytes)
            .map_err(|error| Failure::record(error, streams))?;
        let judged = judged_record(&running, &reading, &spec, record, streams)?;
        schedule::Output::Streaming(&mut writer).take(judged)?;
        return Ok(ExitCode::SUCCESS);
    }
    schedule::over_records(
        &|bytes: &Vec<u8>| {
            let record = reading
                .record(bytes)
                .map_err(|error| Failure::record(error, streams))?;
            judged_record(&running, &reading, &spec, record, streams)
        },
        chunks,
        jobs,
        recording,
        environment.cancel(),
        &mut schedule::Output::Streaming(&mut writer),
    )
}

fn logical_failure() -> Failure {
    Failure::Recognize(crate::cli::failure::recognize::Error::LogicalQuestion)
}

fn judged_record(
    running: &Running<'_>,
    reading: &Reading,
    spec: &RecognizeSpec,
    record: Record,
    streams: bool,
) -> Result<schedule::Judged, Failure> {
    let evidence = reading.evidence(&record)?;
    let text = evidence.as_text()?.into_owned();
    let (value, inputs, aggregate) = recognize_one(running, spec, &text)?;
    let line = if running.common.details {
        let model = aggregate
            .model
            .unwrap_or_else(|| running.backend.model().clone());
        let meta = Meta::new(
            env!("CARGO_PKG_VERSION"),
            recognize_sha256(spec)?,
            running.backend.url().clone(),
            model,
            aggregate.usage,
            RequestMeta::new(
                aggregate.replayed,
                aggregate.requests_sent,
                aggregate.requests,
            )
            .with_profile_warning(running.mismatch.warning()),
        );
        json_line(&Detailed {
            schema: crate::core::RESULT_SCHEMA,
            value: &value,
            input: streams.then_some(record),
            question: spec,
            answer: StrengthInputs { tokens: &inputs },
            meta,
        })?
    } else if streams {
        json_line(&RecordValue::new(record, value))?
    } else {
        json_line(&value)?
    };
    Ok(schedule::Judged {
        printed: Some(line),
        outcome: Outcome::Yes,
        replayed: aggregate.replayed,
        probability: None,
        partial_failure: false,
        profile_mismatch: running.mismatch.notice(),
    })
}

fn recognize_one(
    running: &Running<'_>,
    spec: &RecognizeSpec,
    text: &str,
) -> Result<(Recognized, Vec<TokenInput>, Aggregate), Failure> {
    let tokens = tokenize(text);
    if tokens.is_empty() {
        return Ok((
            Recognized {
                entities: Vec::new(),
                relations: (!spec.relations.is_empty()).then(Vec::new),
            },
            Vec::new(),
            Aggregate::default(),
        ));
    }
    let mut questions = recognition_questions(&tokens)
        .map_err(|_| Failure::Defect("fixed detection questions are invalid"))?;
    questions.extend(
        kind_questions(&tokens, &spec.kinds)
            .map_err(|_| config::error(false, crate::core::RecognizeConfigError::Kinds))?,
    );
    let evidence = crate::core::Evidence::new(text)
        .map_err(|_| Failure::Defect("record evidence became blank"))?;
    let plan = Plan::new(evidence.clone(), running.backend.model().clone(), questions)
        .map_err(|_| Failure::Defect("recognize planned no token questions"))?;
    let (answers, mut aggregate) = execute(running, &plan)?;
    let token_answers = token_answers(&answers, tokens.len(), &spec.kinds)?;
    let kind_names = spec
        .kinds
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    let entities = assemble_names(
        text,
        &tokens,
        &token_answers,
        &kind_names,
        spec.threshold.cut_value().unwrap_or(0.5),
    );
    let inputs = tokens
        .iter()
        .zip(&token_answers)
        .map(|(token, answer)| TokenInput {
            token: token.text().to_owned(),
            detection_probability: answer.detection_probability,
            kind_probabilities: answer.kind_probabilities.clone(),
        })
        .collect();
    let relations = recognize_relations(running, spec, &evidence, &entities, &mut aggregate)?;
    Ok((
        Recognized {
            entities,
            relations,
        },
        inputs,
        aggregate,
    ))
}

fn recognize_relations(
    running: &Running<'_>,
    spec: &RecognizeSpec,
    evidence: &crate::core::Evidence,
    entities: &[RecognizedName],
    aggregate: &mut Aggregate,
) -> Result<Option<Vec<RelationEdge>>, Failure> {
    if spec.relations.is_empty() {
        return Ok(None);
    }
    let mut edges = Vec::new();
    for rule in &spec.relations {
        let mut planned = plan_relation(entities, rule)
            .map_err(|_| Failure::Defect("relation planning failed"))?;
        if planned.questions.is_empty() {
            continue;
        }
        let mut relation_plan = Plan::new(
            evidence.clone(),
            running.backend.model().clone(),
            planned.questions.clone(),
        )
        .map_err(|_| Failure::Defect("relation planned no questions"))?;
        let needs_pairs = matches!(
            PreparedRequests::with_profile(
                &running.backend,
                &relation_plan,
                running.profile.as_ref()
            ),
            Err(EngineError::ProfileLimit(limit)) if limit.permits_relation_fallback()
        );
        if needs_pairs {
            planned = plan_pairs(entities, rule)
                .map_err(|_| Failure::Defect("relation fallback planning failed"))?;
            relation_plan = Plan::new(
                evidence.clone(),
                running.backend.model().clone(),
                planned.questions.clone(),
            )
            .map_err(|_| Failure::Defect("relation fallback planned no questions"))?;
        }
        let (relation_answers, relation_meta) = execute(running, &relation_plan)?;
        aggregate.add(relation_meta)?;
        edges.extend(assemble_edges(
            entities,
            rule,
            &planned.mappings,
            &relation_answers,
            spec.relation_threshold.cut_value().unwrap_or(0.5),
        ));
    }
    Ok(Some(edges))
}

fn token_answers(
    answers: &[Answer],
    count: usize,
    kinds: &[(String, Option<Description>)],
) -> Result<Vec<TokenAnswer>, Failure> {
    let mut built = Vec::with_capacity(count);
    for place in 0..count {
        let detection = answers.get(place).ok_or_else(logical_failure)?;
        let detection_probabilities = detection
            .choice_probabilities()
            .ok_or_else(logical_failure)?;
        let in_probability = detection_probabilities
            .iter()
            .find_map(|(name, value)| (*name == "IN").then_some(*value))
            .ok_or_else(logical_failure)?;
        let out_probability = detection_probabilities
            .iter()
            .find_map(|(name, value)| (*name == "OUT").then_some(*value))
            .ok_or_else(logical_failure)?;
        let kind_probabilities = if kinds.len() == 1 {
            Vec::new()
        } else {
            answers
                .get(count + place)
                .and_then(Answer::choice_probabilities)
                .ok_or_else(logical_failure)?
        };
        let values = kinds
            .iter()
            .map(|(name, _)| {
                if kinds.len() == 1 {
                    return 1.0;
                }
                kind_probabilities
                    .iter()
                    .find_map(|(held, value)| (*held == name).then_some(*value))
                    .unwrap_or(0.0)
            })
            .collect::<Vec<_>>();
        let winner = values
            .iter()
            .enumerate()
            .max_by(|left, right| left.1.total_cmp(right.1).then_with(|| right.0.cmp(&left.0)))
            .map_or(0, |(index, _)| index);
        built.push(TokenAnswer {
            detected: in_probability > out_probability,
            detection_probability: in_probability,
            kind: winner,
            kind_probabilities: values,
        });
    }
    Ok(built)
}

fn execute(running: &Running<'_>, plan: &Plan) -> Result<(Vec<Answer>, Aggregate), Failure> {
    let prepared =
        PreparedRequests::with_profile(&running.backend, plan, running.profile.as_ref())?;
    let mut answers = Vec::new();
    let mut aggregate = Aggregate::default();
    for chunk in prepared.into_chunks() {
        let answered = ask_prepared(
            &running.backend,
            &chunk.plan,
            chunk.request,
            running.common,
            running.environment,
            &running.recorder,
            &running.client,
        )?;
        for outcome in answered.reply.outcomes() {
            match outcome {
                AnswerOutcome::Answered(answer) => answers.push(answer.clone()),
                AnswerOutcome::Failed(_) => return Err(logical_failure()),
            }
        }
        aggregate.add_answered(&answered)?;
    }
    Ok((answers, aggregate))
}

impl Aggregate {
    fn add_answered(
        &mut self,
        answered: &crate::prepared_request::Answered,
    ) -> Result<(), Failure> {
        if let Some(model) = &self.model {
            if model != answered.reply.model() {
                return Err(Failure::ModelsDiffer(Some((
                    model.as_str().to_owned(),
                    answered.reply.model().as_str().to_owned(),
                ))));
            }
        } else {
            self.model = Some(answered.reply.model().clone());
        }
        self.usage = match (self.usage, answered.reply.usage()) {
            (Some(left), Some(right)) => left
                .checked_plus(right)
                .ok_or(Failure::UsageOverflow)
                .map(Some)?,
            (None, held) | (held, None) => held,
        };
        self.replayed &= answered.replayed;
        self.requests_sent = self
            .requests_sent
            .checked_add(answered.requests_sent)
            .ok_or(Failure::UsageOverflow)?;
        self.requests.push(answered.request.as_str().to_owned());
        Ok(())
    }

    fn add(&mut self, other: Self) -> Result<(), Failure> {
        if let Some(model) = other.model {
            if self.model.as_ref().is_some_and(|held| held != &model) {
                return Err(Failure::ModelsDiffer(None));
            }
            self.model.get_or_insert(model);
        }
        self.usage = match (self.usage, other.usage) {
            (Some(left), Some(right)) => left
                .checked_plus(right)
                .ok_or(Failure::UsageOverflow)
                .map(Some)?,
            (None, held) | (held, None) => held,
        };
        self.replayed &= other.replayed;
        self.requests_sent = self
            .requests_sent
            .checked_add(other.requests_sent)
            .ok_or(Failure::UsageOverflow)?;
        self.requests.extend(other.requests);
        Ok(())
    }
}
