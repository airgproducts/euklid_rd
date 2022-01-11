use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod vector_rs;
use vector_rs::*;

/// A Python module implemented in Rust.
#[pymodule]
fn euklid_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(vector))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.vector", m.getattr("vector")?)?;
    Ok(())
}
