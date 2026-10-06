//! Record-stop diagnostics keep the failed request's range and cause.

use std::io::Write;

use super::{Failure, after_signal, say};
use crate::core::NAME;

pub(super) fn report(failure: &Failure, writer: &mut dyn Write) -> Option<u8> {
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
    if matches!(cause.as_ref(), Failure::Cancelled) {
        return Some(after_signal::stopped(
            *finished, *replayed, *recording, *held, writer,
        ));
    }
    let (code, reason) = match cause.as_ref() {
        Failure::BatchFailed { last, cause } => {
            let mut written = Vec::new();
            let code = say(cause, &mut written);
            let written = String::from_utf8_lossy(&written);
            let said = written.trim_end();
            let prefix = format!("{NAME}: ");
            let said = said.strip_prefix(&prefix).unwrap_or(said);
            (
                code,
                format!("the request for records {at} to {last} failed: {said}; "),
            )
        }
        Failure::PartialReply { first, last } => (
            4,
            format!("the reply for records {first} to {last} gave record {at} no usable answer; "),
        ),
        Failure::RecordingStorage | Failure::RecordingForbidden => return Some(say(cause, writer)),
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
        "{NAME}: stopped at record {at}; {reason}{finished} {finished_noun} finished{recording_clause}{withheld}"
    );
    Some(code)
}
