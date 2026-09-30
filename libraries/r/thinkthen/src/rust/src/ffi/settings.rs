//! Shared core settings grammar for R question and call keywords.

use extendr_api::prelude::*;

use super::text_of;
use crate::calls::Crossed;
use crate::usage;

pub(super) fn check(body: &Robj, kind: &Robj) -> Crossed<()> {
    let parsed = thinkthen::Settings::parse(&text_of(body, "settings")?)
        .map_err(|error| usage(&error.to_string()))?;
    let verb = match text_of(kind, "question kind")?.as_str() {
        "decide" => thinkthen::For::Decide,
        "choose" => thinkthen::For::Choose,
        "score" => thinkthen::For::Score,
        "tag" => thinkthen::For::Tag,
        _ => return Err(usage("the question kind is invalid")),
    };
    // The complete question is checked from raw JSON. Decide has no required
    // members, so its key dispositions can be checked immediately.
    if verb == thinkthen::For::Decide {
        parsed
            .check(verb)
            .map_err(|error| usage(&error.to_string()))?;
    }
    Ok(())
}
