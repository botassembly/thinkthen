//! Render the native whole-set selection beside host originals and display positions.
use super::{Rendered, Unit};
use crate::args::Common;
use crate::cli::intake;
use crate::core::{Reading, json_line};
use crate::failure::Failure;

pub(super) fn rendered(
    common: &Common,
    reading: &Reading,
    units: &[Unit],
    found: &crate::CompleteFound<crate::QuestionInput>,
    declarations: &crate::core::declaration::QuestionMetadata,
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
        Some(details(found, declarations)?)
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

// The CLI presents authored reading metadata; route overrides remain in native meta.
fn details(
    found: &crate::CompleteFound<crate::QuestionInput>,
    declarations: &crate::core::declaration::QuestionMetadata,
) -> Result<String, Failure> {
    let presentation = found.canonical.display_question(declarations);
    let question = crate::core::Json::parse(&json_line(&presentation)?)
        .map_err(|_| Failure::Defect("a native find question could not be displayed"))?;
    let mut document = crate::core::Json::parse(&found.to_json().map_err(Failure::from)?)
        .map_err(|_| Failure::Defect("a native find result could not be displayed"))?;
    let crate::core::Json::Object(members) = &mut document else {
        return Err(Failure::Defect("a native find result is not an object"));
    };
    let (_, displayed) = members
        .iter_mut()
        .find(|(name, _)| name == "question")
        .ok_or(Failure::Defect("a native find result has no question"))?;
    *displayed = question;
    json_line(&document).map_err(Failure::from)
}
