//! `decide`, `filter`, `rank`, `choose`, `tag` and `score` over the one
//! question pipeline, by ADR 0111 section 4. A record thread frames records
//! on each ask, and each row comes back in input order.

use std::process::ExitCode;
use std::sync::mpsc::Receiver;
use std::thread;

use super::{Asks, Judging, JudgingInput, RowContext};
use crate::core::pack::{self, Ask, PackError};
use crate::core::{
    AnswerOutcome, BackendProfile, BatchError, Descriptions, Evidence, ModelName, Plan, Reading,
    Record, Setting, Url, quoted_plan, quoted_plan_of,
};
use crate::edge;
use crate::engine::facade::Judgment;
use crate::engine::pipeline::{self, Answered, Asker, Failed, Flow, Input, Packing, Port};
use crate::failure::Failure;
use crate::failure::context::Limits;
use crate::judge::Keeping;
use crate::schedule::{Judged, Output, Placed};

/// One framed record, and the bytes it arrived as when it arrived as a line.
pub(super) struct Held {
    pub(super) record: Record,
    pub(super) images: Option<crate::public::ImageEvidence>,
    pub(super) arrived: Option<Vec<u8>>,
    pub(super) at: usize,
    pub(super) position: Option<crate::cli::intake::Position>,
}

pub(super) type Records = Box<dyn Iterator<Item = Result<Held, Placed>> + Send>;

/// Frame the input as records: table rows, lines, or one document.
pub(super) fn records(
    configuration: &JudgingInput<'_>,
    reading: &Reading,
    source: crate::cli::intake::Intake,
) -> Result<Records, Failure> {
    let reading = reading.clone();
    let streams = reading.streams();
    let limited = configuration.keeping == Keeping::Ordered && configuration.common.located();
    let mut remaining = crate::core::MAX_RECORD_BYTES;
    let mut source = source;
    let mut stopped = false;
    Ok(Box::new(std::iter::from_fn(move || {
        if stopped {
            return None;
        }
        let held = source.next()?.and_then(|item| {
            let (record, arrived, images) = match item.data {
                crate::cli::intake::Data::Record(record) => (record, None, None),
                crate::cli::intake::Data::Images(images) => (
                    reading
                        .record(images.text().unwrap_or_default().as_bytes())
                        .map_err(|error| Placed::at(Failure::record(error, streams), item.at))?,
                    None,
                    Some(images),
                ),
                crate::cli::intake::Data::Bytes(bytes) => {
                    let record = reading
                        .record(&bytes)
                        .map_err(|error| Placed::at(Failure::record(error, streams), item.at))?;
                    (record, Some(bytes), None)
                }
            };
            let held = Held {
                record,
                images,
                arrived,
                at: item.at,
                position: item.position,
            };
            if limited {
                charge(&reading, &held, &mut remaining)?;
            }
            Ok(held)
        });
        stopped = held.is_err();
        Some(held)
    })))
}

/// Charge original evidence at source admission, leaving the iterator tail unread.
fn charge(reading: &Reading, held: &Held, remaining: &mut usize) -> Result<(), Placed> {
    let record_error = |error| Placed::at(Failure::record(error, reading.streams()), held.at);
    let bytes = match &held.arrived {
        Some(bytes) => reading.as_it_arrived(bytes).map_err(record_error)?.len(),
        None => reading
            .evidence(&held.record)
            .map_err(record_error)?
            .as_text()
            .map_err(|error| Placed::at(Failure::from(error), held.at))?
            .len(),
    };
    *remaining = remaining.checked_sub(bytes).ok_or_else(|| {
        Placed::at(
            Failure::Usage("source rank reads at most 16 MiB across all input records"),
            held.at,
        )
    })?;
    Ok(())
}

/// What one record's plan needs besides the record.
pub(super) struct Planner<'a> {
    pub(super) asks: &'a Asks,
    pub(super) reading: &'a Reading,
    pub(super) asked: (ModelName, Descriptions),
    pub(super) context: Option<Evidence>,
    pub(super) profile: Option<&'a BackendProfile>,
    pub(super) limits: Limits,
    pub(super) route: crate::core::adapters::built_in::images::ImageRoute,
}

impl Planner<'_> {
    /// The quoted plan one record sends. A stream's record quotes the JSON
    /// value a batch has always quoted, and one document quotes its evidence.
    pub(super) fn plans(&self, held: &Held) -> Result<Vec<Plan>, Failure> {
        if let Some(images) = &held.images {
            if self.reading.declares_item() && images.text().is_none() {
                return Err(Failure::Record(crate::core::RecordError::ItemSchema));
            }
            if self.reading.declares_item() {
                self.reading.evidence(&held.record)?;
            }
            return crate::core::image::plan(
                self.asked.clone(),
                images.state(),
                self.context.as_ref(),
                self.asks.questions(&held.record)?,
                self.profile,
                self.route,
            )
            .map(|plan| vec![plan])
            .map_err(|error| self.limits.refused(error, false));
        }
        let record = &held.record;
        self.asks
            .questions(record)?
            .into_iter()
            .map(|question| self.plan(record, question))
            .collect()
    }

    fn plan(&self, record: &Record, question: crate::core::Question) -> Result<Plan, Failure> {
        let planned = if self.reading.streams() {
            let batch = self.reading.batch_record(record)?;
            quoted_plan_of(
                self.asked.clone(),
                batch.evidence,
                &batch.value,
                self.context.as_ref(),
                vec![question],
                self.profile,
            )
        } else {
            quoted_plan(
                self.asked.clone(),
                self.reading.evidence(record)?,
                None,
                vec![question],
                self.profile,
            )
        };
        planned.map_err(|error| self.limits.refused(error, false))
    }

    /// The wire questions one record sends.
    pub(super) fn asks(&self, url: &Url, held: &Held) -> Result<Vec<Ask>, Failure> {
        let mut asks = Vec::new();
        for plan in self.plans(held)? {
            asks.extend(pack::asks(url, &plan).map_err(|error| super::encoded(&plan, error))?);
        }
        Ok(asks)
    }

    /// The command's refusal of a question the packer cannot send.
    pub(super) fn refused(&self, error: PackError) -> Failure {
        match error {
            PackError::Profile(limit) => Failure::ProfileLimit(limit),
            PackError::Context {
                initial,
                kind,
                limit,
                actual,
            } => self.limits.refused(
                BatchError::ContextOverLimit {
                    kind,
                    limit,
                    actual,
                },
                initial,
            ),
        }
    }
}

pub(super) struct JudgeAsker<'a> {
    pub(super) judging: &'a Judging<'a>,
    planner: Planner<'a>,
    url: Url,
    downstream: edge::Downstream,
}

impl Asker for JudgeAsker<'_> {
    type Input = Held;
    type Row = Vec<Judged>;
    type Error = Placed;

    fn validates_batches(&self) -> bool {
        self.planner.reading.declares_item()
    }

    fn label(&self, held: &Held) -> usize {
        held.at
    }

    fn asks(&self, held: &Held) -> Result<Vec<Ask>, Placed> {
        self.planner
            .asks(&self.url, held)
            .map_err(|error| Placed::at(error, held.at))
    }

    fn row(&self, held: Held, answers: Vec<Answered>) -> Result<Vec<Judged>, Placed> {
        let at = held.at;
        self.judged(held, &answers)
            .map_err(|error| Placed::at(error, at))
    }

    fn gone(&self) -> bool {
        self.downstream.gone()
    }
}

impl JudgeAsker<'_> {
    fn judged(&self, held: Held, answers: &[Answered]) -> Result<Vec<Judged>, Failure> {
        super::rank_set::rows(self, held, answers)
    }

    pub(super) fn one(
        &self,
        held: &Held,
        question: crate::core::Question,
        answers: &[Answered],
    ) -> Result<Judged, Failure> {
        let outcomes = pipeline::read(&question, answers)
            .map_err(|error| Failure::from(crate::engine::error::Error::from(error)))?;
        let [AnswerOutcome::Answered(answer)] = outcomes.as_slice() else {
            let failed = answers.iter().find_map(|answered| {
                answered
                    .answer
                    .as_ref()
                    .err()
                    .map(|error| (error, answered.span))
            });
            // One document's failed answer refuses the reply, as a request
            // for one record always has.
            return Err(match failed {
                Some((error, _)) if !self.planner.reading.streams() => {
                    Failure::Reply(error.clone())
                }
                Some((_, (first, last))) => Failure::PartialReply { first, last },
                None => Failure::PartialReply {
                    first: held.at,
                    last: held.at,
                },
            });
        };
        let answer = answer.clone();
        let (value, outcome) = answer.read(self.judging.threshold);
        let judgment = Judgment {
            answered: pipeline::receipt(answers, outcomes).map_err(Failure::from)?,
            answer,
            value,
            outcome,
        };
        let mut attempts: Vec<_> = answers
            .iter()
            .flat_map(|answered| answered.attempts.iter().cloned())
            .collect();
        attempts.sort_by_key(crate::public::AttemptObservation::ordinal);
        attempts.dedup_by_key(|event| event.ordinal());
        let mut judged = self.judging.row_of(
            self.planner.reading,
            held.record.clone(),
            question,
            &judgment,
            RowContext {
                record: held
                    .at
                    .checked_sub(1)
                    .ok_or(Failure::Defect("an input occurrence lost its ordinal"))?,
                arrived: held.arrived.as_deref(),
                requests: answers.iter().map(|answered| answered.key.hex()).collect(),
                attempts,
                position: held.position.as_ref(),
                images: held.images.as_ref(),
            },
        )?;
        if judgment.answered.replayed {
            judged.model = None;
        }
        Ok(judged)
    }
}

/// Answer every record and print its row, or print the plan.
pub(super) fn run(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    source: crate::cli::intake::Intake,
    setting: Option<Setting>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let records = records(&configuration, reading, source)?;
    let streams = reading.streams();
    let documents = configuration.documents;
    let context = configuration.context.as_ref().map(super::Context::evidence);
    let inputs = if !streams {
        Some(1)
    } else {
        setting.and_then(|setting| match setting {
            Setting::Records(most) => Some(most.get()),
            Setting::Max => None,
        })
    };
    if configuration.common.dry_run {
        return super::plan::packed(&configuration, reading, records, context, inputs, output);
    }
    if matches!(configuration.keeping, Keeping::Passing | Keeping::Ordered) {
        output.guard_models();
    }
    let judging = Judging::new(configuration)?;
    let downstream = edge::Downstream::default();
    let asker = JudgeAsker {
        planner: Planner {
            asks: &judging.asks,
            reading,
            asked: judging.engine.backend().asked(),
            context,
            profile: judging.engine.profile(),
            limits: Limits::new(judging.engine.profile()),
            route: judging.engine.backend().image_route(),
        },
        url: judging.engine.backend().url().clone(),
        judging: &judging,
        downstream: downstream.clone(),
    };
    let packing = Packing {
        questions: None,
        sized: true,
        inputs,
        context: asker.planner.context.is_some(),
        detailed: judging.view.details,
        continues: false,
    };
    super::plan::check_context(&asker.planner, judging.engine.backend(), packing)?;
    let reader_downstream = downstream.clone();
    let mut ended = Ended::default();
    judging
        .engine
        .ask_all(
            &asker,
            packing,
            pipeline::reader(
                judging.environment.input_pause(),
                move |asks, port| {
                    thread::spawn(move || feed(records, &asks, &port, &reader_downstream));
                },
                |_, result| ended.take(result, &asker.planner, output),
            ),
            judging.environment.cancel(),
        )
        .map_err(Failure::from)?;
    if downstream.latched() || ended.closed {
        return Ok(ExitCode::SUCCESS);
    }
    match ended.stop {
        // A stop after one document's row changes nothing it printed.
        Some(_) if !streams && !documents && ended.finished > 0 => Ok(super::exit_code(
            ended.outcome.unwrap_or(crate::core::Outcome::Unresolved),
        )),
        Some(stop) if !streams => Err(stop
            .cause
            .with_replay_context(crate::failure::ReplayContext::Document(judging.asks.verb()))),
        Some(stop) => Err(Failure::Stopped {
            at: stop.at.unwrap_or(ended.finished + 1),
            finished: ended.finished,
            replayed: ended.replayed,
            recording: judging.engine.recording(),
            held: matches!(judging.keeping, Keeping::Ordered),
            cause: Box::new(stop.cause),
        }),
        None if !streams && !documents => Ok(super::exit_code(
            ended.outcome.unwrap_or(crate::core::Outcome::Unresolved),
        )),
        None => {
            output.ended()?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// What the rows the host took add up to.
#[derive(Default)]
struct Ended {
    finished: usize,
    replayed: usize,
    outcome: Option<crate::core::Outcome>,
    stop: Option<Placed>,
    closed: bool,
}

impl Ended {
    fn take(
        &mut self,
        result: Result<Vec<Judged>, Failed<Placed>>,
        planner: &Planner<'_>,
        output: &mut Output<'_>,
    ) -> Flow {
        let judged = match result {
            Ok(judged) => judged,
            Err(failed) => {
                self.stop = Some(placed(failed, planner));
                return Flow::Stop;
            }
        };
        let replayed = judged.iter().all(|row| row.replayed);
        let outcome = judged
            .first()
            .map_or(crate::core::Outcome::Unresolved, |row| row.outcome);
        match output.take_members(judged, self.finished) {
            Ok(true) => {
                self.finished += 1;
                self.replayed += usize::from(replayed);
                self.outcome = Some(outcome);
                Flow::Continue
            }
            Ok(false) => {
                self.closed = true;
                Flow::Stop
            }
            Err(error) => {
                self.stop = Some(Placed::from(error));
                Flow::Stop
            }
        }
    }
}

/// The command's failure for one input with no row.
pub(super) fn placed(failed: Failed<Placed>, planner: &Planner<'_>) -> Placed {
    match failed {
        Failed::Asker(placed) => placed,
        Failed::Pack { error, at } => Placed::at(planner.refused(error), at),
        Failed::Engine { error, first, last } => {
            Placed::at(ranged(error.into(), first, last), first)
        }
        Failed::Stopped(error) => Placed::from(error),
    }
}

/// A request or a whole reply that failed two or more records names their
/// range on one line. Every other cause stays as it is.
fn ranged(cause: Failure, first: usize, last: usize) -> Failure {
    match cause {
        Failure::Transport(_) | Failure::Status(_) | Failure::TokenLimit | Failure::Reply(_)
            if last > first =>
        {
            Failure::BatchFailed {
                last,
                cause: Box::new(cause),
            }
        }
        other => other,
    }
}

/// Frame one record per ask, and stop after the first refusal or once the
/// output is gone.
pub(super) fn feed<T>(
    mut records: impl Iterator<Item = Result<T, Placed>>,
    asks: &Receiver<()>,
    port: &Port<T, Placed>,
    downstream: &edge::Downstream,
) {
    while asks.recv().is_ok() {
        let next = if downstream.gone() {
            Input::End
        } else {
            match records.next() {
                None => Input::End,
                Some(Ok(record)) => Input::Item(record),
                Some(Err(error)) => Input::Failed(error),
            }
        };
        let last = !matches!(next, Input::Item(_));
        if port.send(next).is_err() || last {
            return;
        }
    }
}
