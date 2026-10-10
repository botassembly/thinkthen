//! Serialize actual native outcomes, retaining completed prefixes and final facts.
use crate::calls::Crossed;
use extendr_api::prelude::*;
use serde::Serialize;
use thinkthen::{CompleteError, CompleteFacts, RequestOutcome, RequestValue};

#[derive(Serialize)]
struct Packet<'a> {
    results: &'a RequestValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    facts: Option<CompleteFacts<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<CompleteError<'a>>,
}

pub(crate) fn written(outcome: &RequestOutcome) -> Crossed<String> {
    let packet = match outcome {
        RequestOutcome::Complete(call) => Packet {
            results: call.value(),
            facts: call.facts().complete(),
            failure: None,
        },
        RequestOutcome::Failed { completed, error } => Packet {
            results: completed,
            facts: error.facts().and_then(thinkthen::Facts::complete),
            failure: Some(error.complete()),
        },
    };
    serde_json::to_string(&packet)
        .map_err(|_| crate::defect("native request outcome could not be written"))
}

// Nullable host columns retain presentation positions beside native results.
#[derive(Clone, Debug)]
pub(crate) struct ColumnMap {
    pub(crate) positions: Vec<f64>,
    pub(crate) length: f64,
}

impl ColumnMap {
    pub(crate) fn from_request(request: &Robj) -> Option<Self> {
        let positions = request.get_attrib("positions")?.as_real_vector()?;
        let length = request.get_attrib("length")?.as_real()?;
        Some(Self { positions, length })
    }

    pub(crate) fn observation_positions(&self) -> Vec<usize> {
        // Native column conversion created these nonnegative indexes from R's length.
        self.positions
            .iter()
            .map(|position| {
                #[expect(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    reason = "positions originate from native R column indexes"
                )]
                let index = *position as usize;
                index
            })
            .collect()
    }

    pub(crate) fn attach(&self, value: Robj) -> Crossed<Robj> {
        let list = List::try_from(value)
            .map_err(|_| crate::defect("native presentation is not an R list"))?;
        let class = list.get_attrib("class");
        let mut fields = list.iter().collect::<Vec<_>>();
        fields.push(("positions", self.positions.clone().into()));
        fields.push(("length", self.length.into()));
        let mut value: Robj = List::from_pairs(fields).into();
        if let Some(class) = class {
            value
                .set_attrib("class", class)
                .map_err(|_| crate::defect("native presentation class could not be assigned"))?;
        }
        Ok(value)
    }
}

pub(crate) fn column(values: Robj, function: &str) -> Crossed<Robj> {
    let values =
        List::try_from(values).map_err(|_| crate::usage("an ordinary column is required"))?;
    let mut present = Vec::new();
    let mut positions = Vec::new();
    for (at, (_, value)) in values.iter().enumerate() {
        let missing = value.is_na()
            || value.as_real_slice().is_some_and(|numbers| {
                numbers.len() == 1 && numbers.first().is_some_and(|n| n.is_nan())
            });
        if missing {
            if !matches!(
                function,
                "decide" | "choose" | "tag" | "score" | "recognize"
            ) {
                return Err(crate::usage(&format!("{function} takes no NA records")));
            }
        } else {
            positions.push(at as f64);
            present.push(value);
        }
    }
    Ok(list!(
        values = List::from_values(present),
        positions = positions,
        length = values.len() as f64
    )
    .into())
}
