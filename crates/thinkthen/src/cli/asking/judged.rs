//! `decide`, `filter`, `rank`, `choose`, `tag` and `score` over the one
//! question pipeline, by ADR 0111 section 4. A record thread frames records
//! on each ask, and each row comes back in input order.

use std::io::BufRead;
use std::process::ExitCode;
use std::sync::mpsc::Receiver;
use std::thread;

use super::{Asks, Judging, JudgingInput, RowContext, table_kind};
use crate::core::adapters::built_in::DecodeError;
use crate::core::pack::{self, Ask, PackError};
use crate::core::{
    AnswerOutcome, BackendProfile, BatchError, Evidence, ModelName, Plan, Reading, Record, Reply,
    Setting, Url, Usage, quoted_plan, quoted_plan_of,
};
use crate::edge;
use crate::engine::facade::{self, Judgment};
use crate::engine::pipeline::{self, Answered, Asker, Failed, Flow, Input, Packing, Port};
use crate::failure::Failure;
use crate::failure::context::Limits;
use crate::judge::Keeping;
use crate::schedule::{Judged, Output, Placed};
use crate::table::Rows as TableRows;

/// One framed record, and the bytes it arrived as when it arrived as a line.
pub(super) struct Held {
    pub(super) record: Record,
    pub(super) arrived: Option<Vec<u8>>,
    pub(super) at: usize,
}

pub(super) type Records = Box<dyn Iterator<Item = Result<Held, Placed>> + Send>;

/// Frame the input as records: table rows, lines, or one document.
pub(super) fn records(
    configuration: &JudgingInput<'_>,
    reading: &Reading,
    source: Box<dyn BufRead + Send>,
) -> Result<Records, Failure> {
    if let Some(kind) = table_kind(configuration.common) {
        let rows = TableRows::new(source, kind)?;
        return Ok(Box::new(rows.enumerate().map(|(place, row)| {
            row.map(|record| Held {
                record,
                arrived: None,
                at: place + 1,
            })
            .map_err(|error| Placed::at(error, place + 1))
        })));
    }
    let reading = reading.clone();
    let streams = reading.streams();
    Ok(Box::new(
        edge::numbered(edge::Chunks::new(source, streams), &reading).map(move |(at, bytes)| {
            let bytes = bytes.map_err(|error| Placed::at(error, at))?;
            let record = reading
                .record(&bytes)
                .map_err(|error| Placed::at(Failure::record(error, streams), at))?;
            Ok(Held {
                record,
                arrived: Some(bytes),
                at,
            })
        }),
    ))
}

/// What one record's plan needs besides the record.
pub(super) struct Planner<'a> {
    pub(super) asks: &'a Asks,
    pub(super) reading: &'a Reading,
    pub(super) model: &'a ModelName,
    pub(super) context: Option<Evidence>,
    pub(super) profile: Option<&'a BackendProfile>,
    pub(super) limits: Limits,
}

impl Planner<'_> {
    /// The quoted plan one record sends. A stream's record quotes the JSON
    /// value a batch has always quoted, and one document quotes its evidence.
    pub(super) fn plan(&self, record: &Record) -> Result<Plan, Failure> {
        let question = self.asks.of(record)?;
        let planned = if self.reading.streams() {
            let batch = self.reading.batch_record(record)?;
            quoted_plan_of(
                self.model.clone(),
                batch.evidence,
                &batch.value,
                self.context.as_ref(),
                vec![question],
                self.profile,
            )
        } else {
            quoted_plan(
                self.model.clone(),
                self.reading.evidence(record)?,
                None,
                vec![question],
                self.profile,
            )
        };
        planned.map_err(|error| self.limits.refused(error, false))
    }

    /// The wire questions one record sends.
    pub(super) fn asks(&self, url: &Url, record: &Record) -> Result<Vec<Ask>, Failure> {
        pack::asks(url, &self.plan(record)?)
            .map_err(|_| Failure::Defect("a request could not be written as JSON"))
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

struct JudgeAsker<'a> {
    judging: &'a Judging<'a>,
    planner: Planner<'a>,
    url: Url,
    downstream: edge::Downstream,
}

impl Asker for JudgeAsker<'_> {
    type Input = Held;
    type Row = Judged;
    type Error = Placed;

    fn label(&self, held: &Held) -> usize {
        held.at
    }

    fn asks(&self, held: &Held) -> Result<Vec<Ask>, Placed> {
        self.planner
            .asks(&self.url, &held.record)
            .map_err(|error| Placed::at(error, held.at))
    }

    fn row(&self, held: Held, answers: Vec<Answered>) -> Result<Judged, Placed> {
        let at = held.at;
        self.judged(held, &answers)
            .map_err(|error| Placed::at(error, at))
    }

    fn gone(&self) -> bool {
        self.downstream.gone()
    }
}

impl JudgeAsker<'_> {
    fn judged(&self, held: Held, answers: &[Answered]) -> Result<Judged, Failure> {
        let question = self.judging.asks.of(&held.record)?;
        let model = answers
            .first()
            .map_or("", |answered| &*answered.answered_by);
        let stored: Vec<_> = answers
            .iter()
            .map(|answered| answered.answer.as_deref().map_err(DecodeError::cause))
            .collect();
        let outcomes = pack::read(std::slice::from_ref(&question), &stored, model)
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
        let (value, outcome) = answer.read(self.judging.threshold);
        let usage = answers
            .iter()
            .try_fold(Usage::new(0, 0), |total, answered| {
                total.checked_plus(answered.usage?)
            });
        let cached = answers.iter().all(|answered| answered.cached);
        let model =
            ModelName::reported(model).map_err(|_| Failure::Defect("a reply named no model"))?;
        let judgment = Judgment {
            answer: answer.clone(),
            value,
            outcome,
            answered: facade::Answered {
                reply: Reply::new(model, vec![AnswerOutcome::Answered(answer.clone())], usage),
                replayed: cached,
                request: crate::core::recording::Digest::named(
                    answers
                        .first()
                        .map(|answered| answered.key.hex())
                        .unwrap_or_default(),
                ),
                requests_sent: answers.iter().map(|answered| answered.requests_sent).sum(),
            },
        };
        let mut attempts: Vec<_> = answers
            .iter()
            .flat_map(|answered| answered.attempts.iter().cloned())
            .collect();
        attempts.sort_by_key(crate::public::AttemptObservation::ordinal);
        attempts.dedup_by_key(|event| event.ordinal());
        let mut judged = self.judging.row_of(
            self.planner.reading,
            held.record,
            question,
            &judgment,
            RowContext {
                arrived: held.arrived.as_deref(),
                requests: answers.iter().map(|answered| answered.key.hex()).collect(),
                attempts,
            },
        )?;
        if cached {
            judged.model = None;
        }
        Ok(judged)
    }
}

/// Answer every record and print its row, or print the plan.
pub(super) fn run(
    configuration: JudgingInput<'_>,
    reading: &Reading,
    source: Box<dyn BufRead + Send>,
    setting: Option<Setting>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure> {
    let records = records(&configuration, reading, source)?;
    let streams = reading.streams();
    let context = configuration.context.as_ref().map(super::Context::evidence);
    let inputs = setting.and_then(|setting| match setting {
        Setting::Records(most) => Some(most.get()),
        Setting::Max => None,
    });
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
            model: judging.engine.backend().model(),
            context,
            profile: judging.engine.profile(),
            limits: Limits::new(judging.engine.profile()),
        },
        url: judging.engine.backend().url().clone(),
        judging: &judging,
        downstream: downstream.clone(),
    };
    let packing = Packing {
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
        Some(_) if !streams && ended.finished > 0 => Ok(super::exit_code(
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
        None if !streams => Ok(super::exit_code(
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
        result: Result<Judged, Failed<Placed>>,
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
        let (replayed, outcome) = (judged.replayed, judged.outcome);
        match output.take(judged) {
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
