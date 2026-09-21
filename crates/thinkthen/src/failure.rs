//! Every way the command fails, mapped once to what the user is told.

use std::io::{self, Write};
use std::process::ExitCode;

use thinkthen_core::adapters::built_in::DecodeError;
use thinkthen_core::{
    BackendError, PointerError, QuestionFileError, QuestionSetError, ReadingError, RecordError,
    RenderError, Source,
};

use crate::table;

/// The phrases are fixed text written here. A backend can quote the evidence
/// back in an error body, so nothing a backend sent ever reaches a message.
const PHRASES: [(u16, &str); 6] = [
    (401, "the key was refused"),
    (402, "the account has no credit"),
    (403, "the key may not use this model or address"),
    (404, "nothing answers at this address"),
    (
        422,
        "the backend refused the request as malformed or too large",
    ),
    (429, "the backend's rate limit was reached"),
];

const NOT_TEXT: &str = "the evidence is not valid UTF-8";

/// What a usage error with no sentence of its own would be told.
const UNNAMED: &str = "defect: a usage error with no sentence";

/// What stopped the command.
#[derive(Debug)]
pub(crate) enum Failure {
    /// The flags and the environment name no backend.
    Backend(BackendError),
    /// Two views of one answer were asked for at once.
    QuietWithDetails,
    /// A bare label was asked for beside another view of the same answer.
    RawWithAnotherView,
    /// The question file, or a value beside it, was refused.
    Question(QuestionFileError),
    /// The file the question was to be read from could not be opened.
    OpenQuestionFile(io::Error),
    /// The question set was refused.
    QuestionSet(QuestionSetError),
    /// The question-set file could not be opened.
    OpenQuestionSet(io::Error),
    /// One input object already holds a question name.
    AnnotationCollision(String),
    /// Replies for one record named different model versions.
    ModelsDiffer(Option<(String, String)>),
    /// Reply token counts cannot be represented as one total.
    UsageOverflow,
    /// A command-line shape was understood but cannot act.
    Usage(&'static str),
    /// `--option` was given beside a list of options on the command line.
    OptionWithList,
    /// `--label` was given beside positional labels.
    LabelWithList,
    /// An `--option` entry carries no `=`, so it names no description.
    OptionWithoutSign,
    /// A `--label` entry carries no `=`.
    LabelWithoutSign,
    /// `--raw` was given to `tag`, whose bare value is already JSON.
    TagRaw,
    /// `--raw` was given with a table, whose rows always become JSONL.
    TableRaw,
    /// `--quiet` was given to `tag`, whose list has no exit-code spelling.
    TagQuiet,
    /// The framing and the pointers cannot act together.
    Reading(ReadingError),
    /// A pointer is not a JSON Pointer, and the message names the one typed.
    Pointer(&'static str, String, PointerError),
    /// `--options` was given beside a list of options on the command line.
    OptionsWithList,
    /// `--options` was given where no record holds a pointer to follow.
    OptionsOutsideJsonl,
    /// An answer was asked to reach the exit code where no answer can.
    QuietOverRecords,
    /// One record could not become the evidence of one request.
    Record(RecordError),
    /// A CSV or TSV header or record broke its table rule.
    Table(table::Error),
    /// The file the records were to be read from could not be opened.
    OpenInput(io::Error),
    /// `--input` named a directory rather than a file.
    InputDirectory,
    /// Input bytes were not UTF-8; the noun depends on the framing.
    InvalidUtf8 { record: bool },
    /// A record command read a question file of the wrong kind.
    QuestionKind {
        command: &'static str,
        held: &'static str,
    },
    /// A record failed, and the run stopped there.
    Stopped {
        /// The record the run stopped at, counted from one.
        at: usize,
        /// How many records finished before it.
        finished: usize,
        /// How many of those a recording answered.
        replayed: usize,
        /// True when the run held every row back and printed none.
        held: bool,
        /// What stopped the record, which sets the exit code.
        cause: Box<Failure>,
    },
    /// Standard input could not be read.
    Input(io::Error),
    /// Standard output could not be written.
    Output(io::Error),
    /// The key variable the backend names holds nothing.
    NoKey(String),
    /// The backend could not be reached at all.
    Transport(String),
    /// The backend answered with a status that is not a success.
    Status(u16),
    /// The adapter refused what the backend answered.
    Reply(DecodeError),
    /// The two recording options named two different folders.
    TwoFolders,
    /// A plan sends nothing, so it has nothing to record or to replay.
    DryRunWithRecording,
    /// `--cache` was given beside one of the two options it stands for.
    CacheWithRecording,
    /// `--jobs` was given to a run that sends one request.
    JobsOutsideRecords,
    /// `--top` was asked for none of the order, which prints nothing.
    TopIsZero,
    /// `--quiet` was given to a verb whose answer is the records it prints.
    QuietOverKept(&'static str),
    /// `--raw` was given where no label is ever printed.
    RawOverKept(&'static str),
    /// A verb that maps over records was given no framing to read them by.
    NoFraming(&'static str),
    /// The replay folder holds no entry for the request being made.
    ReplayMiss(String),
    /// The entry the digest names cannot answer the request being made.
    Entry(String, String),
    /// The entry already keeps another response for this request.
    RecordingConflict(String),
    /// The recording folder could not be read or written.
    Recording(io::Error),
    /// An invariant inside `thinkthen` broke.
    Defect(&'static str),
    /// A document `thinkthen` built could not be written as JSON.
    Render(RenderError),
}

impl Failure {
    /// Name invalid text by its framing while preserving every other record error.
    pub(crate) fn record(error: RecordError, streamed: bool) -> Self {
        match error {
            RecordError::NotUtf8 => Self::InvalidUtf8 { record: streamed },
            other => Self::Record(other),
        }
    }
}

/// Say what failed and give the exit code `specification/channels.md` fixes.
///
/// Every message a user reads is written here. No message carries a key or any
/// evidence text. A backend that answers with an error status is named by that
/// status and by the fixed phrase its status carries, never by its body,
/// because a server may quote the evidence back in one.
pub(crate) fn report(failure: &Failure, mut writer: impl Write) -> ExitCode {
    ExitCode::from(say(failure, &mut writer))
}

/// Write the diagnostic and give back the exit code, over one boxed writer.
///
/// A stopped run reports its cause and then says where it stopped, so this
/// calls itself once. The writer is a trait object, because a generic call
/// into itself has no end.
fn say(failure: &Failure, writer: &mut dyn Write) -> u8 {
    if let Some(code) = stopped(failure, writer) {
        return code;
    }
    if let Some((code, message)) = special_failure(failure) {
        let _unwritten = writeln!(writer, "{}: {message}", thinkthen_core::NAME);
        return code;
    }
    if let Failure::Table(error) = failure {
        let code = if error.input_failure() { 5 } else { 2 };
        let _unwritten = writeln!(writer, "{}: {error}", thinkthen_core::NAME);
        return code;
    }
    let (code, message): (u8, String) = match failure {
        Failure::Backend(error) => (2, error.to_string()),
        Failure::Question(error) => (
            match error.origin() {
                // A value the file holds is a failure of a local file, which
                // `annotate.md` already puts at exit 5. A value the user typed
                // is a usage error, as it has always been.
                Source::File => 5,
                _ => 2,
            },
            error.to_string(),
        ),
        Failure::OpenQuestionFile(error) => {
            (5, format!("the question file could not be opened: {error}"))
        }
        Failure::Reading(error) => (2, error.to_string()),
        Failure::Pointer(option, typed, error) => (2, format!("{option} `{typed}`: {error}")),
        Failure::Record(RecordError::NotUtf8) => (5, NOT_TEXT.to_owned()),
        Failure::Record(RecordError::Render(error)) => (70, format!("defect: {error}")),
        Failure::Record(error) => (2, error.to_string()),
        Failure::Table(_) => (70, "defect: a table failure was not reported".to_owned()),
        Failure::NoKey(variable) => (
            4,
            format!("the environment variable `{variable}` is unset or blank, so no key is sent"),
        ),
        Failure::Transport(what) => (4, format!("the backend could not be reached: {what}")),
        Failure::Status(status) => (4, said(*status)),
        Failure::Reply(error) => (4, format!("the reply was refused: {error}")),
        Failure::ReplayMiss(name) => (
            5,
            format!("the replay folder holds no entry named `{name}`"),
        ),
        Failure::Entry(name, why) => (5, format!("the entry `{name}` was refused: {why}")),
        Failure::RecordingConflict(name) => (
            5,
            format!("the entry `{name}` already records a different response"),
        ),
        Failure::Recording(error) => (
            5,
            format!("the recording folder could not be read or written: {error}"),
        ),
        Failure::OpenInput(error) => (5, format!("--input could not be opened: {error}")),
        Failure::Input(error) => (5, format!("standard input could not be read: {error}")),
        Failure::Output(error) => (5, format!("standard output could not be written: {error}")),
        Failure::QuietOverKept(verb) => (
            2,
            format!(
                "--quiet prints nothing, and `{verb}` answers with the records it prints; \
                 `decide --quiet` carries one answer in the exit code"
            ),
        ),
        Failure::RawOverKept(verb) => (
            2,
            format!(
                "--raw prints a bare label, and `{verb}` prints records; \
                 `choose --raw` prints a label"
            ),
        ),
        Failure::NoFraming(verb) => (
            2,
            format!("`{verb}` maps over a stream, so it takes --lines, --jsonl, --csv, or --tsv"),
        ),
        Failure::Stopped { .. } => (70, "defect: a stopped run reports its cause".to_owned()),
        Failure::Defect(what) => (70, format!("defect: {what}")),
        Failure::Render(error) => (70, format!("defect: {error}")),
        other => (2, refused(other).unwrap_or(UNNAMED).to_owned()),
    };
    // A diagnostic that cannot be written changes neither the failure nor its code.
    let _unwritten = writeln!(writer, "{}: {message}", thinkthen_core::NAME);
    code
}

fn stopped(failure: &Failure, writer: &mut dyn Write) -> Option<u8> {
    let Failure::Stopped {
        at,
        finished,
        replayed,
        held,
        cause,
    } = failure
    else {
        return None;
    };
    let code = say(cause, writer);
    let withheld = if *held {
        ", and nothing was printed because an order needs every record"
    } else {
        ""
    };
    let finished_noun = if *finished == 1 { "record" } else { "records" };
    let replayed_noun = if *replayed == 1 { "record" } else { "records" };
    let _unwritten = writeln!(
        writer,
        "{}: stopped at record {at}; {finished} {finished_noun} finished, {replayed} {replayed_noun} from a recording{withheld}",
        thinkthen_core::NAME
    );
    Some(code)
}

fn special_failure(failure: &Failure) -> Option<(u8, String)> {
    Some(match failure {
        Failure::QuestionSet(error) => (5, error.to_string()),
        Failure::OpenQuestionSet(error) => {
            (5, format!("the question set could not be opened: {error}"))
        }
        Failure::AnnotationCollision(name) => (
            2,
            format!("the record already holds `{name}`, so that question cannot be appended"),
        ),
        Failure::ModelsDiffer(Some((first, second))) => (
            4,
            format!("the backend returned model versions `{first}` and `{second}` for one record; pin --model and rerun with --record or --cache"),
        ),
        Failure::ModelsDiffer(None) => (
            4,
            "the backend returned different model versions for one record; pin --model and rerun with --record or --cache".to_owned(),
        ),
        Failure::UsageOverflow => (
            4,
            "the backend reported token counts whose total is too large".to_owned(),
        ),
        Failure::InputDirectory => (
            5,
            "`--input` names a directory, and a directory is not an input file".to_owned(),
        ),
        Failure::InvalidUtf8 { record } => (
            5,
            if *record {
                "the record is not valid UTF-8"
            } else {
                NOT_TEXT
            }
            .to_owned(),
        ),
        Failure::QuestionKind { command, held } => (
            2,
            format!(
                "`{command}` reads a `decide` question, but the question file holds a `{held}` question"
            ),
        ),
        _ => return None,
    })
}

/// The fixed sentence each option clash is refused with, at exit code 2.
///
/// Every one of these is a command line no run can act on. The sentence names
/// the options and never a record, because the clash is in what was typed.
const fn refused(failure: &Failure) -> Option<&'static str> {
    Some(match failure {
        Failure::QuietWithDetails => "--quiet prints nothing, so it does not take --details",
        Failure::RawWithAnotherView => {
            "--raw prints a bare label, so it does not take --details or --quiet"
        }
        Failure::TwoFolders => {
            "--record and --replay name two different folders, and one run keeps one"
        }
        Failure::DryRunWithRecording => {
            "--dry-run sends nothing, so it takes neither --record nor --replay"
        }
        Failure::CacheWithRecording => {
            "--cache is --record and --replay on one folder, so it stands beside neither"
        }
        Failure::OptionsWithList => {
            "--options takes the options from each record, so the command line gives none"
        }
        Failure::OptionWithList => {
            "--option and a list of options have no order between them, so one run takes one"
        }
        Failure::LabelWithList => {
            "--label and positional labels have no order between them, so one run takes one form"
        }
        Failure::OptionWithoutSign => "--option is LABEL=DESCRIPTION, and this one holds no `=`",
        Failure::LabelWithoutSign => "--label is LABEL=DESCRIPTION, and this one holds no `=`",
        Failure::TagRaw => "`tag` prints one JSON array per record and takes no --raw",
        Failure::TableRaw => {
            "--raw prints a bare label, but table results stay JSONL; omit --raw or use --lines or --jsonl"
        }
        Failure::TagQuiet => {
            "`tag` returns a list, including an empty list, so no exit code can carry its answer and it takes no --quiet"
        }
        Failure::OptionsOutsideJsonl => {
            "--options needs --jsonl, because a pointer needs a JSON record to point into"
        }
        Failure::QuietOverRecords => {
            "--quiet carries the answer in the exit code, and no record's answer sets it"
        }
        Failure::JobsOutsideRecords => {
            "--jobs bounds the requests in flight, and one document sends one request"
        }
        Failure::TopIsZero => {
            "`--top` prints the first N of the order, and N is a whole number of 1 or more"
        }
        Failure::Usage(message) => message,
        _ => return None,
    })
}

/// Name the status the backend answered with, plus its fixed phrase when it has one.
fn said(status: u16) -> String {
    let answered = format!("the backend answered with status {status}");
    match PHRASES.iter().find(|(code, _)| *code == status) {
        Some((_, phrase)) => format!("{answered}: {phrase}"),
        None => answered,
    }
}

impl From<BackendError> for Failure {
    fn from(error: BackendError) -> Self {
        Self::Backend(error)
    }
}

impl From<QuestionFileError> for Failure {
    fn from(error: QuestionFileError) -> Self {
        Self::Question(error)
    }
}

impl From<QuestionSetError> for Failure {
    fn from(error: QuestionSetError) -> Self {
        Self::QuestionSet(error)
    }
}

impl From<DecodeError> for Failure {
    fn from(error: DecodeError) -> Self {
        Self::Reply(error)
    }
}

impl From<ReadingError> for Failure {
    fn from(error: ReadingError) -> Self {
        Self::Reading(error)
    }
}

impl From<RecordError> for Failure {
    fn from(error: RecordError) -> Self {
        Self::Record(error)
    }
}

impl From<RenderError> for Failure {
    fn from(error: RenderError) -> Self {
        Self::Render(error)
    }
}

#[cfg(test)]
mod tests;
