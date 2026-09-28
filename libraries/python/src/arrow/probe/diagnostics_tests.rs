//! One isolated child observes owned panic secrecy and host callback delegation.

use std::ffi::c_void;
use std::io::Write as _;
use std::ptr;

use pyo3::prelude::*;

use super::{
    ArrowArrayStream, STREAM, State, get_next, get_schema, stream_destructor, stream_release,
};
use crate::arrow::Imported;
use crate::arrow::out::capsule;

const CHILD: &str = "THINKTHEN_PYTHON_PANIC_CHILD";
const STRING: &str = "python-owned-string-payload-marker";
const DROP: &str = "python-owned-drop-payload-marker";
const CONVERSION: &str = "python-conversion-marker";
const SIGNAL: &str = "python-signal-marker";
const INGRESS: &str = "arrow-ingress-marker";
const RELEASE: &str = "arrow-release-marker";

struct Exploding;

impl Drop for Exploding {
    fn drop(&mut self) {
        panic!("{DROP}");
    }
}

#[pyclass]
struct HostIterable;

#[pymethods]
impl HostIterable {
    fn __iter__(&self) -> PyResult<Py<PyAny>> {
        panic!("{CONVERSION}");
    }
}

#[pyclass]
struct SignalHandler;

#[pymethods]
impl SignalHandler {
    fn __call__(&self, _signal: i32, _frame: &Bound<'_, PyAny>) {
        panic!("{SIGNAL}");
    }
}

#[pyclass]
struct Producer;

#[pymethods]
impl Producer {
    fn __arrow_c_stream__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let state = Box::new(State {
            remaining: 1,
            token: py.None().into_ptr(),
            table: [
                ptr::null(),
                super::OFFSETS.as_ptr().cast(),
                super::VALUES.as_ptr().cast(),
            ],
            diagnostic: true,
        });
        let stream = Box::new(ArrowArrayStream {
            get_schema: Some(get_schema),
            get_next: Some(get_next),
            get_last_error: None,
            release: Some(stream_release),
            private_data: Box::into_raw(state).cast::<c_void>(),
        });
        capsule(py, Box::into_raw(stream).cast(), STREAM, stream_destructor)
    }
}

#[test]
fn a_caught_python_panic_delegates_each_host_callback() {
    if std::env::var_os(CHILD).is_some() {
        child();
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
        .args([
            "--exact",
            "arrow::probe::diagnostics_tests::a_caught_python_panic_delegates_each_host_callback",
            "--nocapture",
        ])
        .env(CHILD, "1")
        .output()
        .expect("isolated Python proof");
    assert!(
        output.status.success(),
        "stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for stream in [&output.stdout, &output.stderr] {
        let text = String::from_utf8_lossy(stream);
        assert!(!text.contains(STRING), "{text}");
        assert!(!text.contains(DROP), "{text}");
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    for marker in [CONVERSION, SIGNAL, INGRESS, RELEASE, "host-thread-marker"] {
        assert!(stderr.contains(&format!("{marker}\n")), "{stderr}");
    }
}

fn child() {
    std::panic::set_hook(Box::new(|info| {
        let text = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or(if info.payload().is::<Exploding>() {
                DROP
            } else {
                "other panic"
            });
        let _ = std::io::stderr().write_all(format!("{text}\n").as_bytes());
    }));
    Python::initialize();
    Python::attach(|py| {
        for body in [
            || -> PyResult<()> { panic!("{STRING}") },
            || -> PyResult<()> { std::panic::panic_any(Exploding) },
        ] {
            let error = crate::guard(py, body).expect_err("a panic is a defect");
            assert!(error.is_instance_of::<crate::DefectError>(py));
            assert_eq!(
                error.value(py).to_string(),
                "defect: the Python binding panicked"
            );
        }
        assert!(crate::guard(py, || Ok::<_, PyErr>(7)).is_ok());

        let iterable = Py::new(py, HostIterable).expect("synthetic iterable");
        let producer = Py::new(py, Producer).expect("synthetic Arrow producer");
        let signal = py.import("signal").expect("Python signal module");
        let signum = signal.getattr("SIGINT").expect("SIGINT");
        let handler = Py::new(py, SignalHandler).expect("synthetic handler");
        signal
            .call_method1("signal", (signum, handler))
            .expect("install Python handler");
        let converted = crate::guard(py, || {
            crate::input::texts(iterable.bind(py).as_any()).map(|_| ())
        });
        assert!(converted.is_err());
        let observed = crate::guard(py, || {
            let held = Imported::column(producer.bind(py).as_any())?;
            let worker = std::thread::spawn(move || crate::caught(|| drop(held)));
            assert!(py.detach(|| worker.join()).is_ok());
            // SAFETY: this CPython test owns the interpreter; the pending
            // signal is dispatched by the same check the binding's wait uses.
            unsafe { pyo3::ffi::PyErr_SetInterrupt() };
            crate::diagnostics::host(|| py.check_signals())
        });
        assert!(observed.is_err());
    });
    let _ = std::thread::spawn(|| panic!("host-thread-marker")).join();
}
