use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod plane;

pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn plane(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<plane::Plane>()?;
        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(plane))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.plane", m.getattr("plane")?)?;
    Ok(())
}
