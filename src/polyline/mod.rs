use pyo3::prelude::*;
use pyo3::types::PyDict;

mod polyline;
mod polyline_2d;

pub use polyline::PolyLine2D;
pub use polyline::PolyLine3D;

pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(_py, "polyline")?;

    child_module.add_class::<PolyLine2D>()?;
    child_module.add_class::<PolyLine3D>()?;
    m.add_submodule(child_module)?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.polyline", m.getattr("polyline")?)?;

    Ok(())
}
