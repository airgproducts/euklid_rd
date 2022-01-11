use std::convert::TryFrom;
use pyo3::prelude::*;
use pyo3::exceptions::PyIndexError;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::class::number::PyNumberProtocol;
use pyo3::class::sequence::PySequenceProtocol;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;
use nalgebra as na;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Vector2D {
    v: na::Vector2<f64>,
}


#[pymethods]
impl Vector2D {
    #[new]
    pub fn __new__(v: [f64; 2]) -> PyResult<Self> {
        let v = na::Vector2::new(v[0], v[1]);
        Ok(Self {v})
    }

    pub fn angle(&self) -> f64 {
        let result = f64::atan2(self.v[1], self.v[0]);
        result
    }
    
    pub fn copy(&self) -> Self {
        let v = self.v.clone();
        Self {v}
    }

    pub fn cross(&self, other: Self) -> f64 {
        let result = self.v[0] * other.v[1] - other.v[0] * self.v[1];
        result
    }

    pub fn dot(&self, other: &Self) -> f64 {
        let result = self.v.dot(&other.v);
        result
    }

    pub fn normalized(&self) -> Self {
        let v = self.v / self.v.norm();
        Self {v}
    }

    pub fn length(&self) -> f64 {
        self.v.norm()
    }
}

#[pyproto]
impl PyObjectProtocol for Vector2D {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Vector2D({:.4} {:.4})", self.v[0], self.v[1]))
    }

    fn __richcmp__(&'p self, other: PyRef<'p, Vector2D>, op: pyo3::basic::CompareOp) -> PyResult<bool> {

        match op {
            pyo3::basic::CompareOp::Eq => Ok(self.v == other.v),
            pyo3::basic::CompareOp::Lt => Ok(self.length() < other.length()),
            pyo3::basic::CompareOp::Le => Ok(self.length() <= other.length()),
            pyo3::basic::CompareOp::Gt => Ok(self.length() > other.length()),
            pyo3::basic::CompareOp::Ge => Ok(self.length() >= other.length()),
            _ => Err(PyNotImplementedError::new_err("Not Implemented")),
        }
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



pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Vector2D>()?;
        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(vector))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.vector", m.getattr("vector")?)?;
    Ok(())
}