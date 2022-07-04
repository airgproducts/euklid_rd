use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod basis;
mod curve;

pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn spline(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<curve::BezierCurve>()?;
        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(spline))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.spline", m.getattr("spline")?)?;
    Ok(())
}
