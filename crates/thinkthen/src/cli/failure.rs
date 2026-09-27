//! Every way the command fails, mapped once to what the user is told.

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use crate::core::adapters::built_in::DecodeError;
use crate::core::{
    BackendError, PointerError, ProfileError, ProfileLimit, QuestionFileError, QuestionSetError,
    ReadingError, RecordError, RenderError, Source,
};

use crate::engine::error::{TransportKind, reply_too_large};
use crate::table;

mod convert;
pub(crate) mod recognize;
mod recording;
pub(crate) mod relate;
mod status;

const NOT_TEXT: &str = "the evidence is not valid UTF-8";

/// What a usage error with no sentence of its own would be told.
const UNNAMED: &str = "defect: a usage error with no sentence";

/// What stopped the command.
#[derive(Debug)]
pub(crate) enum Failure {
    /// The flags and the environment name no backend.
    Backend(BackendError),
    OpenProfile {
        path: PathBuf,
        error: io::Error,
    },
    Profile {
        path: PathBuf,
        error: ProfileError,
    },
    ProfileLimit(ProfileLimit),
    Relate(relate::Error),
    Recognize(recognize::Error),
    QuietWithDetails,
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
    /// `--jobs` differs from the width this process already selected.
    WidthActive(crate::engine::WidthActive),
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
    InvalidUtf8 {
        record: bool,
    },
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
        /// Whether this run named a recording, replay, or cache folder.
        recording: bool,
        /// True when the run held every row back and printed none.
        held: bool,
        /// What stopped the record, which sets the exit code.
        cause: Box<Failure>,
    },
    /// The request for records `at` to `last` failed as a whole.
    BatchFailed {
        last: usize,
        cause: Box<Failure>,
    },
    /// The reply for records `first` to `last` gave the stopped record no usable answer.
    PartialReply {
        first: usize,
        last: usize,
    },
    Cancelled,
    /// Standard input could not be read.
    Input(io::Error),
    /// Standard output could not be written.
    Output(io::Error),
    /// The key variable the backend names holds nothing.
    NoKey(String),
    /// The backend could not be reached at all.
    Transport(TransportKind),
    /// The backend answered with a status that is not a success.
    Status(u16),
    /// The backend answered status 400 and named `max_tokens_exceeded`.
    TokenLimit,
    /// The reply passed its request's limit of this many bytes and was not kept.
    ReplyTooLarge(u64),
    /// The adapter refused what the backend answered.
    Reply(DecodeError),
    /// The two recording options named two different folders.
    TwoFolders,
    /// A plan sends nothing, so it has nothing to record or to replay.
    DryRunWithRecording,
    /// `--cache` was given beside one of the two options it stands for.
    CacheWithRecording,
    JobsOutsideRecords,
    FindCount {
        none: bool,
    },
    FindTooLarge,
    TopIsZero,
    QuietOverKept(&'static str),
    RawOverKept(&'static str),
    ReplayMiss(String),
    Entry(String, String),
    RecordingConflict(String),
    RecordingStorage,
    RecordingPathIsFile,
    /// This run's endpoint URL, and whether the folder is the default cache.
    RecordingBackendMismatch(String, bool),
    RecordingFolderLegacy,
    DefaultCacheUnavailable,
    DefaultCachePrivate,
    Configuration(&'static str),
    CacheEntry,
    StatusState,
    Defect(&'static str),
    /// A measuring command refused its inputs or options.
    Measure(crate::cli::measure::Refusal),
    /// A document `thinkthen` built could not be written as JSON.
    Render(RenderError),
}

/// Every message a user reads is written here. No message carries a key or any
/// evidence text. A backend that answers with an error status is named by that
/// status and by the fixed phrase its status carries, never by its body,
/// because a server may quote the evidence back in one.
pub(crate) fn report(failure: &Failure, mut writer: impl Write) -> ExitCode {
    ExitCode::from(say(failure, &mut writer))
}

/// A stopped run reports its cause and then says where it stopped, so this
/// calls itself once. The writer is a trait object, because a generic call
/// into itself has no end.
fn say(failure: &Failure, writer: &mut dyn Write) -> u8 {
    if matches!(failure, Failure::Cancelled) {
        return 130;
    }
    if let Some(code) = stopped(failure, writer) {
        return code;
    }
    if let Some((code, message)) = recording::message(failure) {
        let _unwritten = writeln!(writer, "{}: {message}", crate::core::NAME);
        return code;
    }
    if let Some((code, message)) = recognize::message(failure) {
        let _unwritten = writeln!(writer, "{}: {message}", crate::core::NAME);
        return code;
    }
    if let Some((code, message)) = special_failure(failure) {
        let _unwritten = writeln!(writer, "{}: {message}", crate::core::NAME);
        return code;
    }
    if let Failure::Table(error) = failure {
        let code = if error.input_failure() { 5 } else { 2 };
        let _unwritten = writeln!(writer, "{}: {error}", crate::core::NAME);
        return code;
    }
    let (code, message): (u8, String) = match failure {
        Failure::Backend(error) => (2, error.to_string()),
        Failure::Question(error) => (
            match error.origin() {
                // A file value is local failure 5. A value the user typed
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
        Failure::Transport(kind) => (4, transport_message(*kind).to_owned()),
        Failure::Status(status) => (4, status::said(*status)),
        Failure::Reply(error) => (4, format!("the reply was refused: {error}")),
        Failure::ReplayMiss(_)
        | Failure::Entry(_, _)
        | Failure::RecordingConflict(_)
        | Failure::RecordingStorage
        | Failure::RecordingPathIsFile
        | Failure::RecordingBackendMismatch(..)
        | Failure::RecordingFolderLegacy => (
            70,
            "defect: a recording failure was not reported".to_owned(),
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
        Failure::FindCount { none: true } => (2, "`find --none` takes 2 to 254 units".to_owned()),
        Failure::FindCount { none: false } => (2, "`find` takes 2 to 255 units".to_owned()),
        Failure::FindTooLarge => (2, "`find` reads at most 16 MiB across all units".to_owned()),
        Failure::Stopped { .. } | Failure::BatchFailed { .. } | Failure::PartialReply { .. } => {
            (70, "defect: a stopped run reports its cause".to_owned())
        }
        Failure::Defect(what) => (70, format!("defect: {what}")),
        Failure::Render(error) => (70, format!("defect: {error}")),
        Failure::Measure(refusal) => (refusal.code(), refusal.to_string()),
        other => (2, refused(other).unwrap_or(UNNAMED).to_owned()),
    };
    // A diagnostic that cannot be written changes neither the failure nor its code.
    let _unwritten = writeln!(writer, "{}: {message}", crate::core::NAME);
    code
}

fn stopped(failure: &Failure, writer: &mut dyn Write) -> Option<u8> {
    let Failure::Stopped {
        at,
        finished,
        replayed,
        recording,
        held,
        cause,
    } = failure
    else {
        return None;
    };
    let (code, reason) = match cause.as_ref() {
        Failure::BatchFailed { last, cause } => {
            let mut said = Vec::new();
            let code = say(cause, &mut said);
            let said = String::from_utf8_lossy(&said);
            let prefix = format!("{}: ", crate::core::NAME);
            let said = said.trim_end();
            let said = said.strip_prefix(&prefix).unwrap_or(said);
            (code, format!("the request for records {at} to {last} failed: {said}; "))
        }
        Failure::PartialReply { first, last } => (
            4,
            format!("the reply for records {first} to {last} gave record {at} no usable answer; "),
        ),
        Failure::RecordingStorage => return Some(say(cause, writer)),
        _ => (say(cause, writer), String::new()),
    };
    let withheld = if *held {
        ", and nothing was printed because an order needs every record"
    } else {
        ""
    };
    let finished_noun = if *finished == 1 { "record" } else { "records" };
    let recording_clause = if *recording {
        let replayed_noun = if *replayed == 1 { "record" } else { "records" };
        format!(", {replayed} {replayed_noun} from a recording")
    } else {
        String::new()
    };
    let _unwritten = writeln!(
        writer,
        "{}: stopped at record {at}; {reason}{finished} {finished_noun} finished{recording_clause}{withheld}",
        crate::core::NAME
    );
    Some(code)
}

fn special_failure(failure: &Failure) -> Option<(u8, String)> {
    if let Some(message) = relate::message(failure) {
        return Some(message);
    }
    Some(match failure {
        Failure::TokenLimit => (4, status::TOKEN_LIMIT.to_owned()),
        Failure::ReplyTooLarge(limit) => (4, reply_too_large(*limit)),
        Failure::OpenProfile { path, error } => (
            5,
            format!(
                "the profile file `{}` could not be opened: {error}",
                path.display()
            ),
        ),
        Failure::Profile { path, error } => {
            (5, format!("the profile file `{}` {error}", path.display()))
        }
        Failure::ProfileLimit(limit) => (
            2,
            format!(
                "profile {} allows at most {} {}; this request has {}",
                limit.name.as_str(),
                limit.limit,
                limit.kind.words(),
                limit.actual
            ),
        ),
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
            format!("the replies for one record named model versions `{first}` and `{second}`; a cache or recording folder may hold answers from the other version, so rerun with --no-cache or prune it with `thinkthen cache prune DIR --answered-by-other-than VERSION`, naming the version a --no-cache run returns"),
        ),
        Failure::ModelsDiffer(None) => (
            4,
            "the replies for one record named different model versions; a cache or recording folder may hold answers from the other version, so rerun with --no-cache or prune it with thinkthen cache prune DIR --answered-by-other-than VERSION, naming the version a --no-cache run returns".to_owned(),
        ),
        Failure::WidthActive(active) => (2, active.to_string()),
        Failure::UsageOverflow => (
            4,
            "the backend reported token counts whose total is too large".to_owned(),
        ),
        Failure::InputDirectory => (
            5,
            "`--input` names a directory, and a directory is not an input file".to_owned(),
        ),
        Failure::DefaultCacheUnavailable => (
            5,
            "no default cache folder is available; set THINKTHEN_CACHE or use --no-cache".to_owned(),
        ),
        Failure::DefaultCachePrivate => (
            5,
            "the default cache folder is not private; set its permissions to 0700 or use --no-cache"
                .to_owned(),
        ),
        Failure::Configuration(message) => (5, (*message).to_owned()),
        Failure::CacheEntry => (5, "the cache contains a malformed final entry".to_owned()),
        Failure::StatusState => (
            5,
            "status could not read the local cache or usage state; check its permissions and contents"
                .to_owned(),
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
            "--jobs bounds the requests in flight, and a single text sends one request"
        }
        Failure::TopIsZero => {
            "`--top` prints the first N of the order, and N is a whole number of 1 or more"
        }
        Failure::Usage(message) => message,
        _ => return None,
    })
}

const fn transport_message(kind: TransportKind) -> &'static str {
    match kind {
        TransportKind::Timeout => "the backend timed out; increase --timeout or try again",
        TransportKind::NameLookup => {
            "the backend's host could not be found; check --url and the network"
        }
        TransportKind::Refused => {
            "the backend refused the connection; check that it is running and that --url is correct"
        }
        TransportKind::PrematureClose => {
            "the backend closed the connection before a reply and may have received the request; it was not sent again"
        }
        TransportKind::Other => "the backend could not be reached; check --url and the network",
    }
}

#[cfg(test)]
mod tests;
