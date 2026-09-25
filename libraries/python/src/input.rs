//! Reading Python arguments into owned Rust values before any request.
//!
//! A container a list verb does not read is refused first: pandas, Polars,
//! or pyarrow by its module, then anything Arrow-shaped (decision 2). A column
//! reaches the verbs that read one through `frame` (tickets 0106 and 0122). A
//! list is read whole, and a bad item raises `UsageError` naming its index
//! (decision 7).

use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyInt, PyString};
use thinkthen::{CallOptions, CancelToken};

use crate::asked::Entity;
use crate::worker::{Controls, Token};
use crate::{raised, usage};

/// The refusal for a column where a verb reads a list.
pub(crate) const ARROW: &str = "filter, rank, find, and relate read a list of str, not a column, and annotate and recognize read a column only from a Polars or pandas frame with on=. Pass column.to_list()";

/// The refusal for a deadline that is not a number (R5-8).
pub(crate) const DEADLINE: &str =
    "deadline is seconds from now, a number; no deadline is spelled None or -1";

/// The top-level module of a value's type, or `pandas` when any class in its
/// method resolution order is pandas', so a subclass counts.
pub(crate) fn top(value: &Bound<'_, PyAny>) -> PyResult<String> {
    let mut first = None;
    for kind in value.get_type().mro() {
        let module = kind.getattr("__module__")?.str()?;
        let top = module.to_str()?.split('.').next().unwrap_or_default();
        if top == "pandas" {
            return Ok(top.to_owned());
        }
        first.get_or_insert_with(|| top.to_owned());
    }
    Ok(first.unwrap_or_default())
}

/// Refuse a pandas, Polars, or pyarrow value, then any Arrow-shaped object,
/// before anything is read.
pub(crate) fn refuse_container(value: &Bound<'_, PyAny>) -> PyResult<()> {
    let arrow = ["__arrow_c_stream__", "__arrow_c_array__", "__dataframe__"];
    if matches!(top(value)?.as_str(), "pandas" | "polars" | "pyarrow")
        || arrow
            .iter()
            .any(|name| value.hasattr(*name).unwrap_or(false))
    {
        return Err(usage(value.py(), ARROW));
    }
    Ok(())
}

/// A pandas Series the package marked for the door (a holder) or the list reader.
#[pyclass(frozen, name = "_Pandas", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Pandas(pub(crate) Py<PyAny>, pub(crate) bool);

#[pymethods]
impl Pandas {
    #[new]
    const fn new(value: Py<PyAny>, list: bool) -> Self {
        Self(value, list)
    }
}

/// True for an Arrow column or a pandas value, marked or not (`details` refuses).
pub(crate) fn is_column(value: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(value.is_instance_of::<Pandas>()
        || top(value)? == "pandas"
        || value.hasattr("__arrow_c_stream__")?
        || value.hasattr("__arrow_c_array__")?)
}

/// A Polars frame, or a refusal before any request (decision 4).
pub(crate) fn polars_frame(value: &Bound<'_, PyAny>, verb: &str) -> PyResult<()> {
    if top(value)? != "polars" || !value.hasattr("__arrow_c_stream__")? {
        return Err(usage(
            value.py(),
            &format!(
                "{verb} with on= takes a Polars or pandas DataFrame; a list of str takes no on="
            ),
        ));
    }
    Ok(())
}

/// One `str` as Rust text, or `None` when it is not a `str`.
fn string(value: &Bound<'_, PyAny>, what: &str) -> PyResult<Option<String>> {
    let Ok(text) = value.cast::<PyString>() else {
        return Ok(None);
    };
    let text = text.to_str().map_err(|_| {
        usage(
            value.py(),
            &format!("{what} holds a lone surrogate, which is not Unicode text"),
        )
    })?;
    Ok(Some(text.to_owned()))
}

/// The evidence of a single call.
pub(crate) fn text(value: &Bound<'_, PyAny>) -> PyResult<String> {
    if let Some(text) = string(value, "the evidence")? {
        return Ok(text);
    }
    refuse_container(value)?;
    Err(usage(value.py(), "the evidence is a str"))
}

/// Every text of an iterable, read whole before the first send.
pub(crate) fn texts(records: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    let py = records.py();
    refuse_container(records)?;
    if records.is_instance_of::<PyString>() {
        return Err(usage(py, "the records are a list of str, not one str"));
    }
    listed(records)
}

/// Every text of an iterable, with no container check (a marked pandas Series).
pub(crate) fn listed(records: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    let py = records.py();
    let items = records.try_iter().map_err(|_| {
        usage(
            py,
            "the records are a list, tuple, or other iterable of str",
        )
    })?;
    let mut read = Vec::new();
    for (index, item) in items.enumerate() {
        let item = item?;
        let what = format!("record {index}");
        read.push(string(&item, &what)?.ok_or_else(|| usage(py, &format!("{what} is not a str")))?);
    }
    Ok(read)
}

/// The entities `relate` reads: `(name, kind)` pairs, dictionaries with
/// `name` and `kind`, or `Entity` values (decision 10).
pub(crate) fn entities(values: &Bound<'_, PyAny>) -> PyResult<Vec<thinkthen::Entity>> {
    let py = values.py();
    refuse_container(values)?;
    let items = values
        .try_iter()
        .map_err(|_| usage(py, "the entities are a list of (name, kind) pairs"))?;
    let mut read = Vec::new();
    for (index, item) in items.enumerate() {
        let item = item?;
        let refused = || {
            usage(
                py,
                &format!(
                    "entity {index} is not a (name, kind) pair, a dict with name and kind, or an Entity"
                ),
            )
        };
        let (name, kind): (String, String) = if let Ok(entity) = item.cast::<Entity>() {
            let entity = entity.get();
            (entity.name.clone(), entity.kind.clone())
        } else if let Ok(fields) = item.cast::<PyDict>() {
            let field = |key: &str| -> PyResult<String> {
                fields
                    .get_item(key)?
                    .and_then(|value| value.extract().ok())
                    .ok_or_else(refused)
            };
            (field("name")?, field("kind")?)
        } else {
            item.extract().map_err(|_| refused())?
        };
        read.push(thinkthen::Entity::new(&name, &kind).map_err(|error| raised(py, &error))?);
    }
    Ok(read)
}

/// The caller's token and deadline. A deadline is checked here, before any
/// work starts: a bool or a non-number is refused, and the public rule
/// (ADR 0041) judges the number.
pub(crate) fn controls(
    py: Python<'_>,
    deadline: Option<&Bound<'_, PyAny>>,
    token: Option<&Bound<'_, Token>>,
) -> PyResult<Controls> {
    let deadline = match deadline {
        None => None,
        Some(value) => {
            // NumPy 1 names its bool `bool_`, and NumPy 2 names it `bool`.
            let name = value.get_type().name()?;
            if value.is_instance_of::<PyBool>() || matches!(name.to_str()?, "bool" | "bool_") {
                return Err(usage(py, DEADLINE));
            }
            let seconds: f64 = value.extract().map_err(|_| usage(py, DEADLINE))?;
            CallOptions::new()
                .deadline_seconds(seconds)
                .map_err(|error| raised(py, &error))?;
            Some(seconds)
        }
    };
    Ok(Controls {
        token: token.map(|held| CancelToken::clone(&held.get().0)),
        deadline,
    })
}

/// A whole number for an engine setting. A bool or any other type is refused
/// with the setting's own sentence.
pub(crate) fn whole(value: &Bound<'_, PyAny>, sentence: &str) -> PyResult<i64> {
    if value.is_instance_of::<PyBool>() || !value.is_instance_of::<PyInt>() {
        return Err(usage(value.py(), sentence));
    }
    value.extract().map_err(|_| usage(value.py(), sentence))
}
