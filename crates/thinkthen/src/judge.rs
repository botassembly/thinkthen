//! The one flow every judging verb takes, from what was asked to what is printed.

use std::io::{Read, Write};
use std::process::ExitCode;
use std::time::Duration;

use thinkthen_core::recording::Exchange as Recorded;
use thinkthen_core::systemone;
use thinkthen_core::{
    Backend, DecisionResult, Framing, Labels, Meta, Outcome, Plan, PlanDocument, Pointer, Question,
    QuestionText, Reading, Record, Reply, Threshold, json_line,
};

use crate::args::{ChooseArguments, Common, DecideArguments, ScoreArguments};
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::{self, Exchange};
use crate::recorder::Recorder;

/// One question, its rule, and the view its answer prints in.
#[derive(Debug)]
struct Asked<'a> {
    common: &'a Common,
    question: Question,
    threshold: Option<Threshold>,
    view: View,
}

/// Which of the three views of one answer the command line asked for.
#[derive(Clone, Copy, Debug)]
struct View {
    quiet: bool,
    raw: bool,
    details: bool,
}

impl View {
    /// Refuse two views of one answer, which no run can print at once.
    ///
    /// `channels.md` makes an option that cannot act in the chosen mode a usage
    /// error. `--raw` prints a bare label, so it acts in neither other view.
    const fn checked(self) -> Result<Self, Failure> {
        if self.quiet && self.details {
            return Err(Failure::QuietWithDetails);
        }
        if self.raw && (self.quiet || self.details) {
            return Err(Failure::RawWithAnotherView);
        }
        Ok(self)
    }
}

/// Answer the question about the evidence, and set the exit code from the answer.
///
/// # Errors
///
/// Returns [`Failure`] for every outcome `specification/channels.md` gives an
/// exit code other than 0, 1, and 3.
pub(crate) fn decide(
    arguments: &DecideArguments,
    environment: &Environment,
    input: impl Read,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let threshold = match arguments.threshold.as_deref() {
        Some(text) => text.parse()?,
        None => Threshold::default(),
    };
    let question = Question::Decide {
        text: QuestionText::new(arguments.question.as_str())?,
    };
    let view = View {
        quiet: arguments.quiet,
        raw: false,
        details: arguments.common.details,
    };
    run(
        Asked {
            common: &arguments.common,
            question,
            threshold: Some(threshold),
            view,
        },
        environment,
        input,
        writer,
    )
}

/// Pick one label from the options, and set the exit code from the answer.
///
/// # Errors
///
/// Returns [`Failure`] for a band on `choose`, for a list of options the verb
/// does not take, and for every outcome `channels.md` gives a code above 3.
pub(crate) fn choose(
    arguments: &ChooseArguments,
    environment: &Environment,
    input: impl Read,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let threshold = match arguments.threshold.as_deref() {
        Some(text) => {
            let rule: Threshold = text.parse()?;
            if !rule.is_cut() {
                return Err(Failure::BandOnChoose);
            }
            Some(rule)
        }
        None => None,
    };
    let question = Question::Choose {
        text: QuestionText::new(arguments.question.as_str())?,
        options: Labels::options(arguments.options.clone())?,
    };
    let view = View {
        quiet: arguments.quiet,
        raw: arguments.raw,
        details: arguments.common.details,
    };
    run(
        Asked {
            common: &arguments.common,
            question,
            threshold,
            view,
        },
        environment,
        input,
        writer,
    )
}

/// Place the evidence on the levels and print the weighted position.
///
/// # Errors
///
/// Returns [`Failure`] for a rule, which `score` has none of, for a list of
/// levels the verb does not take, and for every outcome `channels.md` gives a
/// code above 3.
pub(crate) fn score(
    arguments: &ScoreArguments,
    environment: &Environment,
    input: impl Read,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    if arguments.threshold.is_some() {
        return Err(Failure::RuleOnScore);
    }
    let question = Question::Score {
        text: QuestionText::new(arguments.question.as_str())?,
        levels: Labels::levels(arguments.levels.clone())?,
    };
    let view = View {
        quiet: false,
        raw: false,
        details: arguments.common.details,
    };
    run(
        Asked {
            common: &arguments.common,
            question,
            threshold: None,
            view,
        },
        environment,
        input,
        writer,
    )
}

/// Send the question over every record, and print one answer for each.
fn run(
    asked: Asked<'_>,
    environment: &Environment,
    input: impl Read,
    mut writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Asked {
        common,
        question,
        threshold,
        view,
    } = asked;
    let view = view.checked()?;
    if common.dry_run && (common.record.is_some() || common.replay.is_some()) {
        return Err(Failure::DryRunWithRecording);
    }
    let backend = Backend::resolve(
        common.url.as_deref(),
        environment.base_url(),
        common.model.as_str(),
    )?;
    let reading = read_by(common)?;
    if view.quiet && reading.streams() {
        return Err(Failure::QuietOverRecords);
    }
    let mut chunks = edge::Chunks::new(
        edge::source(common.input.as_deref(), input)?,
        reading.streams(),
    );

    if common.dry_run {
        return plan(
            &backend,
            &reading,
            &question,
            chunks.next().transpose()?,
            writer,
        );
    }

    let judging = Judging {
        common,
        environment,
        recorder: Recorder::of(common.record.as_deref(), common.replay.as_deref())?,
        backend,
        question,
        threshold,
        view,
        streams: reading.streams(),
    };
    if !judging.streams {
        let bytes = chunks.next().transpose()?.unwrap_or_default();
        let (outcome, _) = judging.one(&reading, &bytes, &mut writer)?;
        return Ok(exit_code(outcome));
    }
    // Every record before this one finished, so its place is that count.
    let mut replayed = 0;
    for (finished, chunk) in chunks.enumerate() {
        let stop = |cause| Failure::Stopped {
            at: finished + 1,
            finished,
            replayed,
            cause: Box::new(cause),
        };
        let bytes = chunk.map_err(&stop)?;
        let (_, from_recording) = judging.one(&reading, &bytes, &mut writer).map_err(stop)?;
        replayed += usize::from(from_recording);
    }
    Ok(ExitCode::SUCCESS)
}

/// Read the framing and the pointers the command line asked for.
fn read_by(common: &Common) -> Result<Reading, Failure> {
    let framing = match (common.lines, common.jsonl) {
        (true, _) => Framing::Lines,
        (_, true) => Framing::Jsonl,
        _ => Framing::Document,
    };
    let mut fields = Vec::with_capacity(common.field.len());
    for typed in &common.field {
        let pointer =
            Pointer::new(typed.as_str()).map_err(|error| Failure::Pointer(typed.clone(), error))?;
        fields.push(pointer);
    }
    Ok(Reading::new(framing, fields)?)
}

/// Print the plan for the first record, and read no further than that record.
fn plan(
    backend: &Backend,
    reading: &Reading,
    question: &Question,
    first: Option<Vec<u8>>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Some(bytes) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let (_, plan) = asked_of(reading, &bytes, backend, question)?;
    let document = PlanDocument::of(backend, &plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
    let document = if reading.streams() {
        document.reading(reading)
    } else {
        document
    };
    edge::write_line(writer, &json_line(&document)?)?;
    Ok(ExitCode::SUCCESS)
}

/// Read one record and build the one request it asks, which both paths do.
fn asked_of(
    reading: &Reading,
    bytes: &[u8],
    backend: &Backend,
    question: &Question,
) -> Result<(Record, Plan), Failure> {
    let record = reading.record(bytes)?;
    let plan = Plan::new(
        reading.evidence(&record)?,
        backend.model().clone(),
        vec![question.clone()],
    )
    .map_err(|_| Failure::Defect("a plan of one question asks nothing"))?;
    Ok((record, plan))
}

/// One question over one backend, asked of every record in turn.
#[derive(Debug)]
struct Judging<'a> {
    common: &'a Common,
    environment: &'a Environment,
    recorder: Recorder,
    backend: Backend,
    question: Question,
    threshold: Option<Threshold>,
    view: View,
    streams: bool,
}

impl Judging<'_> {
    /// Ask one record, print its answer, and say whether a recording answered.
    fn one(
        &self,
        reading: &Reading,
        bytes: &[u8],
        mut writer: impl Write,
    ) -> Result<(Outcome, bool), Failure> {
        let (record, plan) = asked_of(reading, bytes, &self.backend, &self.question)?;
        let (reply, replayed) = ask(
            &self.backend,
            &plan,
            self.common,
            self.environment,
            &self.recorder,
        )?;
        let answer = reply
            .answers()
            .first()
            .ok_or(Failure::Defect("the adapter answered no question"))?
            .clone();
        let (value, outcome) = answer.read(self.threshold);
        if self.view.details {
            let meta = Meta::new(
                env!("CARGO_PKG_VERSION"),
                self.backend.url().clone(),
                reply.model().clone(),
                reply.usage(),
                replayed,
            );
            let row =
                DecisionResult::new(value, self.question.clone(), answer, self.threshold, meta);
            let row = if self.streams {
                row.with_input(record)
            } else {
                row
            };
            edge::write_line(&mut writer, &json_line(&row)?)?;
        } else if self.view.raw {
            // One line stands for one record, so an unresolved record prints
            // an empty line. On one document it prints nothing at all.
            match value.label() {
                Some(label) => edge::write_line(&mut writer, label)?,
                None if self.streams => edge::write_line(&mut writer, "")?,
                None => {}
            }
        } else if !self.view.quiet {
            edge::write_line(&mut writer, &json_line(&value)?)?;
        }
        Ok((outcome, replayed))
    }
}

/// Answer the plan from the recording folder, or from the backend itself.
///
/// The recording is read before a key is, so a replay opens no connection and
/// needs no key. Only an exchange the adapter read is recorded.
fn ask(
    backend: &Backend,
    plan: &Plan,
    common: &Common,
    environment: &Environment,
    recorder: &Recorder,
) -> Result<(Reply, bool), Failure> {
    let body = systemone::encode(plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
    let recorded = Recorded::new(backend.url(), &body);
    if let Some(response) = recorder.replayed(&recorded)? {
        return Ok((systemone::decode(plan, &response)?, true));
    }
    let key = edge::key()?;
    let answered = http::post(&Exchange {
        url: backend.url().as_str(),
        body: &body,
        key: &key,
        timeout: Duration::from_secs(common.timeout),
        max_retries: common.max_retries,
        retry_wait: environment.retry_wait(),
    })?;
    let reply = systemone::decode(plan, &answered)?;
    recorder.record(&recorded, &answered)?;
    Ok((reply, false))
}

/// Turn the outcome into the exit code `specification/channels.md` fixes.
fn exit_code(outcome: Outcome) -> ExitCode {
    match outcome {
        Outcome::Yes => ExitCode::from(0),
        Outcome::No => ExitCode::from(1),
        Outcome::Unresolved => ExitCode::from(3),
    }
}
