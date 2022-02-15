use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

pub mod transform;
pub mod vector;

pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<vector::Vector2D>()?;
        m.add_class::<vector::Vector3D>()?;
        m.add_class::<transform::Transformation>()?;
        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(vector))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.vector", m.getattr("vector")?)?;
    Ok(())
}
