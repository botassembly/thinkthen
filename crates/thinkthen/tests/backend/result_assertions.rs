//! Shared assertions over detailed result metadata.

use std::io;
use std::process::Output;

pub(crate) fn normalized_details(output: &Output) -> io::Result<(String, bool, u64)> {
    let mut details = std::str::from_utf8(&output.stdout)
        .map_err(|_| io::Error::other("details are not UTF-8"))?
        .to_owned();
    let meta = details
        .find(r#""meta":{"#)
        .ok_or_else(|| io::Error::other("details carry no meta"))?;
    let sent_marker = r#""requests_sent":"#;
    let sent_start = details
        .find(sent_marker)
        .ok_or_else(|| io::Error::other("details carry no requests_sent field"))?;
    if sent_start <= meta {
        return Err(io::Error::other("requests_sent does not belong to meta"));
    }
    let sent_value = sent_start + sent_marker.len();
    let sent_end = details[sent_value..]
        .find(',')
        .map(|place| sent_value + place)
        .ok_or_else(|| io::Error::other("requests_sent has no following field"))?;
    let requests_sent = details[sent_value..sent_end]
        .parse()
        .map_err(|_| io::Error::other("requests_sent is not an integer"))?;
    details.replace_range(sent_value..sent_end, "<requests_sent>");

    let marker = r#""replayed":"#;
    if details.matches(marker).count() != 1 {
        return Err(io::Error::other("details do not carry one replayed field"));
    }
    let start = details
        .find(marker)
        .ok_or_else(|| io::Error::other("details carry no replayed field"))?;
    if start <= meta {
        return Err(io::Error::other("replayed does not belong to meta"));
    }
    let value = start + marker.len();
    let (replayed, end) = if details[value..].starts_with("true") {
        (true, value + 4)
    } else if details[value..].starts_with("false") {
        (false, value + 5)
    } else {
        return Err(io::Error::other("replayed is not a boolean"));
    };
    details.replace_range(value..end, "<replayed>");
    Ok((details, replayed, requests_sent))
}
