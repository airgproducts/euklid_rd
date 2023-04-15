use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

pub mod _vector;
pub mod transform;

pub use _vector::{Vector2D, Vector3D};
pub use transform::Transformation;

pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Vector2D>()?;
        m.add_class::<Vector3D>()?;
        m.add_class::<Transformation>()?;
        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(vector))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.vector", m.getattr("vector")?)?;
    Ok(())
}
