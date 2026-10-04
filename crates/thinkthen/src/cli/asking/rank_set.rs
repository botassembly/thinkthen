//! Decode each set member with its own receipt through the ordinary rank view.
use super::{
    Asks,
    judged::{Held, JudgeAsker},
};
use crate::core::{Json, json_line, pack};
use crate::engine::pipeline::Answered;
use crate::failure::Failure;
use crate::schedule::Judged;

pub(super) fn rows(
    asker: &JudgeAsker<'_>,
    held: Held,
    answers: &[Answered],
) -> Result<Vec<Judged>, Failure> {
    let questions = asker.judging.asks.questions(&held.record)?;
    let mut rest = answers;
    let mut rows = Vec::with_capacity(questions.len());
    for (place, question) in questions.into_iter().enumerate() {
        let (own, after) = rest
            .split_at_checked(pack::wire_count(&question))
            .ok_or(Failure::Defect("a rank member lost its answers"))?;
        rest = after;
        let mut row = asker.one(&held, question, own)?;
        if asker.judging.view.details
            && let Asks::Set(set) = &asker.judging.asks
        {
            let name = set
                .questions()
                .get(place)
                .ok_or(Failure::Defect("a rank member lost its name"))?
                .name();
            attribute(&mut row.printed, name)?;
        }
        rows.push(row);
    }
    Ok(rows)
}

fn attribute(printed: &mut Option<String>, name: &str) -> Result<(), Failure> {
    let Some(printed) = printed else {
        return Ok(());
    };
    let mut value =
        Json::parse(printed).map_err(|_| Failure::Defect("a rank detail is not JSON"))?;
    let Json::Object(fields) = &mut value else {
        return Err(Failure::Defect("a rank detail is not an object"));
    };
    fields.push(("question_name".to_owned(), Json::String(name.to_owned())));
    *printed = json_line(&value)?;
    Ok(())
}
