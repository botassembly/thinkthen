//! Python value conversion into native request arguments.
use crate::{raised, usage};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use thinkthen::{RequestArguments, RequestInput, RequestItem, RequestQuestion};

pub(crate) fn json(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<String> {
    let kwargs = PyDict::new(py);
    kwargs.set_item("ensure_ascii", false)?;
    kwargs.set_item("allow_nan", false)?;
    py.import("thinkthen._calls")?
        .getattr("_json")?
        .call1((value,))
        .and_then(|value| {
            py.import("json")?
                .getattr("dumps")?
                .call((value,), Some(&kwargs))?
                .extract()
        })
        .map_err(|_| usage(py, "input cannot be converted to native JSON"))
}
pub(crate) fn decode<T: serde::de::DeserializeOwned>(
    py: Python<'_>,
    value: &Bound<'_, PyAny>,
) -> PyResult<T> {
    serde_json::from_str(&json(py, value)?).map_err(|_| usage(py, "invalid canonical request"))
}
pub(crate) fn question(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<RequestQuestion> {
    if let Ok(value) = value.extract::<PyRef<'_, crate::question::Question>>() {
        return Ok(value.0.clone());
    }
    let (kind, body): (String, Py<PyAny>) = value.extract()?;
    let body = body.bind(py);
    if kind == "selector" {
        let mut selected = Vec::new();
        for (attribute, kind) in [
            ("path", "file"),
            ("name", "name"),
            ("reference", "reference"),
        ] {
            let value = body.getattr(attribute)?;
            if !value.is_none() {
                selected.push((kind, value));
            }
        }
        let raw = body.getattr("raw")?;
        let authored = body.getattr("body")?;
        if !raw.is_none() {
            selected.push(("raw", raw));
        } else if !authored.is_none() {
            selected.push(("definition", authored));
        }
        if selected.len() != 1 {
            return Err(usage(py, "invalid canonical request"));
        }
        let (kind, value) = selected
            .pop()
            .ok_or_else(|| usage(py, "invalid canonical request"))?;
        return question(
            py,
            pyo3::types::PyTuple::new(py, [kind.into_pyobject(py)?.into_any(), value])?.as_any(),
        );
    }
    Ok(match kind.as_str() {
        "text" => RequestQuestion::Text {
            text: body.extract()?,
        },
        "file" => RequestQuestion::File {
            path: body.extract()?,
        },
        "name" => RequestQuestion::Name {
            name: body.extract()?,
        },
        "reference" => RequestQuestion::Reference {
            reference: body.extract()?,
        },
        "definition" => RequestQuestion::Definition {
            value: thinkthen::RequestDefinition::from_request_json(&json(py, body)?)
                .map_err(|error| raised(py, &error))?,
        },
        "raw" => RequestQuestion::Definition {
            value: thinkthen::RequestDefinition::from_request_json(&body.extract::<String>()?)
                .map_err(|error| raised(py, &error))?,
        },
        _ => return Err(usage(py, "invalid question selector")),
    })
}
pub(crate) fn reading(
    py: Python<'_>,
    unit: &str,
    window: Option<usize>,
) -> PyResult<thinkthen::ReaderOptions> {
    let unit = match unit {
        "line" => thinkthen::SourceUnit::Line,
        "window" => thinkthen::SourceUnit::Window,
        "file" => thinkthen::SourceUnit::File,
        _ => {
            return Err(usage(
                py,
                "reader window requires unit window and a positive whole number",
            ));
        }
    };
    Ok(thinkthen::ReaderOptions { unit, window })
}
pub(crate) fn item(py: Python<'_>, value: &Bound<'_, PyAny>, verb: &str) -> PyResult<RequestItem> {
    let descriptor = py
        .import("thinkthen._calls")?
        .getattr("_item")?
        .call1((value, verb))?;
    RequestItem::from_json(&json(py, &descriptor)?).map_err(|e| raised(py, &e))
}
fn input(
    py: Python<'_>,
    value: &Bound<'_, PyAny>,
    verb: &str,
    feed: bool,
) -> PyResult<RequestInput> {
    if feed {
        return Ok(RequestInput::Feed {
            name: "python".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        });
    }
    let descriptor = py
        .import("thinkthen._calls")?
        .getattr("_input")?
        .call1((value, verb))?;
    RequestInput::from_json(&json(py, &descriptor)?).map_err(|e| raised(py, &e))
}
fn build_request(
    py: Python<'_>,
    verb: &str,
    asked: &Bound<'_, PyAny>,
    source: &Bound<'_, PyAny>,
    options: &Bound<'_, PyAny>,
    feed: bool,
) -> PyResult<thinkthen::Request> {
    let args = RequestArguments {
        question: question(py, asked)?,
        input: input(py, source, verb, feed)?,
        options: crate::controls::options(py, options)?,
    };
    let request = thinkthen::Request::named(verb, args).map_err(|e| raised(py, &e))?;
    Ok(request)
}

pub(crate) fn request(
    py: Python<'_>,
    verb: &str,
    asked: &Bound<'_, PyAny>,
    source: &Bound<'_, PyAny>,
    options: &Bound<'_, PyAny>,
    feed: bool,
) -> PyResult<thinkthen::Request> {
    build_request(py, verb, asked, source, options, feed).map_err(|error| {
        if error.is_instance_of::<pyo3::exceptions::PyTypeError>(py)
            || error.is_instance_of::<pyo3::exceptions::PyValueError>(py)
            || error.is_instance_of::<pyo3::exceptions::PyOverflowError>(py)
        {
            usage(py, "invalid canonical request")
        } else {
            error
        }
    })
}
