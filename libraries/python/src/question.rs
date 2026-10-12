//! Convert Python question and label representations into shared native constructors.
use crate::{inputs, raised, usage};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use thinkthen::{Description, RequestQuestion};
#[pyclass(frozen, name = "_RequestQuestion", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Question(pub(crate) RequestQuestion);
fn field(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<Description> {
    Description::from_json(&inputs::json(py, value)?).map_err(|e| raised(py, &e))
}
#[pyfunction]
pub(crate) fn _build_question(
    py: Python<'_>,
    verb: &str,
    value: &Bound<'_, PyAny>,
    controls: &Bound<'_, PyDict>,
    frame: bool,
) -> PyResult<Question> {
    let function: thinkthen::RequestFunction = verb.parse().map_err(|e| raised(py, &e))?;
    if controls
        .get_item("on")?
        .is_some_and(|value| !value.is_none())
    {
        if function == thinkthen::RequestFunction::Annotate && !frame {
            return Err(usage(
                py,
                "annotate with on= takes a Polars or pandas DataFrame; a list of str takes no on=",
            ));
        }
        if function == thinkthen::RequestFunction::Recognize
            && controls
                .get_item("relations")?
                .is_some_and(|value| !value.is_none())
        {
            return Err(usage(
                py,
                "recognize with on= takes no relations; ask them of one text",
            ));
        }
    }
    let selected = py.import("thinkthen._inputs")?.getattr("QuestionSource")?;
    if value.is_instance(&selected)? {
        if value.getattr("none")?.extract::<bool>()? {
            controls.set_item("none", true)?;
        }
        let pair = PyTuple::new(
            py,
            ["selector".into_pyobject(py)?.into_any(), value.clone()],
        )?;
        return inputs::question(py, pair.as_any()).map(Question);
    }
    let recognition = value.is_none()
        && matches!(
            function,
            thinkthen::RequestFunction::Recognize | thinkthen::RequestFunction::Relate
        );
    if value.is_instance(&py.import("os")?.getattr("PathLike")?)? {
        let path = py
            .import("os")?
            .getattr("fspath")?
            .call1((value,))?
            .extract()?;
        return Ok(Question(RequestQuestion::File { path }));
    }
    if value.is_instance(&py.import("collections.abc")?.getattr("Mapping")?)? {
        return thinkthen::RequestDefinition::from_request_json(&inputs::json(py, value)?)
            .map(|value| Question(RequestQuestion::Definition { value }))
            .map_err(|e| raised(py, &e));
    }
    let mut fields = Vec::new();
    if recognition {
        relation_fields(py, function, controls, &mut fields)?;
    }
    for name in if recognition {
        &["kinds", "instructions", "entity_definition"][..]
    } else {
        &["options", "labels", "levels", "true", "false"][..]
    } {
        if let Some(value) = controls.get_item(name)? {
            fields.push(((*name).to_owned(), field(py, &value)?));
            controls.del_item(name)?;
        }
    }
    if recognition {
        return RequestQuestion::inline(function, None, fields)
            .map(Question)
            .map_err(|e| raised(py, &e));
    }
    if matches!(
        function,
        thinkthen::RequestFunction::Annotate
            | thinkthen::RequestFunction::Recognize
            | thinkthen::RequestFunction::Relate
    ) {
        return Ok(Question(RequestQuestion::File {
            path: value
                .extract()
                .map_err(|_| usage(py, "invalid canonical request"))?,
        }));
    }
    let text = value
        .extract()
        .map_err(|_| usage(py, "invalid canonical request"))?;
    RequestQuestion::inline(function, Some(text), fields)
        .map(Question)
        .map_err(|e| raised(py, &e))
}
#[pyfunction]
pub(crate) fn _label_names(
    py: Python<'_>,
    family: &str,
    names: &Bound<'_, PyAny>,
) -> PyResult<Vec<String>> {
    let names: Vec<String> = names
        .extract()
        .map_err(|_| usage(py, "Literal labels are strings"))?;
    RequestQuestion::labels(names.clone(), Vec::new(), false).map_err(|_| {
        usage(
            py,
            match family {
                "Enum" => "Enum labels must not have aliases",
                "EnumValues" => "Enum labels must be distinct",
                _ => "Literal labels must be distinct",
            },
        )
    })?;
    Ok(names)
}
#[pyfunction]
pub(crate) fn _label_values(
    py: Python<'_>,
    names: Vec<String>,
    meanings: &Bound<'_, PyDict>,
    descriptions: &Bound<'_, PyAny>,
    bare_levels: bool,
) -> PyResult<Py<PyAny>> {
    let meanings = meanings.copy()?;
    if !descriptions.is_none() {
        let descriptions = descriptions
            .cast::<PyDict>()
            .map_err(|_| usage(py, "descriptions= maps selected labels to meanings"))?;
        meanings.update(descriptions.as_mapping())?;
    }
    let descriptions = meanings
        .iter()
        .map(|(name, value)| {
            Ok((
                name.extract()
                    .map_err(|_| usage(py, "descriptions= names an unknown label"))?,
                field(py, &value)?,
            ))
        })
        .collect::<PyResult<_>>()?;
    let value =
        RequestQuestion::labels(names, descriptions, bare_levels).map_err(|e| raised(py, &e))?;
    Ok(py
        .import("json")?
        .getattr("loads")?
        .call1((value.as_json(),))?
        .unbind())
}

fn relation_fields(
    py: Python<'_>,
    function: thinkthen::RequestFunction,
    controls: &Bound<'_, PyDict>,
    fields: &mut Vec<(String, Description)>,
) -> PyResult<()> {
    if controls.contains("relations")? || controls.contains("either")? {
        let relations = controls
            .get_item("relations")?
            .unwrap_or_else(|| py.None().into_bound(py));
        let either = controls
            .get_item("either")?
            .unwrap_or_else(|| py.None().into_bound(py));
        let rules = py
            .import("thinkthen")?
            .getattr("_rules")?
            .call1((&relations, &either))?;
        let values = pyo3::types::PyList::empty(py);
        for rule in rules.try_iter()? {
            let (name, source, target, either): (String, String, String, bool) = rule?.extract()?;
            let value = PyDict::new(py);
            value.set_item("name", name)?;
            value.set_item("source", source)?;
            value.set_item("target", target)?;
            value.set_item("either", either)?;
            values.append(value)?;
        }
        if values.len() > 0 || function == thinkthen::RequestFunction::Relate {
            fields.push(("relations".into(), field(py, values.as_any())?));
        }
        for name in ["relations", "either"] {
            if controls.contains(name)? {
                controls.del_item(name)?;
            }
        }
    }
    Ok(())
}
