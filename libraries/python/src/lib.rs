//! The Python binding: `import thinkthen as tt` (ticket 0105, ADR 0047).
//!
//! Every call reaches the real engine through the public `thinkthen` API on a
//! detachable worker thread (`worker`). A Polars column or frame crosses
//! through the Arrow door (`arrow`, `frame`, ticket 0106), and a pandas column
//! through the door or the list reader (ticket 0122). This file holds the module edge: the
//! six exception classes, the one table from an error kind to its class, and
//! the one panic guard.

mod arrow;
mod asked;
mod diagnostics;
mod engine;
mod frame;
mod input;
mod result;
mod stream;
mod tally;
mod worker;

use pyo3::exceptions::{PyException, PyKeyboardInterrupt};
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyDict, PyTuple, PyType};
use thinkthen::{Error, ErrorKind, contained};

pyo3::create_exception!(
    thinkthen._thinkthen,
    ThinkThenError,
    PyException,
    "Every thinkthen failure. `kind` names it, and `retryable` says whether the same call may pass later."
);
pyo3::create_exception!(
    thinkthen._thinkthen,
    UsageError,
    ThinkThenError,
    "The call or a setting cannot act as given. Nothing was sent."
);
pyo3::create_exception!(
    thinkthen._thinkthen,
    BackendError,
    ThinkThenError,
    "The backend failed or refused the call."
);
pyo3::create_exception!(
    thinkthen._thinkthen,
    LocalError,
    ThinkThenError,
    "A local file, folder, or cache failed."
);
pyo3::create_exception!(
    thinkthen._thinkthen,
    DeadlineError,
    ThinkThenError,
    "The call's deadline passed. It says nothing about the backend's health."
);
pyo3::create_exception!(
    thinkthen._thinkthen,
    DefectError,
    ThinkThenError,
    "A fault inside thinkthen. Report it."
);

/// `Cancelled` subclasses both `KeyboardInterrupt` and `ThinkThenError`, so
/// either `except` catches it. `create_exception!` gives a class one base, so
/// this class is built once with two.
static CANCELLED: PyOnceLock<Py<PyType>> = PyOnceLock::new();

const CANCELLED_DOC: &str = "The call stopped on Ctrl-C or its token. No new request started, and sent requests finish on their own.";

fn cancelled_class(py: Python<'_>) -> PyResult<Bound<'_, PyType>> {
    let class = CANCELLED.get_or_try_init(py, || -> PyResult<Py<PyType>> {
        let namespace = PyDict::new(py);
        namespace.set_item("__doc__", CANCELLED_DOC)?;
        namespace.set_item("__module__", "thinkthen._thinkthen")?;
        let bases = PyTuple::new(
            py,
            [
                py.get_type::<PyKeyboardInterrupt>().into_any(),
                py.get_type::<ThinkThenError>().into_any(),
            ],
        )?;
        let made = py
            .get_type::<PyType>()
            .call1(("Cancelled", bases, namespace))?;
        Ok(made.cast_into::<PyType>()?.unbind())
    })?;
    Ok(class.bind(py).clone())
}

/// The one table from an error kind to its class (R1-31).
fn class_of(py: Python<'_>, kind: ErrorKind) -> PyResult<Bound<'_, PyType>> {
    Ok(match kind {
        ErrorKind::Usage => py.get_type::<UsageError>(),
        ErrorKind::Backend => py.get_type::<BackendError>(),
        ErrorKind::Local => py.get_type::<LocalError>(),
        ErrorKind::Cancelled => cancelled_class(py)?,
        ErrorKind::Deadline => py.get_type::<DeadlineError>(),
        ErrorKind::Defect => py.get_type::<DefectError>(),
    })
}

const KINDS: [ErrorKind; 6] = [
    ErrorKind::Usage,
    ErrorKind::Backend,
    ErrorKind::Local,
    ErrorKind::Cancelled,
    ErrorKind::Deadline,
    ErrorKind::Defect,
];

/// An exception of this kind, carrying its `kind` word and retry signal.
pub(crate) fn raise(py: Python<'_>, kind: ErrorKind, message: &str, retryable: bool) -> PyErr {
    let class = match class_of(py, kind) {
        Ok(class) => class,
        Err(error) => return error,
    };
    let error = PyErr::from_type(class, message.to_owned());
    let value = error.value(py);
    match value
        .setattr("kind", kind.name())
        .and_then(|()| value.setattr("retryable", retryable))
    {
        Ok(()) => error,
        Err(failed) => failed,
    }
}

/// The engine's error as the exception of its kind.
pub(crate) fn raised(py: Python<'_>, error: &Error) -> PyErr {
    let raised = raise(py, error.kind(), &error.to_string(), error.retryable());
    if let Some(facts) = error.facts()
        && let Ok(value) = result::python_facts(py, facts)
    {
        let _set = raised.value(py).setattr("facts", value);
    }
    raised
}

/// A usage error with this message.
pub(crate) fn usage(py: Python<'_>, message: &str) -> PyErr {
    raise(py, ErrorKind::Usage, message, false)
}

/// A defect with this message.
pub(crate) fn defect(py: Python<'_>, message: &str) -> PyErr {
    raise(py, ErrorKind::Defect, &format!("defect: {message}"), false)
}

/// The module edge: a panic in the binding raises `DefectError`, and the
/// interpreter carries on.
pub(crate) fn guard<T>(py: Python<'_>, call: impl FnOnce() -> PyResult<T>) -> PyResult<T> {
    contained(call).unwrap_or_else(|| Err(defect(py, "the Python binding panicked")))
}

#[pymodule]
fn _thinkthen(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = module.py();
    module.add("ThinkThenError", py.get_type::<ThinkThenError>())?;
    for kind in KINDS {
        let class = class_of(py, kind)?;
        class.setattr("kind", kind.name())?;
        class.setattr("retryable", false)?;
        module.add(class.name()?, class)?;
    }
    module.add_class::<engine::Engine>()?;
    module.add_class::<result::PyCall>()?;
    module.add_class::<tally::PyTally>()?;
    module.add_class::<stream::PyStream>()?;
    module.add_class::<worker::Token>()?;
    module.add_class::<worker::Receipt>()?;
    module.add_class::<worker::Completion>()?;
    module.add_class::<asked::Question>()?;
    module.add_class::<asked::QuestionSet>()?;
    module.add_class::<asked::Recognize>()?;
    module.add_class::<asked::Relate>()?;
    module.add_class::<asked::Entity>()?;
    module.add_class::<asked::Edge>()?;
    module.add_class::<asked::RecognizedEntity>()?;
    module.add_class::<asked::Relation>()?;
    module.add_class::<asked::Recognized>()?;
    module.add_class::<arrow::Arrow>()?;
    module.add_class::<input::Pandas>()?;
    module.add_function(wrap_pyfunction!(frame::_annotate_frame, module)?)?;
    module.add_function(wrap_pyfunction!(frame::_recognize_frame, module)?)?;
    module.add_function(wrap_pyfunction!(frame::_annotate_column, module)?)?;
    module.add_function(wrap_pyfunction!(frame::_recognize_column, module)?)?;
    // A column's batches are released behind this hook at exit (change 6).
    let gate = wrap_pyfunction!(arrow::_exit_gate, module)?;
    py.import("atexit")?.call_method1("register", (&gate,))?;
    module.add_function(gate)?;
    #[cfg(feature = "probe")]
    {
        module.add_function(wrap_pyfunction!(worker::_live_workers, module)?)?;
        module.add_function(wrap_pyfunction!(frame::_arrow_probe, module)?)?;
        module.add_function(wrap_pyfunction!(arrow::_raw_producer, module)?)?;
        module.add_function(wrap_pyfunction!(arrow::_probe_trace, module)?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use pyo3::prelude::*;

    use super::{KINDS, class_of, guard, raise};

    /// R1-31: each kind maps to its own class and `kind` word.
    #[test]
    fn each_kind_raises_its_own_class() {
        Python::initialize();
        Python::attach(|py| {
            let named: Vec<(String, String)> = KINDS
                .iter()
                .map(|kind| {
                    let value = raise(py, *kind, "a message", false).value(py).clone();
                    let class = class_of(py, *kind).and_then(|class| class.name());
                    let word = value.getattr("kind").and_then(|word| word.extract());
                    (
                        class.map(|name| name.to_string()).unwrap_or_default(),
                        word.unwrap_or_default(),
                    )
                })
                .collect();
            let expected = [
                ("UsageError", "usage"),
                ("BackendError", "backend"),
                ("LocalError", "local"),
                ("Cancelled", "cancelled"),
                ("DeadlineError", "deadline"),
                ("DefectError", "defect"),
            ];
            let named: Vec<(&str, &str)> = named
                .iter()
                .map(|(class, word)| (class.as_str(), word.as_str()))
                .collect();
            assert_eq!(named, expected);
        });
    }

    /// R1-10 host half: a panic at the module edge is a `DefectError`, and
    /// the next attached call runs.
    #[test]
    fn a_panic_is_a_defect_and_the_next_call_runs() {
        Python::initialize();
        Python::attach(|py| {
            let panicked: PyResult<()> = guard(py, || std::panic::resume_unwind(Box::new(7)));
            let seen = panicked.err().map(|error| {
                let defect = error.is_instance_of::<super::DefectError>(py);
                (defect, error.value(py).to_string())
            });
            let said = "defect: the Python binding panicked".to_owned();
            assert_eq!(seen, Some((true, said)));
            assert_eq!(guard(py, || Ok(7)).ok(), Some(7));
        });
    }
}
