//! One actual find question's borrowed detail at the public caller boundary.

use crate::core;
use crate::engine::facade;
use crate::public::error::Error;
use crate::public::options::Stop;
use crate::public::question::Question;
use crate::public::results::{ObservedQuestion, ObservedRow, QuestionDetail, RecordObservation};

pub(crate) fn observe_find(
    stop: &Stop<'_>,
    engine: &facade::Engine,
    question: &Question,
    find: &core::Find,
    found: &facade::Found,
) -> Result<(), Error> {
    if !stop.observing() {
        return Ok(());
    }
    let planned = find
        .plan()
        .questions()
        .first()
        .ok_or_else(|| Error::defect("find planned no question"))?;
    let answer = found
        .answered
        .reply
        .outcomes()
        .first()
        .ok_or_else(|| Error::defect("find returned no question"))?;
    let mut detail = ObservedQuestion::from_reply(
        planned,
        None,
        question.profile.as_ref(),
        engine.backend(),
        answer,
        &found.answered.reply,
        found.answered.request.as_str(),
        found.answered.requests_sent,
        found.answered.replayed,
        1,
        0,
    )?;
    detail.question_sha256 = find
        .question_sha256()
        .map_err(|_| Error::defect("a find digest could not be written"))?;
    stop.observe(RecordObservation::Question {
        index: 0,
        member: None,
        stage: None,
        position: 0,
        detail: QuestionDetail::of(&detail),
    });
    stop.observe(RecordObservation::Row {
        index: 0,
        value: ObservedRow::Find(found.selection.selected()),
    });
    Ok(())
}
