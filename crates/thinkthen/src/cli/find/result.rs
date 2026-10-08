//! Render whole-set selections with every original candidate and location.
use super::{Rendered, Unit};
use crate::args::Common;
use crate::asking;
use crate::cli::intake;
use crate::core::{Find, Reading, Record, json_line};
use crate::engine::facade::{self, Found};
use crate::failure::Failure;
use serde::{Serialize, Serializer};

pub(super) fn rendered(
    common: &Common,
    find: &Find,
    engine: &facade::Engine,
    (reading, context): (&Reading, Option<&str>),
    units: &[Unit],
    (found, declarations, question): (
        Found,
        crate::core::declaration::QuestionMetadata,
        crate::core::Question,
    ),
) -> Result<Rendered, Failure> {
    let selected = &found.selection;
    let place = selected.selected();
    let unit = place
        .map(|place| {
            units
                .get(place)
                .ok_or(Failure::Defect("a find selection is outside its units"))
        })
        .transpose()?;
    let position = unit.map(|unit| unit.position.clone());
    let score = place
        .map(|place| {
            selected
                .probabilities()
                .get(place)
                .map(|(_, probability)| *probability)
                .ok_or(Failure::Defect("a find selection carries no probability"))
        })
        .transpose()?;
    let mut line = if common.details {
        Some(complete(
            find,
            engine,
            units,
            (found, declarations, question),
            context,
            unit,
        )?)
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

fn complete(
    find: &Find,
    engine: &facade::Engine,
    units: &[Unit],
    (found, declarations, question): (
        Found,
        crate::core::declaration::QuestionMetadata,
        crate::core::Question,
    ),
    context: Option<&str>,
    unit: Option<&Unit>,
) -> Result<String, Failure> {
    let value = unit.map(|unit| unit.record.clone());
    let candidates = units
        .iter()
        .map(|unit| unit.evidence.as_text().map(|text| text.into_owned()))
        .collect::<Result<Vec<_>, _>>()?;
    let attempts = Some(found.answered.attempts.clone());
    let probabilities = found.selection.probabilities().to_vec();
    let canonical = crate::result_json::complete::find(
        engine,
        find,
        found,
        crate::result_json::complete::FindRow {
            declarations,
            question,
            candidates,
            input: value,
            context_sha256: asking::context::digest(context),
            attempts,
        },
    )
    .map_err(|_| Failure::Defect("a complete find result could not be constructed"))?;
    let sources: Vec<_> = units
        .iter()
        .map(|unit| {
            unit.position
                .file
                .as_ref()
                .map(|file| crate::core::CompletePhysicalSource {
                    file: file.clone(),
                    first_line: unit.position.first,
                    last_line: unit.position.last,
                })
        })
        .collect();
    let candidates = units
        .iter()
        .zip(&probabilities)
        .zip(&sources)
        .enumerate()
        .map(
            |(index, ((unit, (_, probability)), source))| crate::core::CompleteFindCandidate {
                index: Some(index),
                input: Some(&unit.record),
                probability: *probability,
                source: source.as_ref(),
            },
        )
        .collect();
    json_line(&Candidates {
        canonical: &canonical,
        selected: unit.map(|unit| &unit.record),
        candidates,
    })
    .map_err(Failure::from)
}

struct Candidates<'a> {
    canonical: &'a crate::core::CompleteFind,
    selected: Option<&'a Record>,
    candidates: Vec<crate::core::CompleteFindCandidate<'a, Record>>,
}
impl Serialize for Candidates<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical
            .serialize_candidates(self.selected, Some(&self.candidates), serializer)
    }
}
