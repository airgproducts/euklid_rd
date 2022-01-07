use pyo3::prelude::*;

/// Prints out Hello World
#[pyfunction]
fn hello_world() -> PyResult<String> {
    let hello = String::from("Hello, world!");
    Ok(hello)
}

/// A Python module implemented in Rust.
/// For tests "Hello, world!" is returned.
#[pymodule]
fn euklid_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}
