//! The one flow every judging verb takes, from what was asked to what is printed.

use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use thinkthen_core::adapters::built_in;
use thinkthen_core::recording::Exchange as Recorded;
use thinkthen_core::{
    Backend, DecisionResult, Framing, Meta, Outcome, Plan, PlanDocument, Pointer, Question,
    QuestionText, Reading, Record, Reply, Resolved, Sources, Threshold, json_line, question_sha256,
};

use crate::args::{ChooseArguments, Common, DecideArguments, ScoreArguments};
use crate::asked;
use crate::edge::{self, Environment};
use crate::failure::Failure;
use crate::http::{Client, Exchange};
use crate::recorder::Recorder;
use crate::schedule::{self, Judged};

/// One question, its rule, and the view its answer prints in.
#[derive(Debug)]
struct Asked<'a> {
    common: &'a Common,
    asks: Asks,
    settled: &'a Resolved,
    view: View,
}

/// Where one record's question comes from.
///
/// Every verb but `choose --options` asks the same question of every record.
/// `--options` names a pointer, and each record holds its own candidate list
/// there, so the question is built again for each one.
#[derive(Debug)]
enum Asks {
    /// One question, asked of every record.
    Fixed(Question),
    /// A pick whose options each record carries at this pointer.
    FromRecord {
        /// The question the model receives.
        text: QuestionText,
        /// Where in the record the candidate list sits.
        pointer: Pointer,
    },
}

impl Asks {
    /// Build the question this record is asked.
    fn of(&self, record: &Record) -> Result<Question, Failure> {
        match self {
            Self::Fixed(question) => Ok(question.clone()),
            Self::FromRecord { text, pointer } => Ok(Question::Choose {
                text: text.clone(),
                options: record.choices(pointer)?,
            }),
        }
    }
}

/// One record, the question it was asked, and the request that carries both.
#[derive(Debug)]
struct Sending {
    record: Record,
    question: Question,
    plan: Plan,
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
    let settled = asked::decide(arguments)?;
    let view = View {
        quiet: arguments.quiet,
        raw: false,
        details: arguments.common.details,
    };
    run(
        Asked {
            common: &arguments.common,
            asks: fixed(&settled)?,
            settled: &settled,
            view,
        },
        environment,
        input,
        writer,
    )
}

/// The one question every record is asked, which every verb but one has.
fn fixed(settled: &Resolved) -> Result<Asks, Failure> {
    settled
        .question()
        .cloned()
        .map(Asks::Fixed)
        .ok_or(Failure::Defect(
            "a verb with no list left its question open",
        ))
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
    let settled = asked::choose(arguments)?;
    let asks = match arguments.options_pointer.as_deref() {
        Some(typed) => {
            if !arguments.common.jsonl {
                return Err(Failure::OptionsOutsideJsonl);
            }
            Asks::FromRecord {
                text: settled.text().clone(),
                pointer: Pointer::new(typed)
                    .map_err(|error| Failure::Pointer("--options", typed.to_owned(), error))?,
            }
        }
        None => fixed(&settled)?,
    };
    let view = View {
        quiet: arguments.quiet,
        raw: arguments.raw,
        details: arguments.common.details,
    };
    run(
        Asked {
            common: &arguments.common,
            asks,
            settled: &settled,
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
    let settled = asked::score(arguments)?;
    let view = View {
        quiet: false,
        raw: false,
        details: arguments.common.details,
    };
    run(
        Asked {
            common: &arguments.common,
            asks: fixed(&settled)?,
            settled: &settled,
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
        asks,
        settled,
        view,
    } = asked;
    let threshold = settled.threshold();
    let view = view.checked()?;
    let folders = Folders::of(common)?;
    if common.dry_run && folders.named() {
        return Err(Failure::DryRunWithRecording);
    }
    let backend = Backend::resolve(
        common.url.as_deref(),
        environment.base_url(),
        settled.model().as_str(),
    )?;
    let reading = read_by(common, settled)?;
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
            &Planning {
                asks: &asks,
                sources: settled.sources().from_file().then(|| *settled.sources()),
            },
            chunks.next().transpose()?,
            writer,
        );
    }

    let jobs = schedule::jobs_of(common.jobs, reading.streams())?;
    let judging = Judging {
        common,
        environment,
        client: Client::new(Duration::from_secs(common.timeout), backend.is_secure()),
        recorder: Recorder::of(folders.record, folders.replay)?,
        backend,
        asks,
        threshold,
        view,
        streams: reading.streams(),
    };
    if !judging.streams {
        let bytes = chunks.next().transpose()?.unwrap_or_default();
        let judged = judging.row(&reading, &bytes)?;
        if let Some(line) = judged.printed {
            edge::write_line(&mut writer, &line)?;
        }
        return Ok(exit_code(judged.outcome));
    }
    schedule::over_records(
        &|bytes| judging.row(&reading, bytes),
        &mut chunks,
        jobs,
        &mut writer,
    )
}

/// The folders `--record`, `--replay`, and `--cache` name between them.
#[derive(Debug)]
struct Folders<'a> {
    record: Option<&'a Path>,
    replay: Option<&'a Path>,
}

impl<'a> Folders<'a> {
    /// Read the two folders, with `--cache` standing for both at once.
    ///
    /// # Errors
    ///
    /// Returns [`Failure::CacheWithRecording`] when `--cache` is given beside
    /// one of the two options it stands for.
    fn of(common: &'a Common) -> Result<Self, Failure> {
        let Some(cached) = common.cache.as_deref() else {
            return Ok(Self {
                record: common.record.as_deref(),
                replay: common.replay.as_deref(),
            });
        };
        if common.record.is_some() || common.replay.is_some() {
            return Err(Failure::CacheWithRecording);
        }
        Ok(Self {
            record: Some(cached),
            replay: Some(cached),
        })
    }

    /// True when a folder is named at all, which a plan may not name.
    const fn named(&self) -> bool {
        self.record.is_some() || self.replay.is_some()
    }
}

/// Read the framing the command line asked for, over the settled pointers.
fn read_by(common: &Common, settled: &Resolved) -> Result<Reading, Failure> {
    let framing = match (common.lines, common.jsonl) {
        (true, _) => Framing::Lines,
        (_, true) => Framing::Jsonl,
        _ => Framing::Document,
    };
    Ok(Reading::new(framing, settled.on().to_vec())?)
}

/// What a plan shows beyond the request: the question and where it came from.
struct Planning<'a> {
    asks: &'a Asks,
    sources: Option<Sources>,
}

/// Print the plan for the first record, and read no further than that record.
fn plan(
    backend: &Backend,
    reading: &Reading,
    planning: &Planning<'_>,
    first: Option<Vec<u8>>,
    writer: impl Write,
) -> Result<ExitCode, Failure> {
    let Some(bytes) = first else {
        return Ok(ExitCode::SUCCESS);
    };
    let sending = asked_of(reading, &bytes, backend, planning.asks)?;
    let document = PlanDocument::of(backend, &sending.plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
    let document = if reading.streams() {
        document.reading(reading)
    } else {
        document
    };
    let document = match planning.sources {
        Some(sources) => document.from(sources),
        None => document,
    };
    edge::write_line(writer, &json_line(&document)?)?;
    Ok(ExitCode::SUCCESS)
}

/// Read one record and build the one request it asks, which both paths do.
fn asked_of(
    reading: &Reading,
    bytes: &[u8],
    backend: &Backend,
    asks: &Asks,
) -> Result<Sending, Failure> {
    let record = reading.record(bytes)?;
    let question = asks.of(&record)?;
    let plan = Plan::new(
        reading.evidence(&record)?,
        backend.model().clone(),
        vec![question.clone()],
    )
    .map_err(|_| Failure::Defect("a plan of one question asks nothing"))?;
    Ok(Sending {
        record,
        question,
        plan,
    })
}

/// One question over one backend, asked of every record in turn.
#[derive(Debug)]
struct Judging<'a> {
    common: &'a Common,
    environment: &'a Environment,
    recorder: Recorder,
    backend: Backend,
    client: Client,
    asks: Asks,
    threshold: Option<Threshold>,
    view: View,
    streams: bool,
}

impl Judging<'_> {
    /// Ask one record and build the line its answer prints.
    ///
    /// Nothing here touches the writer, so a worker thread may call it and the
    /// one thread that owns standard output prints the lines in input order.
    fn row(&self, reading: &Reading, bytes: &[u8]) -> Result<Judged, Failure> {
        let sending = asked_of(reading, bytes, &self.backend, &self.asks)?;
        let (reply, replayed) = ask(
            &self.backend,
            &sending.plan,
            self.common,
            self.environment,
            &self.recorder,
            &self.client,
        )?;
        let answer = reply
            .answers()
            .first()
            .ok_or(Failure::Defect("the adapter answered no question"))?
            .clone();
        let (value, outcome) = answer.read(self.threshold);
        let printed = if self.view.details {
            let meta = Meta::new(
                env!("CARGO_PKG_VERSION"),
                question_sha256(&sending.question, self.threshold)?,
                self.backend.url().clone(),
                reply.model().clone(),
                reply.usage(),
                replayed,
            );
            let row = DecisionResult::new(value, sending.question, answer, self.threshold, meta);
            let row = if self.streams {
                row.with_input(sending.record)
            } else {
                row
            };
            Some(json_line(&row)?)
        } else if self.view.raw {
            // One line stands for one record, so an unresolved record prints
            // an empty line. On one document it prints nothing at all.
            match value.label() {
                Some(label) => Some(label.to_owned()),
                None if self.streams => Some(String::new()),
                None => None,
            }
        } else if self.view.quiet {
            None
        } else {
            Some(json_line(&value)?)
        };
        Ok(Judged {
            printed,
            outcome,
            replayed,
        })
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
    client: &Client,
) -> Result<(Reply, bool), Failure> {
    let body = built_in::encode(plan)
        .map_err(|_| Failure::Defect("a request could not be written as JSON"))?;
    let recorded = Recorded::new(backend.url(), &body);
    if let Some(response) = recorder.replayed(&recorded)? {
        return Ok((built_in::decode(plan, &response)?, true));
    }
    let key = edge::key()?;
    let answered = client.post(&Exchange {
        url: backend.url().as_str(),
        body: &body,
        key: &key,
        max_retries: common.max_retries,
        retry_wait: environment.retry_wait(),
    })?;
    let reply = built_in::decode(plan, &answered)?;
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
