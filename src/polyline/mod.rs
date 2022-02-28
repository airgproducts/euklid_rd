use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

mod polyline;
mod polyline_2d;

pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn polyline(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<polyline::PolyLine2D>()?;
        m.add_class::<polyline::PolyLine3D>()?;
        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(polyline))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.polyline", m.getattr("polyline")?)?;
    Ok(())
}
