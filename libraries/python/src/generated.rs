//! Generated from `functions.toml` by `scripts/generate_functions.py`.
//!
//! Do not edit by hand: edit the table and run the generator. Every name
//! here is a `#[pyfunction]` in this crate, and a missing one fails the
//! build with a clear error.

use pyo3::prelude::*;

/// Register every function of the table on the extension module.
pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(crate::decide, module)?)?;
    module.add_function(wrap_pyfunction!(crate::choose, module)?)?;
    module.add_function(wrap_pyfunction!(crate::score, module)?)?;
    module.add_function(wrap_pyfunction!(crate::tag, module)?)?;
    module.add_function(wrap_pyfunction!(crate::filter, module)?)?;
    module.add_function(wrap_pyfunction!(crate::rank, module)?)?;
    module.add_function(wrap_pyfunction!(crate::find, module)?)?;
    module.add_function(wrap_pyfunction!(crate::annotate_rows, module)?)?;
    module.add_function(wrap_pyfunction!(crate::decide_many, module)?)?;
    module.add_function(wrap_pyfunction!(crate::details, module)?)?;
    module.add_function(wrap_pyfunction!(crate::usage, module)?)?;
    module.add_function(wrap_pyfunction!(crate::question, module)?)?;
    module.add_function(wrap_pyfunction!(crate::recognize, module)?)?;
    module.add_function(wrap_pyfunction!(crate::relate, module)?)?;
    Ok(())
}
