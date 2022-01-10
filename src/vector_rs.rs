use std::convert::TryFrom;
use pyo3::prelude::*;
use pyo3::exceptions::PyIndexError;
use pyo3::class::number::PyNumberProtocol;
use pyo3::class::sequence::PySequenceProtocol;
use pyo3::class::basic::PyObjectProtocol;
use nalgebra as na;

#[pyclass]
#[derive(Clone, Copy)]
struct Vector2D {
    v: na::Vector2<f64>,
}


#[pymethods]
impl Vector2D {
    #[new]
    pub fn __new__(v: [f64; 2]) -> Self {
        let v =  na::Vector2::new(v[0], v[1]);
        Self {v}
    }
    
    pub fn normalized(&self) -> PyResult<Self> {
        let v = self.v / self.v.norm();
        Ok(Self {v})
    }

    pub fn length(&self) -> PyResult<f64> {
        Ok(self.v.norm())
    }
}

#[pyproto]
impl PyObjectProtocol for Vector2D {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Vector2D({:.4} {:.4})", self.v[0], self.v[1]))
    }

}

#[pyproto]
impl PyNumberProtocol for Vector2D {
    fn __add__(lhs: Self, rhs: Self) -> PyResult<Self> {
        Ok(Self {v: lhs.v + rhs.v })
   }

   fn __sub__(lhs: Self, rhs: Self) -> PyResult<Self> {
       Ok(Self {v: lhs.v - rhs.v })
   }

   fn __mul__(lhs: Self, value: f64) -> PyResult<Self> {
       Ok(Self {v: lhs.v * value})
   }

   fn __truediv__(lhs: Self, value: f64) -> PyResult<Self> {
       Ok(Self {v: lhs.v / value})
   }
}

#[pyproto]
impl PySequenceProtocol for Vector2D {
    fn __getitem__(&self, idx: isize) -> PyResult<f64> {
        match idx {
            0 | 1 => {
                let n_us = usize::try_from(idx).unwrap();
                Ok(self.v[n_us])
            }
            _ => Err(PyIndexError::new_err("index out of range"))
        }        
    }

    fn __setitem__(&mut self, idx: isize, value: f64) -> PyResult<()> {
        match idx {
            0 | 1 => {
                let n_us = usize::try_from(idx).unwrap();
                self.v[n_us] = value;
                Ok(())
            }
            _ => Err(PyIndexError::new_err("index out of range"))
        }
    }
}

#[pymodule]
pub fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vector2D>()?;
    Ok(())
}