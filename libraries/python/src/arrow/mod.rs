//! The Arrow door (ticket 0106): a Polars `Series` or any Arrow column in,
//! read in place, and answer columns or the caller's frame out, through the
//! Arrow PyCapsule interface. The wheel never imports Polars, and no Python
//! loop touches a row.
//!
//! The three `ffi.rs` files hold every `unsafe` block of the binding: `ffi` reads a producer's memory, `out` hands answers back, and `probe` is a test producer. `memory` lists what this
//! process can read, `read` checks and borrows a text column, `write` builds
//! what goes back, and `gate` releases a column's batches at exit safely.

#[allow(
    unsafe_code,
    reason = "the Arrow C data interface is raw memory and C callbacks (ADR 0047 item 3)"
)]
mod ffi;
mod gate;
mod memory;
#[allow(
    unsafe_code,
    reason = "the Arrow C data interface is raw memory and C callbacks (ADR 0047 item 3)"
)]
#[path = "out/ffi.rs"]
mod out;
#[cfg(feature = "probe")]
#[allow(
    unsafe_code,
    reason = "a C producer's lock-bound release, for the exit-gate test (ticket 0106 change 6)"
)]
#[path = "probe/ffi.rs"]
mod probe;
mod read;
mod write;

pub(crate) use ffi::Imported;
pub(crate) use gate::_exit_gate;
#[cfg(feature = "probe")]
pub(crate) use gate::_probe_trace;
pub(crate) use memory::Readable;
#[cfg(feature = "probe")]
pub(crate) use probe::_raw_producer;
pub(crate) use read::{frame, series};
pub(crate) use write::{Arrow, Cells, Output, annotated, column, decided, kept, table};

/// The caller's frame with its new columns, from `write`.
pub(crate) use write::frame as frame_out;

/// One answer column as a pair: its Python values and the pandas dtype that
/// holds them (ticket 0122). "Not sure" is `None`, which pandas reads as NA.
pub(crate) fn pandas(py: pyo3::Python<'_>, cells: Cells) -> pyo3::PyResult<pyo3::Py<pyo3::PyAny>> {
    use pyo3::IntoPyObject as _;
    let (values, dtype) = match cells {
        Cells::Bools(values) => (values.into_pyobject(py)?, "boolean"),
        Cells::Numbers(values) => (values.into_pyobject(py)?, "Float64"),
        Cells::Counts(values) => (values.into_pyobject(py)?, "Int64"),
        Cells::Texts(values) => (values.into_pyobject(py)?, "string"),
        Cells::Lists(values) => (values.into_pyobject(py)?, "object"),
    };
    Ok((values, dtype).into_pyobject(py)?.into_any().unbind())
}
