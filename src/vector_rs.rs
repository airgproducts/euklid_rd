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
    values: na::Vector2<f64>,
}


#[pymethods]
impl Vector2D {
    #[new]
    pub fn __new__(v: [f64; 2]) -> Self {
        let values =  na::Vector2::new(v[0], v[1]);
        Vector2D {values}
    }
    
    pub fn normalized(&self) -> PyResult<Self> {
        let values = self.values / self.values.norm();
        Ok(Vector2D {values})
    }

    pub fn length(&self) -> PyResult<f64> {
        Ok(self.values.norm())
    }
}

#[pyproto]
impl PyObjectProtocol for Vector2D {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Vector2D({:.4} {:.4})", self.values[0], self.values[1]))
    }

}

#[pyproto]
impl PyNumberProtocol for Vector2D {
    fn __add__(lhs: Self, rhs: Self) -> PyResult<Self> {
        Ok(Self {values: lhs.values + rhs.values })
   }

   fn __sub__(lhs: Self, rhs: Self) -> PyResult<Self> {
       Ok(Vector2D {values: lhs.values - rhs.values })
   }

   fn __mul__(lhs: Self, value: f64) -> PyResult<Self> {
       Ok(Vector2D {values: lhs.values * value})
   }

   fn __truediv__(lhs: Self, value: f64) -> PyResult<Self> {
       Ok(Vector2D {values: lhs.values / value})
   }
}

#[pyproto]
impl PySequenceProtocol for Vector2D {
    fn __getitem__(&self, idx: isize) -> PyResult<f64> {
        match idx {
            0 | 1 => {
                let n_us = usize::try_from(idx).unwrap();
                Ok(self.values[n_us])
            }
            _ => Err(PyIndexError::new_err("index out of range"))
        }        
    }

    fn __setitem__(&mut self, idx: isize, value: f64) -> PyResult<()> {
        match idx {
            0 | 1 => {
                let n_us = usize::try_from(idx).unwrap();
                self.values[n_us] = value;
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