//! The `thinkthen` command.

mod annotate;
mod filter;
mod rank;
mod relate;
mod saved_run;
mod table;

use std::cell::Cell;
use std::io::{self, BufRead, Read, Write};
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use thinkthen::{
    Answer, Backend, Batch, Cancel, ChoiceOption, Choose, Decide, Engine, Error, ErrorKind, Family,
    Find, Function, Outcome, Pointer, Recognize, Record, Row, Score, Tag,
};

/// Ask typed questions about evidence and branch on the answer.
#[derive(Debug, Parser)]
#[command(name = "thinkthen", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run a saved question or a declared JSON-lines pipeline.
    Run(saved_run::Args),
    /// Answer a yes or no question about the text on standard input.
    ///
    /// Prints true, false or unsure, and exits 0, 1 or 3. With --lines or
    /// --jsonl, prints one JSON row per record and exits 0 when no row failed.
    Decide(DecideArgs),
    /// Select one nonblank line of each input record that answers the question.
    ///
    /// Prints the selected line, nothing when none fits, or unsure below the cut.
    Find(DecideArgs),
    /// Print the original records that answer yes, in input order.
    Filter(DecideArgs),
    /// Choose the best fitting option, or print unsure and exit 3.
    Choose(ChooseArgs),
    /// Print every label whose probability reaches the threshold.
    Tag(TagArgs),
    /// Return a number from 0 to 1 on an ordered scale.
    Score(ScoreArgs),
    /// Print records best first, preserving input order for ties.
    Rank(RankArgs),
    /// Print recognized names as JSON objects in text order.
    Recognize(RecognizeArgs),
    /// Print relations the text states between recognized names.
    Relate(RelateArgs),
    /// Answer every named question in a toolbox for each record.
    Annotate(AnnotateArgs),
}

#[derive(Debug, Args)]
struct AnnotateArgs {
    /// The inline JSON toolbox of named questions to ask.
    toolbox: String,
    #[command(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args)]
struct DecideArgs {
    /// The literal question to ask about the text.
    question: String,
    #[command(flatten)]
    common: CommonArgs,
    /// A probability cut such as 0.7. Decide and filter also accept a band
    /// such as 0.2:0.8 with a not sure middle; find accepts only a cut.
    #[arg(long)]
    threshold: Option<String>,
}

#[derive(Debug, Args)]
struct ChooseArgs {
    /// The question to ask about the text.
    question: String,
    /// An option label. Repeat in order for each possible answer.
    #[arg(long = "option", required = false)]
    options: Vec<String>,
    #[command(flatten)]
    common: CommonArgs,
    /// A cut such as 0.7, or a band such as 0.2:0.8 with a not sure middle.
    #[arg(long)]
    threshold: Option<String>,
}

#[derive(Debug, Args)]
struct TagArgs {
    /// The question to ask about the text.
    question: String,
    #[command(flatten)]
    common: CommonArgs,
    /// A label to judge independently. Repeat for each label.
    #[arg(long = "label", required = false)]
    labels: Vec<String>,
    /// A single probability cut, such as 0.7.
    #[arg(long)]
    threshold: Option<String>,
}

#[derive(Debug, Args)]
struct ScoreArgs {
    /// The question to ask about the text.
    question: String,
    /// A scale level. Repeat from lowest to highest.
    #[arg(long = "level")]
    levels: Vec<String>,
    #[command(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args)]
struct RankArgs {
    /// A literal yes or no question.
    question: String,
    /// A scale level. Repeat from lowest to highest to rank by score.
    #[arg(long = "level")]
    levels: Vec<String>,
    /// Keep only the first N records.
    #[arg(long)]
    top: Option<String>,
    #[command(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args)]
struct RecognizeArgs {
    /// Ordered kinds.
    kinds: Vec<String>,
    #[command(flatten)]
    common: CommonArgs,
    /// A single minimum probability cut.
    #[arg(long)]
    threshold: Option<String>,
}

#[derive(Debug, Args)]
struct RelateArgs {
    /// Ordered NAME=SOURCE:TARGET rules.
    #[arg(required = true)]
    rules: Vec<String>,
    /// Ask each unordered pair once for every inline rule.
    #[arg(long)]
    either: bool,
    /// A single minimum relation probability cut.
    #[arg(long)]
    threshold: Option<String>,
    #[command(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args)]
struct CommonArgs {
    #[command(flatten)]
    table: table::TableArgs,
    /// The backend that answers questions.
    #[arg(long, default_value = "systemone")]
    adapter: String,

    /// The model to ask.
    #[arg(long)]
    model: Option<String>,
    /// The backend's base address. Beats THINKTHEN_BASE_URL.
    #[arg(long)]
    url: Option<String>,
    /// Print the whole row as JSON.
    #[arg(long)]
    details: bool,
    /// Read each line as one text record.
    #[arg(long, conflicts_with = "jsonl")]
    lines: bool,
    /// Read each line as one JSON record.
    #[arg(long, group="structured", conflicts_with_all=["csv", "tsv", "excel"])]
    jsonl: bool,
    /// The JSON Pointer to each JSON record's evidence, such as /body.
    #[arg(long, requires = "structured", conflicts_with = "lines")]
    on: Option<String>,
    /// How many records share one request. Beats THINKTHEN_BATCH.
    #[arg(long)]
    batch: Option<String>,
}

const UNREAD: &str = "standard input could not be read";

const NOT_UTF8: &str = "standard input is not UTF-8 text";

fn main() -> ExitCode {
    let cancel = Cancel::default();
    let signal = cancel.clone();
    let handler = ctrlc::set_handler(move || {
        if signal.is_cancelled() {
            for cleanup in [
                thinkthen::runs::force_cleanup(),
                thinkthen::files::force_cleanup(),
            ] {
                if let Err(error) = cleanup {
                    report(&error);
                }
            }
            std::process::exit(130);
        }
        signal.cancel();
        let _ = writeln!(
            io::stderr().lock(),
            "thinkthen: cancelling; press Ctrl-C again to exit"
        );
    })
    .map_err(|_| local("the interrupt handler could not be installed"));
    let result = handler.and_then(|()| {
        let command = Cli::parse().command;
        if let Some(result) = table::dispatch(&command, &cancel) {
            return result;
        }
        match command {
            Command::Run(args) => saved_run::run(args, cancel.clone()),
            Command::Decide(args) => decide(args, cancel.clone()),
            Command::Find(args) => find(args, cancel.clone()),
            Command::Filter(args) => filter::run(args, cancel.clone()),
            Command::Choose(args) => choose(args, cancel.clone()),
            Command::Tag(args) => tag(args, cancel.clone()),
            Command::Score(args) => score(args, cancel.clone()),
            Command::Recognize(args) => recognize(args, cancel.clone()),
            Command::Relate(args) => relate::run(args, cancel.clone()),
            Command::Rank(args) => rank::run(args, cancel.clone()),
            Command::Annotate(args) => annotate::run(args, cancel.clone()),
        }
    });
    if cancel.is_cancelled() {
        return ExitCode::from(130);
    }
    match result {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            report(&error);
            ExitCode::from(family_code(error.kind().family()))
        }
    }
}

fn decide(args: DecideArgs, cancel: Cancel) -> Result<u8, Error> {
    let question = Decide::new(args.question)?;
    let question = match args.threshold {
        Some(text) => question.threshold(text.parse()?),
        None => question,
    };
    run(args.common, &question, cancel)
}
fn find(args: DecideArgs, cancel: Cancel) -> Result<u8, Error> {
    let question = Find::new(args.question)?;
    let question = match args.threshold {
        Some(text) => question.threshold(text.parse()?)?,
        None => question,
    };
    run(args.common, &question, cancel)
}
fn choose(args: ChooseArgs, cancel: Cancel) -> Result<u8, Error> {
    let options = args
        .options
        .into_iter()
        .map(ChoiceOption::new)
        .collect::<Result<Vec<_>, _>>()?;
    let question = Choose::new(args.question, options)?;
    let question = match args.threshold {
        Some(text) => question.threshold(text.parse()?),
        None => question,
    };
    run(args.common, &question, cancel)
}
fn tag(args: TagArgs, cancel: Cancel) -> Result<u8, Error> {
    let question = Tag::new(args.question, args.labels)?;
    let question = match args.threshold {
        Some(text) => question.threshold(text.parse()?)?,
        None => question,
    };
    run(args.common, &question, cancel)
}
fn score(args: ScoreArgs, cancel: Cancel) -> Result<u8, Error> {
    let question = Score::new(args.question, args.levels)?;
    run(args.common, &question, cancel)
}
fn recognize(args: RecognizeArgs, cancel: Cancel) -> Result<u8, Error> {
    let question = Recognize::new(args.kinds)?;
    let question = match args.threshold {
        Some(text) => question.threshold(text.parse()?)?,
        None => question,
    };
    run(args.common, &question, cancel)
}

fn run(args: CommonArgs, question: &impl Function, cancel: Cancel) -> Result<u8, Error> {
    let (engine, on) = settings(&args, cancel)?;
    if args.lines {
        return Ok(stream(&engine, question, Record::text));
    }
    if args.jsonl {
        return Ok(stream(&engine, question, |line| Record::json(&line, &on)));
    }
    let mut unread = None;
    let text = input(InputMode::Single, &mut unread)
        .next()
        .unwrap_or_default();
    if let Some(error) = unread {
        return Err(error);
    }
    let mut code = 0;
    let mut failure = None;
    let result = engine.call(question, [Record::text(text)], |row| {
        code = row_code(&row);
        failure = print(&row, args.details).err();
    });
    if let Err(error) = result {
        code = family_code(error.kind().family());
    }
    failure.map_or(Ok(code), Err)
}

/// Resolve the shared input and backend settings once at the command edge.
fn settings(args: &CommonArgs, cancel: Cancel) -> Result<(Engine, Pointer), Error> {
    let on = match args.on.as_deref() {
        Some(text) => text.parse::<Pointer>()?,
        None => Pointer::default(),
    };
    let variable = |name: &str| std::env::var(name).ok();
    let batch = Batch::resolve(args.batch.as_deref(), variable)?;
    let backend = Backend::resolve_named(
        &args.adapter,
        args.url.as_deref(),
        args.model.as_deref(),
        variable,
    )?;
    let engine = Engine::new(backend).batch(batch).cancel(cancel);

    Ok((engine, on))
}

/// Answer each line of standard input as one record and print one JSON row per
/// record. The exit code is 0 when no row failed, and otherwise the family of
/// the first failed row.
fn stream(engine: &Engine, question: &impl Function, record: impl Fn(String) -> Record) -> u8 {
    let mut unread = None;
    let mut annotation_failure = None;
    let mut unwritten = None;
    // Once a row cannot be written, no more records are read, so only the
    // requests already open go out.
    let writing = Cell::new(true);
    let records = lines(&mut unread).take_while(|_| writing.get()).map(record);
    let result = engine.call(question, records, |row| {
        if matches!(
            row.outcome(),
            Outcome::Answered {
                answer: Answer::Annotations(_),
                ..
            }
        ) && let Some(error) = row.outcome().failure()
        {
            annotation_failure.get_or_insert(error.kind().family());
        }
        if unwritten.is_none() {
            unwritten = print(&row, true).err();
            writing.set(unwritten.is_none());
        }
    });
    let mut first_failed = result
        .err()
        .map(|error| error.kind().family())
        .or(annotation_failure);
    if let Some(error) = unread.or(unwritten) {
        report(&error);
        first_failed.get_or_insert(error.kind().family());
    }
    first_failed.map_or(0, family_code)
}

/// Each nonblank line of standard input, without its line ending.
fn lines(unread: &mut Option<Error>) -> impl Iterator<Item = String> + '_ {
    input(InputMode::Lines, unread).map(|line| evidence(&line).to_owned())
}

#[derive(Clone, Copy)]
enum InputMode {
    Single,
    Lines,
}

/// Read original record bytes as UTF-8. A single text includes empty input;
/// a stream skips blank lines and stops at its first read or encoding failure.
fn input(mode: InputMode, unread: &mut Option<Error>) -> impl Iterator<Item = String> + '_ {
    let mut input = io::stdin().lock();
    let mut finished = false;
    std::iter::from_fn(move || {
        if finished {
            return None;
        }
        let mut bytes = Vec::new();
        let result = match mode {
            InputMode::Lines => input.read_until(b'\n', &mut bytes),
            InputMode::Single => {
                finished = true;
                input.read_to_end(&mut bytes)
            }
        };
        match result {
            Ok(0) if matches!(mode, InputMode::Lines) => None,
            Ok(_) => match String::from_utf8(bytes) {
                Ok(text) => Some(text),
                Err(_) => {
                    finished = true;
                    *unread = Some(local(NOT_UTF8));
                    None
                }
            },
            Err(_) => {
                finished = true;
                *unread = Some(local(UNREAD));
                None
            }
        }
    })
    .filter(move |text| matches!(mode, InputMode::Single) || !text.trim().is_empty())
}

/// A text line's evidence excludes its line ending, while its original keeps it.
fn evidence(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

/// A local failure that says `message`.
fn local(message: &str) -> Error {
    Error::new(ErrorKind::Io, message)
}

#[expect(
    clippy::expect_used,
    reason = "a row of strings, numbers and booleans always serializes"
)]
fn print(row: &Row, details: bool) -> Result<(), Error> {
    let line = if details {
        serde_json::to_string(row).expect("a row serializes")
    } else {
        match row.outcome() {
            Outcome::Answered {
                answer: Answer::Found(Some(found)),
                ..
            } => found.text.clone(),
            Outcome::Answered {
                answer: Answer::Found(None),
                ..
            } => return Ok(()),
            Outcome::Answered {
                answer: answer @ (Answer::Bool(_) | Answer::Position(_)),
                ..
            } => serde_json::to_string(answer).expect("an integer or boolean answer serializes"),
            Outcome::Answered {
                answer: Answer::Label(label),
                ..
            } => label.clone(),
            Outcome::Answered {
                answer: Answer::Labels(labels),
                ..
            } => {
                if labels.is_empty() {
                    return Ok(());
                }
                labels.join("\n")
            }
            Outcome::Answered {
                answer: Answer::Names(names),
                ..
            } => {
                if names.is_empty() {
                    return Ok(());
                }
                names
                    .iter()
                    .map(|name| serde_json::to_string(name).expect("a name serializes"))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            Outcome::Answered {
                answer: Answer::Relations(relations),
                ..
            } => {
                if relations.is_empty() {
                    return Ok(());
                }
                relations
                    .iter()
                    .map(|relation| serde_json::to_string(relation).expect("a relation serializes"))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            Outcome::Answered {
                answer: Answer::Number(number),
                ..
            } => number.to_string(),
            Outcome::Answered {
                answer: Answer::Annotations(_),
                ..
            } => serde_json::to_string(row).expect("a row serializes"),
            Outcome::Unsure { .. } => "unsure".to_owned(),
            Outcome::Failed { error } => {
                report(error);
                return Ok(());
            }
        }
    };
    writeln!(io::stdout().lock(), "{line}")
        .map_err(|_| local("standard output could not be written"))
}

fn report(error: &Error) {
    // Nothing is left to tell when standard error itself fails.
    let _ = writeln!(
        io::stderr().lock(),
        "thinkthen: {}: {error}",
        error.kind().code()
    );
}

fn row_code(row: &Row) -> u8 {
    match row.outcome() {
        Outcome::Answered {
            answer: Answer::Bool(true),
            ..
        } => 0,
        Outcome::Answered {
            answer: Answer::Bool(false),
            ..
        } => 1,
        Outcome::Answered {
            answer: Answer::Found(None),
            ..
        } => 1,
        Outcome::Answered { .. } => row
            .outcome()
            .failure()
            .map_or(0, |error| family_code(error.kind().family())),
        Outcome::Unsure { .. } => 3,
        Outcome::Failed { error } => family_code(error.kind().family()),
    }
}

const fn family_code(family: Family) -> u8 {
    match family {
        Family::Usage => 2,
        Family::Backend => 4,
        Family::Local => 5,
    }
}
