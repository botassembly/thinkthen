//! Every way the command fails, with the one map to an exit code and a message.

use std::io::{self, Write};
use std::process::ExitCode;

use thinkthen_core::systemone::DecodeError;
use thinkthen_core::{BackendError, BlankTextError, PassMarkError, RenderError};

/// What stopped the command.
#[derive(Debug)]
pub(crate) enum Failure {
    /// The flags and the environment name no backend.
    Backend(BackendError),
    /// The condition or the evidence arrived blank.
    Blank(BlankTextError),
    /// The pass mark is not a pass mark.
    Mark(PassMarkError),
    /// Standard input could not be read.
    Input(io::Error),
    /// Standard input held bytes that are not text.
    NotUtf8,
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
    /// An invariant inside `thinkthen` broke.
    Defect(&'static str),
    /// A document `thinkthen` built could not be written as JSON.
    Render(RenderError),
}

/// Say what failed and give the exit code `specification/channels.md` fixes.
///
/// Every message a user reads is written here. No message carries a key or any
/// evidence text, and a backend that answers with an error status is named by
/// that status alone, because a server may quote the evidence back in its body.
pub(crate) fn report(failure: &Failure, mut writer: impl Write) -> ExitCode {
    let (code, message): (u8, String) = match failure {
        Failure::Backend(error) => (2, error.to_string()),
        Failure::Blank(error) => (2, error.to_string()),
        Failure::Mark(error) => (2, format!("--min-prob: {error}")),
        Failure::NoKey(variable) => (
            4,
            format!("the environment variable `{variable}` is unset or blank, so no key is sent"),
        ),
        Failure::Transport(what) => (4, format!("the backend could not be reached: {what}")),
        Failure::Status(status) => (4, format!("the backend answered with status {status}")),
        Failure::Reply(error) => (4, format!("the reply was refused: {error}")),
        Failure::Input(error) => (5, format!("standard input could not be read: {error}")),
        Failure::NotUtf8 => (5, "the evidence is not valid UTF-8".to_owned()),
        Failure::Output(error) => (5, format!("standard output could not be written: {error}")),
        Failure::Defect(what) => (70, format!("defect: {what}")),
        Failure::Render(error) => (70, format!("defect: {error}")),
    };
    // A diagnostic that cannot be written changes neither the failure nor its code.
    let _unwritten = writeln!(writer, "{}: {message}", thinkthen_core::NAME);
    ExitCode::from(code)
}

impl From<BackendError> for Failure {
    fn from(error: BackendError) -> Self {
        Self::Backend(error)
    }
}

impl From<BlankTextError> for Failure {
    fn from(error: BlankTextError) -> Self {
        Self::Blank(error)
    }
}

impl From<PassMarkError> for Failure {
    fn from(error: PassMarkError) -> Self {
        Self::Mark(error)
    }
}

impl From<DecodeError> for Failure {
    fn from(error: DecodeError) -> Self {
        Self::Reply(error)
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

    #[test]
    fn every_failure_reaches_its_own_exit_code_and_says_what_stopped() {
        let cases = [
            (Failure::NotUtf8, 5, "not valid UTF-8"),
            (Failure::Status(503), 4, "status 503"),
            (Failure::NoKey("TYPESAFE_API_KEY".to_owned()), 4, "unset"),
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
