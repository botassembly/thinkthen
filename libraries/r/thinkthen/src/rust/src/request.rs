//! Serialize actual native outcomes, retaining completed prefixes and final facts.
use crate::calls::Crossed;
use extendr_api::prelude::*;
pub(crate) fn attach(column: &thinkthen::RequestColumn, value: Robj) -> Crossed<Robj> {
    let list =
        List::try_from(value).map_err(|_| crate::defect("native presentation is not an R list"))?;
    let class = list.get_attrib("class");
    let mut fields = list.iter().collect::<Vec<_>>();
    fields.push((
        "positions",
        column
            .positions()
            .iter()
            .map(|at| *at as f64)
            .collect::<Vec<_>>()
            .into(),
    ));
    fields.push(("length", (column.len() as f64).into()));
    if !fields.iter().any(|(name, _)| *name == "probability") {
        fields.push(("probability", ().into()));
    }
    let mut value: Robj = List::from_pairs(fields).into();
    if let Some(class) = class {
        value
            .set_attrib("class", class)
            .map_err(|_| crate::defect("native presentation class could not be assigned"))?;
    }
    Ok(value)
}
pub(crate) fn view(
    value: Robj,
    view: Option<thinkthen::RequestColumnView>,
    column: Option<&thinkthen::RequestColumn>,
    scalar: bool,
) -> Crossed<Robj> {
    use thinkthen::RequestColumnValues;
    let Some(view) = view else {
        return Ok(value);
    };
    let list =
        List::try_from(value).map_err(|_| crate::defect("native presentation is not an R list"))?;
    let class = list.get_attrib("class");
    let mut fields = list
        .iter()
        .filter(|(name, _)| !["value", "probability"].contains(name))
        .collect::<Vec<_>>();
    let values: Robj = match view.values {
        RequestColumnValues::Decisions(_) => column_value(&list, column, scalar, false)?,
        RequestColumnValues::Choices(_) => column_value(&list, column, scalar, true)?,
        RequestColumnValues::Scores(values) => values
            .into_iter()
            .map(|value| value.map_or(Rfloat::na(), Rfloat::from))
            .collect::<Vec<_>>()
            .into(),
        RequestColumnValues::Tags(values) => {
            List::from_values(values.into_iter().map(Robj::from)).into()
        }
    };
    fields.push(("value", values));
    if let Some(values) = view.probability {
        let probability = list
            .iter()
            .find(|(name, _)| *name == "probability")
            .map(|(_, value)| value)
            .ok_or_else(|| crate::defect("native result has no probability"))?;
        let probabilities = if probability.rtype() == Rtype::List {
            List::try_from(probability)
                .map_err(|_| crate::defect("native probability is not a list"))?
        } else {
            List::from_values([probability])
        };
        let column = column.ok_or_else(|| crate::defect("native presentation has no column"))?;
        let mut aligned = vec![Rfloat::na(); values.len()];
        for (row, position) in column.positions().iter().enumerate() {
            if let Some(slot) = aligned.get_mut(*position) {
                *slot = probabilities
                    .elt(row)
                    .ok()
                    .and_then(|value| value.as_real())
                    .map_or(Rfloat::na(), Rfloat::from);
            }
        }
        fields.push(("probability", aligned.into()));
    }
    if !fields.iter().any(|(name, _)| *name == "probability") {
        fields.push(("probability", ().into()));
    }
    let mut value: Robj = List::from_pairs(fields).into();
    if let Some(class) = class {
        value
            .set_attrib("class", class)
            .map_err(|_| crate::defect("native presentation class could not be assigned"))?;
    }
    Ok(value)
}

fn column_value(
    call: &List,
    column: Option<&thinkthen::RequestColumn>,
    scalar: bool,
    choices: bool,
) -> Crossed<Robj> {
    let column = column.ok_or_else(|| crate::defect("native presentation has no column"))?;
    let value = call
        .iter()
        .find(|(name, _)| *name == "value")
        .map(|(_, value)| value)
        .ok_or_else(|| crate::defect("native result has no value"))?;
    let probability = call
        .iter()
        .find(|(name, _)| *name == "probability")
        .map(|(_, value)| value)
        .ok_or_else(|| crate::defect("native result has no probability"))?;
    let rows = if scalar {
        List::from_values([value])
    } else {
        List::try_from(value).map_err(|_| crate::defect("native column values are not a list"))?
    };
    let probabilities = if probability.rtype() == Rtype::List {
        List::try_from(probability)
            .map_err(|_| crate::defect("native probabilities are not a list"))?
    } else {
        List::from_values([probability])
    };
    let ordinary = rows.iter().enumerate().all(|(at, (_, value))| {
        (if choices {
            value.as_str().is_some()
        } else {
            value.as_bool().is_some()
        }) || (value.is_null() && probabilities.elt(at).is_ok_and(|value| value.is_null()))
    });
    let missing: Robj = if choices {
        Rstr::na().into()
    } else {
        Rbool::na().into()
    };
    let mut aligned = vec![missing; column.len()];
    for ((_, value), position) in rows.iter().zip(column.positions()) {
        let slot = aligned
            .get_mut(*position)
            .ok_or_else(|| crate::defect("native column position is out of range"))?;
        *slot = value;
    }
    if ordinary && choices {
        return Ok(aligned
            .iter()
            .map(|value| value.as_str().map_or(Rstr::na(), Rstr::from))
            .collect::<Vec<_>>()
            .into());
    }
    if ordinary {
        return Ok(aligned
            .iter()
            .map(|value| value.as_bool().map_or(Rbool::na(), Rbool::from))
            .collect::<Vec<_>>()
            .into());
    }
    if scalar {
        return aligned
            .into_iter()
            .next()
            .ok_or_else(|| crate::defect("native scalar has no value"));
    }
    Ok(List::from_values(aligned).into())
}
