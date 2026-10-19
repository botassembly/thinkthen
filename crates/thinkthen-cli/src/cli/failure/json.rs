//! Fixed command sentences from closed public JSON classifications.

use thinkthen::JsonFailure;

pub(super) fn record(failure: JsonFailure) -> String {
    match failure {
        JsonFailure::DuplicateName => "a JSON record holds each member name once, and one name arrived twice".to_owned(),
        JsonFailure::NotFinite => "a JSON number is finite, so `NaN` and `Infinity` are refused".to_owned(),
        JsonFailure::TooDeep => "the JSON nests more than 127 levels of arrays and objects, the most this tool reads".to_owned(),
        JsonFailure::Syntax { .. } => "the record is not valid JSON".to_owned(),
    }
}

pub(super) fn question(failure: JsonFailure) -> String {
    match failure {
        JsonFailure::Syntax { line, column } => format!("the question file is not valid JSON: the JSON at line {line} column {column} is not one"),
        other => format!("the question file is not JSON this tool reads: {}", record(other)),
    }
}
