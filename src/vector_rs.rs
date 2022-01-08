use pyo3::prelude::*;


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
    
    pub fn normalized(&mut self) -> PyResult<()> {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        self.x = self.x / length;
        self.y = self.y / length;
        Ok(())
    }
}

#[pymodule]
pub fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vector2D>()?;
    Ok(())
}