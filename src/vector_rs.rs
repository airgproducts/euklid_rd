use std::convert::TryFrom;
use pyo3::prelude::*;
use pyo3::exceptions::PyIndexError;
use pyo3::class::sequence::PySequenceProtocol;
use pyo3::class::basic::PyObjectProtocol;
use nalgebra as na;

#[pyclass]
struct Vector2D {
    vec: na::Vector2<f64>
}


#[pymethods]
impl Vector2D {
    #[new]
    pub fn __new__(v: [f64; 2]) -> Self {
        Vector2D {vec: na::Vector2::new(v[0], v[1])}
    }
    
    pub fn normalized(&self) -> PyResult<Self> {
        let length = self.vec.norm();

        return Ok(Vector2D {vec: self.vec / length});
    }

    pub fn length(&self) -> PyResult<f64> {
        return Ok(self.vec.norm());
    }
}

#[pyproto]
impl PyObjectProtocol for Vector2D {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Vector2D({:.4} {:.4})", self.vec[0], self.vec[1]))
    }

}

#[pyproto]
impl PySequenceProtocol for Vector2D {
    fn __getitem__(&self, idx: isize) -> PyResult<f64> {
        if idx < 0 || idx > 1 {
            return Err(PyIndexError::new_err("index out of range"));
        }
        let n_us = usize::try_from(idx).unwrap();
        return Ok(self.vec[n_us]);
        
    }

    fn __setitem__(&mut self, idx: isize, value: f64) -> PyResult<()> {
        if idx < 0 || idx > 1 {
            return Err(PyIndexError::new_err("index out of range"));
        }
        let n_us = usize::try_from(idx).unwrap();
        self.vec[n_us] = value;
        
        return Ok({});
    }
}

#[pymodule]
pub fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vector2D>()?;
    Ok(())
}