//! Saved vendor replies for the opt-in original-image admission fixtures.
use crate::{Canned, Recorded};
pub(crate) fn reply(vendor: &str, role: &str) -> Canned {
    let body = match (vendor, role) {
        ("liquid", "decide") => {
            include_str!("../../../specification/fixtures/images/liquid-decide-reply.json")
        }
        ("liquid", "choose") => {
            include_str!("../../../specification/fixtures/images/liquid-choose-reply.json")
        }
        ("liquid", "score") => {
            include_str!("../../../specification/fixtures/images/liquid-score-reply.json")
        }
        ("perplexity", "decide") => {
            include_str!("../../../specification/fixtures/images/perplexity-decide-reply.json")
        }
        ("perplexity", "choose") => {
            include_str!("../../../specification/fixtures/images/perplexity-choose-reply.json")
        }
        ("perplexity", "score") => {
            include_str!("../../../specification/fixtures/images/perplexity-score-reply.json")
        }
        _ => return crate::arms::drift("unknown saved image reply"),
    };
    Canned::ok(body)
}

/// Image-only fixtures opt in to the larger bounded body capture.
pub(crate) fn role(request: &Recorded) -> Option<(&str, &str)> {
    let path = request.line.split(' ').nth(1)?;
    let mut parts = path.trim_start_matches('/').split('/');
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (
            Some("arm"),
            Some("images"),
            Some(vendor @ ("liquid" | "perplexity")),
            Some(role @ ("decide" | "choose" | "score")),
        ) => Some((vendor, role)),
        _ => None,
    }
}
