use pyo3::prelude::*;
use pyo3::class::basic::PyObjectProtocol;

#[pyclass]
struct Vector2D {
    #[pyo3(get, set)]
    x: f32,
    #[pyo3(get, set)]
    y: f32,
}


#[pymethods]
impl Vector2D {
    #[new]
    pub fn __new__(v: [f32; 2]) -> Self {
        Vector2D {x: v[0], y: v[1]}
    }
    
    pub fn normalized(&self) -> PyResult<Self> {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        let x = self.x / length;
        let y = self.y / length;
        Ok(Vector2D {x, y})
    }
}

#[pyproto]
impl PyObjectProtocol for Vector2D {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Vector2D({:.4} {:.4})", self.x, self.y))
    }
}

#[pymodule]
pub fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vector2D>()?;
    Ok(())
}