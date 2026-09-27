//! A command interrupted by a signal keeps completed counts and local failures.

use std::io::Write;

use super::{Failure, recognize};

impl Failure {
    /// Replace only a backend cause with the signal's stop.
    pub(crate) fn after_signal(self) -> Self {
        match self {
            Self::Stopped {
                at,
                finished,
                replayed,
                recording,
                held,
                cause,
            } => Self::Stopped {
                at,
                finished,
                replayed,
                recording,
                held,
                cause: Box::new(cause.after_signal()),
            },
            Self::BatchFailed { last, cause } => {
                let cause = cause.after_signal();
                if matches!(cause, Self::Cancelled) {
                    Self::Cancelled
                } else {
                    Self::BatchFailed {
                        last,
                        cause: Box::new(cause),
                    }
                }
            }
            Self::Transport(_)
            | Self::Status(_)
            | Self::TokenLimit
            | Self::ReplyTooLarge(_)
            | Self::Reply(_)
            | Self::PartialReply { .. }
            | Self::Recognize(recognize::Error::LogicalQuestion) => Self::Cancelled,
            other => other,
        }
    }
}

/// A signal stop names finished records, never a record that may not exist.
pub(super) fn stopped(
    finished: usize,
    replayed: usize,
    recording: bool,
    held: bool,
    writer: &mut dyn Write,
) -> u8 {
    let noun = if finished == 1 { "record" } else { "records" };
    let recording_clause = if recording {
        let noun = if replayed == 1 { "record" } else { "records" };
        format!(", {replayed} {noun} from a recording")
    } else {
        String::new()
    };
    let withheld = if held {
        ", and nothing was printed because an order needs every record"
    } else {
        ""
    };
    let _unwritten = writeln!(
        writer,
        "{}: stopped by a signal; {finished} {noun} finished{recording_clause}{withheld}",
        crate::core::NAME
    );
    130
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::adapters::built_in::DecodeError;
    use crate::engine::error::TransportKind;

    fn backend() -> Vec<Failure> {
        vec![
            Failure::Transport(TransportKind::Timeout),
            Failure::Status(503),
            Failure::TokenLimit,
            Failure::ReplyTooLarge(12),
            Failure::Reply(DecodeError::NoModel),
            Failure::PartialReply { first: 1, last: 2 },
            Failure::Recognize(recognize::Error::LogicalQuestion),
            Failure::BatchFailed {
                last: 2,
                cause: Box::new(Failure::Status(503)),
            },
        ]
    }

    #[test]
    fn a_signal_replaces_only_backend_causes_and_keeps_stop_counts() {
        for cause in backend() {
            assert!(matches!(cause.after_signal(), Failure::Cancelled));
        }
        for cause in backend() {
            let stopped = Failure::Stopped {
                at: 7,
                finished: 2,
                replayed: 1,
                recording: true,
                held: false,
                cause: Box::new(cause),
            };
            assert!(matches!(
                stopped.after_signal(),
                Failure::Stopped { at: 7, finished: 2, replayed: 1, cause, .. }
                    if matches!(*cause, Failure::Cancelled)
            ));
        }
        let local = || {
            [
                Failure::RecordingStorage,
                Failure::Input(std::io::Error::other("input")),
                Failure::Defect("defect"),
            ]
        };
        for local in local() {
            assert!(!matches!(local.after_signal(), Failure::Cancelled));
        }
        for local in local() {
            assert!(matches!(
                Failure::Stopped {
                    at: 3,
                    finished: 2,
                    replayed: 0,
                    recording: false,
                    held: false,
                    cause: Box::new(local),
                }
                .after_signal(),
                Failure::Stopped { at: 3, cause, .. } if !matches!(*cause, Failure::Cancelled)
            ));
        }
    }
}
