use pyo3::prelude::*;

mod polyline;
mod vector;

/// A Python module implemented in Rust.
#[pymodule]
fn euklid_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    vector::register(_py, m)?;
    polyline::register(_py, m)?;
    Ok(())
}
