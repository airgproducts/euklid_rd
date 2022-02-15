use nalgebra as na;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::class::number::PyNumberProtocol;
use pyo3::class::sequence::PySequenceProtocol;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use std::convert::TryFrom;
use std::ops;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Vector2D {
    pub v: na::Vector2<f64>,
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct Vector3D {
    pub v: na::Vector3<f64>,
}

trait Vector {
    fn copy(&self) -> Self;
    fn dot(&self, other: &Self) -> f64;
    fn length(&self) -> f64;
    fn normalized(&self) -> Self;
}

macro_rules! pyvector {
    ($dst: ident) => {
        impl Vector for $dst {
            fn copy(&self) -> Self {
                let v = self.v.clone();
                Self { v }
            }

            fn dot(&self, other: &Self) -> f64 {
                self.v.dot(&other.v)
            }

            fn length(&self) -> f64 {
                self.v.norm()
            }

            fn normalized(&self) -> Self {
                let v = self.v / self.v.norm();
                Self { v }
            }
        }

        impl ops::Add<&$dst> for $dst {
            type Output = $dst;

            fn add(self, _rhs: &$dst) -> $dst {
                let v = self.v + _rhs.v;

                Self { v }
            }
        }
        impl ops::Sub<&$dst> for $dst {
            type Output = $dst;

            fn sub(self, _rhs: &$dst) -> $dst {
                let v = self.v - _rhs.v;

                Self { v }
            }
        }
        impl ops::Mul<f64> for $dst {
            type Output = $dst;

            fn mul(self, _rhs: f64) -> $dst {
                let v = self.v * _rhs;

                Self { v }
            }
        }

        #[pymethods]
        impl $dst {
            /// copy($self)
            /// --
            ///
            /// This function copies a Vector object.
            pub fn copy(&self) -> Self {
                Vector::copy(self)
            }

            /// dot($self, other)
            /// --
            ///
            /// This function calculates the dot product of two Vectors.
            pub fn dot(&self, other: &Self) -> f64 {
                Vector::dot(self, &other)
            }

            /// normalized($self)
            /// --
            ///
            /// This function calculates a normalized Vector.
            pub fn normalized(&self) -> Self {
                Vector::normalized(&self)
            }

            /// length($self)
            /// --
            ///
            /// This function calculates the length of a Vector.
            pub fn length(&self) -> f64 {
                Vector::length(self)
            }
        }

        #[pyproto]
        impl PyObjectProtocol for $dst {
            fn __repr__(&self) -> PyResult<String> {
                let temp_string = match self.v.len() {
                    2 => format!("Vector2D({:.4} {:.4})", self.v[0], self.v[1]),
                    3 => format!(
                        "Vector3D({:.4} {:.4} {:.4})",
                        self.v[0], self.v[1], self.v[2]
                    ),
                    _ => panic!("Not implemented"),
                };
                Ok(temp_string)
            }

            fn __richcmp__(
                &'p self,
                other: PyRef<'p, $dst>,
                op: pyo3::basic::CompareOp,
            ) -> PyResult<bool> {
                match op {
                    pyo3::basic::CompareOp::Eq => Ok(self.v == other.v),
                    pyo3::basic::CompareOp::Ne => Ok(self.v != other.v),
                    pyo3::basic::CompareOp::Lt => Ok(self.length() < other.length()),
                    pyo3::basic::CompareOp::Le => Ok(self.length() <= other.length()),
                    pyo3::basic::CompareOp::Gt => Ok(self.length() > other.length()),
                    pyo3::basic::CompareOp::Ge => Ok(self.length() >= other.length()),
                }
            }
        }

        #[pyproto]
        impl PyNumberProtocol for $dst {
            fn __add__(lhs: Self, rhs: Self) -> PyResult<Self> {
                Ok(Self { v: lhs.v + rhs.v })
            }

            fn __sub__(lhs: Self, rhs: Self) -> PyResult<Self> {
                Ok(Self { v: lhs.v - rhs.v })
            }

            fn __mul__(lhs: Self, value: f64) -> PyResult<Self> {
                Ok(Self { v: lhs.v * value })
            }

            fn __truediv__(lhs: Self, value: f64) -> PyResult<Self> {
                Ok(Self { v: lhs.v / value })
            }
        }

        #[pyproto]
        impl PySequenceProtocol for $dst {
            fn __getitem__(&self, idx: isize) -> PyResult<f64> {
                match (usize::try_from(idx)) {
                    Ok(index) => {
                        if index < Self::DIMENSIONS {
                            return Ok(self.v[index]);
                        }
                    }
                    Err(_) => {}
                }
                return Err(PyIndexError::new_err("index out of range"));
            }

            fn __setitem__(&mut self, idx: isize, value: f64) -> PyResult<()> {
                struct S(usize, isize);
                match S(Self::DIMENSIONS, idx) {
                    S(2, 0..=2) | S(3, 0..=3) => {
                        let n_us = usize::try_from(idx).unwrap();
                        self.v[n_us] = value;
                        Ok(())
                    }
                    _ => Err(PyIndexError::new_err("index out of range")),
                }
            }
        }
    };
}

pyvector!(Vector2D);
pyvector!(Vector3D);

#[pymethods]
impl Vector2D {
    pub const DIMENSIONS: usize = 2;
    #[new]
    pub fn __new__(v: [f64; 2]) -> PyResult<Self> {
        let v = na::Vector2::new(v[0], v[1]);
        Ok(Self { v })
    }

    /// angle($self)
    /// --
    ///
    /// This function calculates the angle angle relative to the x-axis and y-axis from a Vector2D.
    fn angle(&self) -> f64 {
        f64::atan2(self.v[1], self.v[0])
    }

    /// cross($self, other)
    /// --
    ///
    /// This function calculates the cross product of two Vector2D vectors.
    fn cross(&self, other: &Self) -> f64 {
        self.v[0] * other.v[1] - other.v[0] * self.v[1]
    }
}

#[pymethods]
impl Vector3D {
    pub const DIMENSIONS: usize = 3;
    #[new]
    pub fn __new__(v: [f64; 3]) -> PyResult<Self> {
        let v = na::Vector3::new(v[0], v[1], v[2]);
        Ok(Self { v })
    }

    /// cross($self, other)
    /// --
    ///
    /// This function calculates the cross product of two Vector3D vectors.
    fn cross(&self, other: &Self) -> Self {
        let v = self.v.cross(&other.v);
        Self { v }
    }
}
