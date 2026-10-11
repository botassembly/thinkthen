//! Python value conversion into native request arguments.
use crate::{raised, usage};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString, PyTuple};
use std::path::PathBuf;
use thinkthen::{
    RawRecord, RequestArguments, RequestCall, RequestInput, RequestItem, RequestOriginal,
    RequestQuestion,
};

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
    serde_json::from_str(&json(py, value)?).map_err(|error| {
        let unsupported = "the question declaration uses an unsupported feature";
        usage(
            py,
            if error.to_string().starts_with(unsupported) {
                unsupported
            } else {
                "invalid canonical request"
            },
        )
    })
}
pub(crate) fn question(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<RequestQuestion> {
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
            value: decode(py, body)?,
        },
        "raw" => RequestQuestion::Definition {
            value: serde_json::from_str(&body.extract::<String>()?)
                .map_err(|_| usage(py, "invalid authored question definition"))?,
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
fn original(py: Python<'_>, value: &Bound<'_, PyAny>, text: bool) -> PyResult<RequestOriginal> {
    if text {
        Ok(RequestOriginal::Text {
            text: value.extract()?,
        })
    } else {
        Ok(RequestOriginal::Json {
            value: RawRecord::json(&json(py, value)?).map_err(|e| raised(py, &e))?,
        })
    }
}
pub(crate) fn item(py: Python<'_>, value: &Bound<'_, PyAny>, verb: &str) -> PyResult<RequestItem> {
    let inputs = py.import("thinkthen._inputs")?;
    let absent = inputs.getattr("ABSENT")?;
    let mut item = RequestItem {
        original: None,
        context: None,
        options: None,
        seed_spans: None,
        examples: None,
        images: Vec::new(),
    };
    if value.is_instance(&inputs.getattr("Item")?)? {
        if !value.getattr("image_only")?.extract::<bool>()? {
            item.original = Some(original(
                py,
                &value.getattr("value")?,
                value.getattr("text")?.extract()?,
            )?);
        }
        for image in value.getattr("images")?.try_iter()? {
            let image = image?;
            item.images.push(thinkthen::RequestImage::Bytes {
                media: decode(py, &image.getattr("media")?)?,
                bytes: image.getattr("data")?.extract()?,
            });
        }
        let context = value.getattr("context")?;
        if !context.is(&absent) {
            item.context = Some(if context.is_instance_of::<PyString>() {
                thinkthen::RecordContext::Text(context.extract()?)
            } else {
                let raw = RawRecord::json(&json(py, &context)?).map_err(|e| raised(py, &e))?;
                thinkthen::RecordContext::Object(
                    thinkthen::ObjectContext::new(&raw).map_err(|e| raised(py, &e))?,
                )
            });
        }
        let options = value.getattr("options")?;
        if !options.is(&absent) {
            let options = options
                .try_iter()?
                .map(|pair| {
                    let (name, description): (String, Py<PyAny>) = pair?.extract()?;
                    let description = description.bind(py);
                    Ok(thinkthen::RecordOption {
                        name,
                        description: if description.is(&absent) {
                            None
                        } else {
                            Some(
                                thinkthen::Description::from_json(&json(py, description)?)
                                    .map_err(|e| raised(py, &e))?,
                            )
                        },
                    })
                })
                .collect::<PyResult<Vec<_>>>()?;
            item.options =
                Some(thinkthen::RecordOptions::new(options).map_err(|e| raised(py, &e))?);
        }
    } else {
        let entity;
        let value = if verb == "relate" && value.is_instance_of::<PyTuple>() {
            let (name, kind): (Py<PyAny>, Py<PyAny>) = value.extract()?;
            entity = PyDict::new(py);
            entity.set_item("name", name)?;
            entity.set_item("kind", kind)?;
            entity.as_any()
        } else if verb == "relate"
            && value.is_instance(
                &py.import("thinkthen._thinkthen")?
                    .getattr("_NativeResult")?,
            )?
        {
            entity = PyDict::new(py);
            entity.set_item("name", value.getattr("text")?)?;
            entity.set_item("kind", value.getattr("kind")?)?;
            entity.as_any()
        } else {
            value
        };
        item.original = Some(original(py, value, value.is_instance_of::<PyString>())?);
    }
    Ok(item)
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
    let inputs = py.import("thinkthen._inputs")?;
    if value.is_instance(&inputs.getattr("Files")?)?
        || value.is_instance(&py.import("thinkthen.files")?.getattr("FileSelection")?)?
    {
        let window = value.getattr("window")?;
        let window = if window.is_none() || window.is(&inputs.getattr("ABSENT")?) {
            None
        } else {
            crate::controls::whole_number(py, &window)?;
            Some(window.extract()?)
        };
        return Ok(RequestInput::Source {
            source: thinkthen::RequestSource {
                paths: value.getattr("paths")?.extract::<Vec<PathBuf>>()?,
                reading: reading(py, &value.getattr("unit")?.extract::<String>()?, window)?,
                media: value
                    .getattr("media")
                    .map_or(Ok(thinkthen::ReaderMedia::Text), |v| decode(py, &v))?,
                framing: if value
                    .getattr("jsonl")
                    .is_ok_and(|v| v.extract::<bool>().unwrap_or(false))
                {
                    Some(thinkthen::RequestFraming::Jsonl)
                } else {
                    None
                },
            },
        });
    }
    let records;
    let value = if value.is_instance(&inputs.getattr("Records")?)? {
        records = value.getattr("items")?;
        &records
    } else {
        value
    };
    let items = if value.is_instance_of::<PyList>() || value.is_instance_of::<PyTuple>() {
        value
            .try_iter()?
            .map(|v| item(py, &v?, verb))
            .collect::<PyResult<Vec<_>>>()?
    } else {
        vec![item(py, value, verb)?]
    };
    Ok(match verb {
        "find" => RequestInput::Units { items },
        "relate" => RequestInput::Entities { items },
        _ => RequestInput::Records { items },
    })
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
    let call = match verb {
        "decide" => RequestCall::Decide(args),
        "choose" => RequestCall::Choose(args),
        "tag" => RequestCall::Tag(args),
        "score" => RequestCall::Score(args),
        "filter" => RequestCall::Filter(args),
        "rank" => RequestCall::Rank(args),
        "find" => RequestCall::Find(args),
        "annotate" => RequestCall::Annotate(args),
        "recognize" => RequestCall::Recognize(args),
        "relate" => RequestCall::Relate(args),
        _ => return Err(usage(py, "invalid request function")),
    };
    Ok(thinkthen::Request::new(call))
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
