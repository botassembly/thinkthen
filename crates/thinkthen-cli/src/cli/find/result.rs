//! Render the native whole-set selection beside host originals and display positions.
use super::{Rendered, Unit};
use crate::args::Common;
use crate::cli::intake;
use crate::cli::reading::Reading;
use crate::failure::Failure;
use crate::render::json_line;

pub(super) fn rendered(
    common: &Common,
    reading: &Reading,
    units: &[Unit],
    found: &crate::CompleteFound<crate::QuestionInput>,
    authored_model: Option<&str>,
) -> Result<Rendered, Failure> {
    let place = match found.selection() {
        crate::FindSelection::Unit(at) => Some(at),
        crate::FindSelection::None => None,
    };
    let unit = place
        .map(|at| {
            units
                .get(at)
                .ok_or(Failure::Defect("a find selection is outside its units"))
        })
        .transpose()?;
    let position = unit.map(|unit| unit.position.clone());
    let score = place
        .map(|at| {
            found
                .candidates()
                .get(at)
                .map(crate::Candidate::probability)
                .ok_or(Failure::Defect("a find selection carries no probability"))
        })
        .transpose()?;
    let mut line = if common.details {
        Some(details(found, authored_model)?)
    } else {
        unit.map(|unit| {
            reading
                .as_it_arrived(&unit.bytes)
                .map(str::to_owned)
                .map_err(|error| Failure::record(error, true))
        })
        .transpose()?
    };
    if common.details {
        intake::locate(&mut line, position.as_ref())?;
        intake::source_members(&mut line, position.as_ref())?;
    } else if let (Some(unit), Some(position), Some(line)) =
        (unit, position.as_ref().filter(|p| p.located), &mut line)
    {
        *line = intake::source_value(&unit.record, &json_line(&unit.record)?, position)?;
    }
    Ok(Rendered {
        line,
        resolved: place.is_some(),
        position,
        score,
    })
}

// The command displays the authored model beside the actual native answer.
// Route/model overrides remain in native metadata; native identities and values
// pass through unchanged. The native canonical question already owns every
// semantic field and declaration, so only this presentation label is adjusted.
fn details(
    found: &crate::CompleteFound<crate::QuestionInput>,
    authored_model: Option<&str>,
) -> Result<String, Failure> {
    let document = crate::saved_json::Json::parse(&found.to_json()?)?;
    let crate::saved_json::Kind::Object(mut members) = document.kind().clone() else {
        return Err(Failure::Defect("a native find result is not an object"));
    };
    let question = members
        .iter_mut()
        .find(|(name, _)| name == "question")
        .ok_or(Failure::Defect("a native find result has no question"))?;
    let crate::saved_json::Kind::Object(mut fields) = question.1.kind().clone() else {
        return Err(Failure::Defect("a native find question is not an object"));
    };
    if let Some((_, model)) = fields.iter_mut().find(|(name, _)| name == "model") {
        if let Some(authored) = authored_model {
            *model =
                crate::saved_json::Json::made(crate::saved_json::Kind::String(authored.to_owned()));
        } else {
            fields.retain(|(name, _)| name != "model");
        }
    }
    question.1 = crate::saved_json::Json::made(crate::saved_json::Kind::Object(fields));
    json_line(&crate::saved_json::Json::made(
        crate::saved_json::Kind::Object(members),
    ))
    .map_err(Failure::from)
}
