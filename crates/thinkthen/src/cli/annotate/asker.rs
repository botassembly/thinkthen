//! `annotate` over the one question pipeline, by ADR 0111 section 5: every
//! group's questions share the fixed state, so records and groups pack
//! together, and each record's row gathers its questions back by group.

use std::io::Write as _;
use std::process::ExitCode;
use std::thread;

use super::{Judging, PrepareError, plan_for};
use crate::core::adapters::built_in::DecodeError;
use crate::core::pack::{self, Ask};
use crate::core::{AnswerOutcome, Question, Reading, Record, Url};
use crate::engine::facade::{GroupAnswer, QuestionAnswer};
use crate::engine::pipeline::{self, Answered, Asker, Failed, Flow, Input, Packing, Port};
use crate::failure::{Failure, ReplayContext};
use crate::schedule::{Judged, Output, Placed};

pub(crate) type Framed = crate::cli::intake::Item;

/// One parsed input and the number its messages name.
pub(super) struct Held {
    at: usize,
    record: Record,
    position: Option<crate::cli::intake::Position>,
}

/// Why one input has no row: a pointer that missed its record, or another failure.
pub(super) struct Refused {
    at: usize,
    error: PrepareError,
}

struct AnnotateAsker<'a> {
    judging: &'a Judging<'a>,
    reading: &'a Reading,
    url: Url,
    downstream: crate::edge::Downstream,
}

impl Asker for AnnotateAsker<'_> {
    type Input = Held;
    type Row = Judged;
    type Error = Refused;

    fn validates_batches(&self) -> bool {
        self.judging.set().questions().iter().any(|member| {
            member.metadata().item_schema.is_some() || member.metadata().context_schema.is_some()
        })
    }

    fn refuses_batch(&self, error: &Refused) -> bool {
        !matches!(error.error, PrepareError::MissingOn(_))
    }

    fn label(&self, held: &Held) -> usize {
        held.at
    }

    fn asks(&self, held: &Held) -> Result<Vec<Ask>, Refused> {
        asks(self.judging, self.reading, &self.url, &held.record)
            .map(|asks| asks.into_iter().flat_map(|(_, asks)| asks).collect())
            .map_err(|error| Refused { at: held.at, error })
    }

    fn row(&self, held: Held, answers: Vec<Answered>) -> Result<Judged, Refused> {
        let at = held.at;
        let refused = |error| Refused {
            at,
            error: PrepareError::Other(error),
        };
        let groups = grouped(self.judging, &answers).map_err(refused)?;
        let original = held
            .position
            .as_ref()
            .filter(|p| p.located)
            .map(|_| held.record.clone());
        let mut judged = self.judging.finish(held.record, groups).map_err(refused)?;
        if let (Some(position), Some(original), Some(line)) = (
            held.position.as_ref().filter(|p| p.located),
            original,
            &mut judged.printed,
        ) {
            *line = crate::cli::intake::source_value(&original, line, position).map_err(refused)?;
        }
        if self.judging.details() {
            crate::cli::intake::locate(&mut judged.printed, held.position.as_ref())
                .map_err(refused)?;
        }
        Ok(judged)
    }

    fn gone(&self) -> bool {
        self.downstream.gone()
    }
}

/// One group's number and its wire questions.
pub(super) type GroupAsks = (usize, Vec<Ask>);

/// Every group's wire questions for one record, in set order, each beside
/// its group's number.
pub(super) fn asks(
    judging: &Judging<'_>,
    reading: &Reading,
    url: &Url,
    record: &Record,
) -> Result<Vec<GroupAsks>, PrepareError> {
    judging
        .groups()
        .into_iter()
        .enumerate()
        .map(|(group, places)| {
            let plan = plan_for(
                judging.set(),
                &places,
                judging.engine().backend(),
                judging.engine().profile(),
                reading,
                record,
            )?;
            let asks = pack::asks_for(judging.engine().backend().api_type(), url, &plan).map_err(
                |_| PrepareError::Other(Failure::Defect("a request could not be written as JSON")),
            )?;
            Ok((group, asks))
        })
        .collect()
}

/// Gather one record's answers back into its groups. A group with no
/// usable answer fails the record, as a whole refusal would.
fn grouped(judging: &Judging<'_>, answers: &[Answered]) -> Result<Vec<GroupAnswer>, Failure> {
    let mut rest = answers;
    let mut groups = Vec::new();
    for places in judging.groups() {
        let mut questions = Vec::with_capacity(places.len());
        let mut usable = false;
        let mut failed = None;
        for place in places {
            let question: Question = judging
                .set()
                .questions()
                .get(place)
                .map(|named| named.question().clone())
                .ok_or(Failure::Defect("a group points outside its set"))?;
            let (own, after) = rest
                .split_at_checked(pack::wire_count(&question))
                .ok_or(Failure::Defect("an annotate question lost its answers"))?;
            rest = after;
            let answer = QuestionAnswer::read(place, question, own)?;
            match answer.reply.outcomes() {
                [AnswerOutcome::Answered(_)] => usable = true,
                _ => failed = failed.or_else(|| first_failure(own)),
            }
            questions.push(answer);
        }
        if !usable {
            // A group of one document with no usable answer refuses the
            // reply, as that group's own request always has.
            return Err(match failed {
                Some((error, _)) if !judging.streams() => Failure::Reply(error.clone()),
                Some((_, (first, last))) => Failure::PartialReply { first, last },
                None => Failure::PartialReply { first: 0, last: 0 },
            });
        }
        groups.push(GroupAnswer::of_questions(questions));
    }
    Ok(groups)
}

/// The first wire answer that failed, and the span of its request.
fn first_failure(own: &[Answered]) -> Option<(&DecodeError, (usize, usize))> {
    own.iter().find_map(|answered| {
        answered
            .answer
            .as_ref()
            .err()
            .map(|error| (error, answered.span))
    })
}

/// Annotate every input and print each row in input order.
pub(super) fn run<I>(
    judging: &Judging<'_>,
    reading: &Reading,
    inputs: I,
    inputs_cap: Option<usize>,
    output: &mut Output<'_>,
) -> Result<ExitCode, Failure>
where
    I: Iterator<Item = Result<Framed, Placed>> + Send + 'static,
{
    let asker = AnnotateAsker {
        judging,
        reading,
        url: judging.engine().backend().url().clone(),
        downstream: crate::edge::Downstream::default(),
    };
    let packing = Packing {
        questions: None,
        sized: true,
        inputs: inputs_cap,
        context: false,
        detailed: false,
        continues: false,
    };
    let parse = Parser::of(judging, reading);
    let mut ended = Ended::default();
    let cancel = judging.cancel();
    judging
        .engine()
        .ask_all(
            &asker,
            packing,
            pipeline::reader(
                judging.environment.input_pause(),
                move |asks, port: Port<Held, Refused>| {
                    thread::spawn(move || parse.feed(inputs, &asks, &port));
                },
                |_, result| ended.take(result, judging, output),
            ),
            cancel,
        )
        .map_err(Failure::from)?;
    if ended.skipped > 0 {
        let count = ended.skipped;
        writeln!(
            std::io::stderr().lock(),
            "thinkthen: {count} record{} skipped",
            if count == 1 { "" } else { "s" }
        )
        .map_err(Failure::Output)?;
    }
    // A stop after one document's row changes nothing it printed.
    if let Some(stop) = ended.stop.filter(|_| {
        reading.streams()
            || judging.common.input.len() > 1
            || judging.common.located()
            || ended.finished == 0
    }) {
        if !reading.streams() {
            return Err(stop.cause);
        }
        return Err(Failure::Stopped {
            at: stop.at.unwrap_or(ended.finished + 1),
            finished: ended.finished,
            replayed: ended.replayed,
            recording: judging.engine().recording(),
            held: false,
            cause: Box::new(stop.cause),
        });
    }
    Ok(if ended.skipped > 0 {
        ExitCode::from(7)
    } else if ended.partial {
        ExitCode::from(6)
    } else {
        ExitCode::SUCCESS
    })
}

#[derive(Default)]
struct Ended {
    finished: usize,
    replayed: usize,
    skipped: usize,
    partial: bool,
    stop: Option<Placed>,
}

impl Ended {
    fn take(
        &mut self,
        result: Result<Judged, Failed<Refused>>,
        judging: &Judging<'_>,
        output: &mut Output<'_>,
    ) -> Flow {
        let (judged, skipped) = match result {
            Ok(judged) => (judged, false),
            Err(Failed::Asker(Refused {
                at,
                error: PrepareError::MissingOn(pointer),
            })) if judging.continue_missing() => {
                match crate::annotate::error_row::missed(at, &pointer) {
                    Ok(judged) => (judged, true),
                    Err(error) => return self.stopped(Placed::at(error, at)),
                }
            }
            Err(failed) => return self.stopped(placed(failed, judging)),
        };
        let (replayed, partial) = (judged.replayed, judged.partial_failure);
        match output.take(judged) {
            Ok(true) => {
                self.finished += 1;
                self.replayed += usize::from(replayed);
                self.partial |= partial;
                self.skipped += usize::from(skipped);
                Flow::Continue
            }
            Ok(false) => Flow::Stop,
            Err(error) => self.stopped(Placed::from(error)),
        }
    }

    fn stopped(&mut self, placed: Placed) -> Flow {
        self.stop = Some(placed);
        Flow::Stop
    }
}

fn placed(failed: Failed<Refused>, judging: &Judging<'_>) -> Placed {
    match failed {
        Failed::Asker(Refused { at, error }) => Placed::at(error.into_failure(), at),
        Failed::Pack { error, at } => Placed::at(
            match error {
                pack::PackError::Profile(limit) => Failure::ProfileLimit(limit),
                pack::PackError::Context { .. } => Failure::Defect("annotate packed a context"),
            },
            at,
        ),
        Failed::Engine { error, first, last } => {
            let cause = Failure::from(error).with_replay_context(ReplayContext::Annotate);
            let cause = match cause {
                Failure::Transport(_)
                | Failure::Status(_)
                | Failure::TokenLimit
                | Failure::Reply(_)
                    if last > first || judging.streams() =>
                {
                    Failure::BatchFailed {
                        last,
                        cause: Box::new(cause),
                    }
                }
                other => other,
            };
            Placed::at(cause, first)
        }
        Failed::Stopped(error) => Placed::from(error),
    }
}

/// Parses each input on the reader's thread, so a bad record stops the
/// input where it stands.
pub(super) struct Parser {
    reading: Reading,
    set: crate::core::QuestionSet,
    details: bool,
}

impl Parser {
    pub(super) fn of(judging: &Judging<'_>, reading: &Reading) -> Self {
        Self {
            reading: reading.clone(),
            set: judging.set().clone(),
            details: judging.details(),
        }
    }

    /// One input's record, as a dry run reads it.
    pub(super) fn record(&self, framed: Framed) -> Result<Record, Failure> {
        self.parse(framed)
            .map(|held| held.record)
            .map_err(|refused| refused.error.into_failure())
    }

    fn feed<I>(
        &self,
        mut inputs: I,
        asks: &std::sync::mpsc::Receiver<()>,
        port: &Port<Held, Refused>,
    ) where
        I: Iterator<Item = Result<Framed, Placed>>,
    {
        while asks.recv().is_ok() {
            let next = match inputs.next() {
                None => Input::End,
                Some(Ok(framed)) => match self.parse(framed) {
                    Ok(held) => Input::Item(held),
                    Err(refused) => Input::Failed(refused),
                },
                Some(Err(placed)) => Input::Failed(Refused {
                    at: placed.at.unwrap_or(0),
                    error: PrepareError::Other(placed.cause),
                }),
            };
            let last = !matches!(next, Input::Item(_));
            if port.send(next).is_err() || last {
                return;
            }
        }
    }

    fn parse(&self, framed: Framed) -> Result<Held, Refused> {
        let at = framed.at;
        let record = match framed.data {
            crate::cli::intake::Data::Bytes(bytes) => self
                .reading
                .annotation_record(&bytes)
                .map_err(|error| Refused {
                    at,
                    error: PrepareError::Other(Failure::record(error, self.reading.streams())),
                })?,
            crate::cli::intake::Data::Record(record) => record,
            crate::cli::intake::Data::Images(_) => {
                return Err(Refused {
                    at,
                    error: PrepareError::Other(Failure::Usage(
                        "annotate accepts text only; images are unsupported",
                    )),
                });
            }
        };
        if !self.details {
            super::collisions(&self.set, &record).map_err(|error| Refused {
                at,
                error: PrepareError::Other(error),
            })?;
        }
        Ok(Held {
            at,
            record,
            position: framed.position,
        })
    }
}
