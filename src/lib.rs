use pyo3::prelude::*;

mod plane;
pub mod vector;
mod polyline;

/// A Python module implemented in Rust.
#[pymodule]
fn euklid_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    plane::register(_py, m)?;
    vector::register(_py, m)?;
    polyline::register(_py, m)?;
    Ok(())
}
