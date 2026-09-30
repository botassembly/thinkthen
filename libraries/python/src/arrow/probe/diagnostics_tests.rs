//! One isolated child observes owned panic secrecy and host callback delegation.

use std::ffi::{CString, c_void};
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
const NEXT: &str = "python-next-marker";
const ITEM_DROP: &str = "python-item-finalize-marker";
const BAD_DROP: &str = "python-bad-item-finalize-marker";
const ITER_DROP: &str = "python-iterator-finalize-marker";
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

#[pyfunction]
fn host_callback_marker(marker: &str) {
    let _ = std::panic::catch_unwind(|| panic!("{marker}"));
}

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
    } else {
        let mut command = std::process::Command::new(std::env::current_exe().expect("test binary"));
        command.env_clear();
        for name in ["LD_LIBRARY_PATH", "PYTHONHOME"] {
            if let Some(value) = std::env::var_os(name) {
                command.env(name, value);
            }
        }
        let output = command
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
        for marker in [
            CONVERSION,
            NEXT,
            ITEM_DROP,
            BAD_DROP,
            ITER_DROP,
            SIGNAL,
            INGRESS,
            RELEASE,
            "host-thread-marker",
        ] {
            assert!(stderr.contains(&format!("{marker}\n")), "{stderr}");
        }
    }
}

fn callback_records(py: Python<'_>) {
    let source = CString::new(
        r#"
class Item(str):
    def __del__(self):
        host_callback_marker("python-item-finalize-marker")
class Bad:
    def __del__(self):
        host_callback_marker("python-bad-item-finalize-marker")
class Iterator:
    def __init__(self, bad):
        self.bad = bad
        self.used = False
    def __iter__(self):
        return self
    def __next__(self):
        if self.used:
            raise StopIteration
        self.used = True
        host_callback_marker("python-next-marker")
        return Bad() if self.bad else Item("x")
    def __del__(self):
        host_callback_marker("python-iterator-finalize-marker")
class Records:
    def __init__(self, bad):
        self.bad = bad
    def __iter__(self):
        return Iterator(self.bad)
"#,
    )
    .expect("callback source");
    let callbacks =
        PyModule::from_code(py, &source, c"callbacks.py", c"callbacks").expect("callback module");
    callbacks
        .add_function(wrap_pyfunction!(host_callback_marker, &callbacks).expect("marker"))
        .expect("install marker");
    let records = callbacks
        .getattr("Records")
        .expect("records class")
        .call1((false,))
        .expect("records");
    assert_eq!(
        crate::guard(py, || crate::input::texts(&records)).expect("records read"),
        ["x"]
    );
    let invalid = callbacks
        .getattr("Records")
        .expect("records class")
        .call1((true,))
        .expect("invalid records");
    assert!(crate::guard(py, || crate::input::texts(&invalid)).is_err());
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
            assert!(
                !error
                    .value(py)
                    .getattr("retryable")
                    .expect("retry flag")
                    .extract::<bool>()
                    .expect("boolean retry flag")
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
        callback_records(py);
        let observed = crate::guard(py, || {
            let held = Imported::column(producer.bind(py).as_any())?;
            let worker = std::thread::spawn(move || thinkthen::contained(|| drop(held)));
            assert!(py.detach(|| worker.join()).is_ok());
            super::interrupt_for_diagnostic();
            crate::worker::run(py, crate::worker::Controls::default(), |_options| {
                std::thread::sleep(std::time::Duration::from_millis(150));
                Ok(7)
            })
            .map(|_| ())
        });
        assert!(observed.is_err());
    });
    let _ = std::thread::spawn(|| panic!("host-thread-marker")).join();
}
