//! Every way the command fails, mapped once to what the user is told.

use std::io::{self, Write};
use std::process::ExitCode;

use thinkthen_core::adapters::built_in::DecodeError;
use thinkthen_core::{
    BackendError, PointerError, QuestionFileError, ReadingError, RecordError, RenderError, Source,
};

/// The phrase `specification/backends.md` fixes for each common failure status.
///
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

/// What a run that read bytes which are not text is told.
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
    /// `--option` was given beside a list of options on the command line.
    OptionWithList,
    /// An `--option` entry carries no `=`, so it names no description.
    OptionWithoutSign,
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
    /// The file the records were to be read from could not be opened.
    OpenInput(io::Error),
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
    /// The recording folder could not be read or written.
    Recording(io::Error),
    /// An invariant inside `thinkthen` broke.
    Defect(&'static str),
    /// A document `thinkthen` built could not be written as JSON.
    Render(RenderError),
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
    if let Failure::Stopped {
        at,
        finished,
        replayed,
        held,
        cause,
    } = failure
    {
        let code = say(cause, writer);
        // The line names the record by its number and never by its content,
        // because a record is evidence and a diagnostic is read by a person.
        let withheld = if *held {
            ", and nothing was printed because an order needs every record"
        } else {
            ""
        };
        let _unwritten = writeln!(
            writer,
            "{}: stopped at record {at}; {finished} records finished, {replayed} from a recording{withheld}",
            thinkthen_core::NAME
        );
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
            format!("`{verb}` maps over a stream, so it takes --lines or --jsonl"),
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
        Failure::OptionWithoutSign => "--option is LABEL=DESCRIPTION, and this one holds no `=`",
        Failure::OptionsOutsideJsonl => {
            "--options needs --jsonl, because a pointer needs a JSON record to point into"
        }
        Failure::QuietOverRecords => {
            "--quiet carries the answer in the exit code, and no record's answer sets it"
        }
        Failure::JobsOutsideRecords => {
            "--jobs bounds the requests in flight, and one document sends one request"
        }
        Failure::TopIsZero => "--top prints the first N of the order, and N is 1 or more",
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
mod tests {
    use super::{Failure, report};
    use std::process::ExitCode;
    use std::time::Duration;
    use thinkthen_core::recording::{Entry, Exchange as Recorded};
    use thinkthen_core::{RecordError, Url};

    /// The key and the evidence every case here is built from.
    ///
    /// They are the same two markers `crates/thinkthen/tests/backend/secrecy.rs`
    /// sweeps the compiled binary with. That sweep reads what a run writes, and
    /// this one reads the `Debug` lines a message could be built from.
    const KEY: &str = "sk-marker-2f9d41c6";
    const EVIDENCE: &str = "marker-evidence-7b3ac5";

    /// Every `Debug` line that could hold the key or the evidence holds neither.
    ///
    /// A `{:?}` is how a key reaches a log by accident, so every type that
    /// carries one is rendered here and read. A new type that holds either goes
    /// in this list.
    #[test]
    fn no_debug_line_shows_the_key_or_the_evidence() {
        let key = crate::edge::Key::of(KEY);
        let body = format!(r#"{{"state":"{EVIDENCE}"}}"#);
        let url = Url::new("http://127.0.0.1:1/v1/systemone").expect("an address");
        let recorded = Recorded::new(&url, body.as_bytes());
        let entry = Entry::of(&recorded, body.as_bytes()).expect("both bodies are JSON");
        let exchange = crate::http::Exchange {
            url: url.as_str(),
            body: body.as_bytes(),
            key: &key,
            max_retries: 2,
            retry_wait: Duration::from_secs(1),
        };
        let judged = crate::schedule::Judged {
            printed: Some(body.clone()),
            outcome: thinkthen_core::Outcome::Yes,
            probability: Some(0.91),
            replayed: false,
        };
        let client = crate::http::Client::new(Duration::from_secs(1), false);

        let shown = format!(
            "{key:?} {exchange:?} {judged:?} {client:?} {recorded:?} {entry:?} \
             {:?} {:?}",
            Failure::NoKey("THINKTHEN_API_KEY".to_owned()),
            Failure::Status(401),
        );

        assert!(!shown.contains(KEY), "{shown}");
        assert!(!shown.contains(EVIDENCE), "{shown}");
        assert!(shown.contains("withheld"), "{shown}");
    }

    /// No message a user reads holds the key or the evidence.
    ///
    /// Every variant is reported, so a variant added later that quotes either
    /// one fails here.
    #[test]
    fn no_diagnostic_holds_the_key_or_the_evidence() {
        let body = format!(r#"{{"state":"{EVIDENCE}"}}"#);
        let cases = [
            Failure::NoKey("THINKTHEN_API_KEY".to_owned()),
            Failure::Status(401),
            Failure::Transport("connection refused".to_owned()),
            Failure::Record(RecordError::NotUtf8),
            Failure::Record(RecordError::TooLarge),
            Failure::ReplayMiss("abc.json".to_owned()),
            Failure::Entry("abc.json".to_owned(), "it records another".to_owned()),
            Failure::Stopped {
                at: 2,
                finished: 1,
                replayed: 0,
                held: false,
                cause: Box::new(Failure::Record(RecordError::TooLarge)),
            },
            Failure::Stopped {
                at: 2,
                finished: 1,
                replayed: 0,
                held: true,
                cause: Box::new(Failure::Record(RecordError::TooLarge)),
            },
            Failure::QuietOverKept("filter"),
            Failure::RawOverKept("rank"),
            Failure::NoFraming("filter"),
            Failure::TopIsZero,
        ];

        for failure in cases {
            let mut written = Vec::new();
            report(&failure, &mut written);
            let said = String::from_utf8(written).expect("a diagnostic is text");

            assert!(!said.contains(KEY), "{said}");
            assert!(!said.contains(EVIDENCE), "{said}");
            assert!(!said.contains(&body), "{said}");
        }
    }

    #[test]
    fn a_common_failure_status_carries_the_phrase_the_specification_fixes() {
        let cases = [
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

        for (status, phrase) in cases {
            let mut written = Vec::new();
            report(&Failure::Status(status), &mut written);
            let message = String::from_utf8(written).expect("a diagnostic is text");
            assert_eq!(
                message,
                format!("thinkthen: the backend answered with status {status}: {phrase}\n")
            );
        }

        let mut written = Vec::new();
        report(&Failure::Status(418), &mut written);
        let message = String::from_utf8(written).expect("a diagnostic is text");
        assert_eq!(message, "thinkthen: the backend answered with status 418\n");
    }

    #[test]
    fn every_failure_reaches_its_own_exit_code_and_says_what_stopped() {
        let cases = [
            (Failure::Record(RecordError::NotUtf8), 5, "not valid UTF-8"),
            (Failure::Status(503), 4, "status 503"),
            (Failure::NoKey("THINKTHEN_API_KEY".to_owned()), 4, "unset"),
            (Failure::Defect("a plan asks nothing"), 70, "defect"),
        ];

        for (failure, code, said) in cases {
            let mut written = Vec::new();
            let exit = report(&failure, &mut written);
            let message = String::from_utf8(written).expect("a diagnostic is text");

            assert_eq!(format!("{exit:?}"), format!("{:?}", ExitCode::from(code)));
            assert!(message.starts_with("thinkthen: "), "{message}");
            assert!(message.contains(said), "{message}");
        }
    }
}
