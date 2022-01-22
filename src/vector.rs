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


trait Vector {
    fn angle(&self) -> f64;
    fn copy(&self) -> Self;
    fn cross(&self, other: Self) -> f64;
    fn dot(&self, other: &Self) -> f64;
    fn length(&self) -> f64;
    fn normalized(&self) -> Self;
}

macro_rules! vector{($src: ident, $dst: ident) => {
    impl $src for $dst {
        fn angle(&self) -> f64 {
            match self.v.len() {
                2 => f64::atan2(self.v[1], self.v[0]),
                _ => panic!("Not Implemented")
            }
        }

        fn copy(&self) -> Self {
            let v = self.v.clone();
            Self {v}
        }

        fn dot(&self, other: &Self) -> f64 {
            self.v.dot(&other.v)
        }

        fn cross(&self, other: Self) -> f64 {
            match self.v.len() {
                2 => self.v[0] * other.v[1] - other.v[0] * self.v[1],
                _ => panic!("Not Implemented")
            }
        }

        fn length(&self) -> f64 {
            self.v.norm()
        }

        fn normalized(&self) -> Self {
            let v = self.v / self.v.norm();
            Self {v}
        }
    }

    #[pyproto]
    impl PyObjectProtocol for $dst {
        fn __repr__(&self) -> PyResult<String> {
            let temp_string = match self.v.len() {
                2 => format!("Vector2D({:.4} {:.4})", self.v[0], self.v[1]),
                3 => format!("Vector3D({:.4} {:.4} {:.4})", self.v[0], self.v[1], self.v[2]),
                _ => panic!("Not implemented")
            };
            Ok(temp_string)
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
    impl PyNumberProtocol for $dst {
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
    impl PySequenceProtocol for $dst {
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
}}

vector!(Vector, Vector2D);
vector!(Vector, Vector3D);

#[pyclass]
#[derive(Clone, Copy)]
struct Vector2D {
    v: na::Vector2<f64>,
}

#[pyclass]
#[derive(Clone, Copy)]
struct Vector3D {
    v: na::Vector3<f64>,
}

#[pymethods]
impl Vector2D {
    #[new]
    fn __new__(v: [f64; 2]) -> PyResult<Self> {
        let v = na::Vector2::new(v[0], v[1]);
        Ok(Self {v})
    }

    /// angle($self)
    /// --
    ///
    /// This function calculates the angle angle relative to the x-axis and y-axis from a Vector2D.
    fn angle(&self) -> f64 {
        Vector::angle(self)
    }
    
    /// copy($self)
    /// --
    ///
    /// This function copies a Vector2D object.
    fn copy(&self) -> Self {
        Vector::copy(self)
    }

    /// cross($self, other)
    /// --
    ///
    /// This function calculates the cross product of two Vector2D vectors.
    fn cross(&self, other: Self) -> f64 {
        Vector::cross(self, other)
    }

    /// dot($self, other)
    /// --
    ///
    /// This function calculates the dot product of two Vector2D.
    fn dot(&self, other: &Self) -> f64 {
        Vector::dot(self, &other)
    }

    /// normalized($self)
    /// --
    ///
    /// This function calculates a normalized Vector2D.
    fn normalized(&self) -> Self {
        Vector::normalized(&self)
    }

    /// length($self)
    /// --
    ///
    /// This function calculates the length of a Vector2D.
    fn length(&self) -> f64 {
        Vector::length(self)
    }
}

#[pymethods]
impl Vector3D {
    #[new]
    fn __new__(v: [f64; 3]) -> PyResult<Self> {
        let v = na::Vector3::new(v[0], v[1], v[2]);
        Ok(Self {v})
    }

    /// angle($self)
    /// --
    ///
    /// This function calculates the angle angle relative to the x-axis and y-axis from a Vector2D.
    fn angle(&self) -> f64 {
        Vector::angle(self)
    }
    
    /// copy($self)
    /// --
    ///
    /// This function copies a Vector2D object.
    fn copy(&self) -> Self {
        Vector::copy(self)
    }

    /// cross($self, other)
    /// --
    ///
    /// This function calculates the cross product of two Vector2D vectors.
    fn cross(&self, other: Self) -> f64 {
        Vector::cross(self, other)
    }

    /// dot($self, other)
    /// --
    ///
    /// This function calculates the dot product of two Vector2D.
    fn dot(&self, other: &Self) -> f64 {
        Vector::dot(self, &other)
    }

    /// normalized($self)
    /// --
    ///
    /// This function calculates a normalized Vector2D.
    fn normalized(&self) -> Self {
        Vector::normalized(&self)
    }

    /// length($self)
    /// --
    ///
    /// This function calculates the length of a Vector2D.
    pub fn length(&self) -> f64 {
        Vector::length(self)
    }
}

pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pymodule]
    fn vector(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Vector2D>()?;
        m.add_class::<Vector3D>()?;
        Ok(())
    }

    m.add_wrapped(wrap_pymodule!(vector))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("euklid_rs.vector", m.getattr("vector")?)?;
    Ok(())
}