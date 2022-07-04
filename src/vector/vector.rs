use nalgebra as na;
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

pub trait Vector {
    fn copy(&self) -> Self;
    fn dot(&self, other: &Self) -> f64;
    fn scale(&self, other: &Self) -> Self;
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

            fn scale(&self, other: &Self) -> Self {
                let mut v = self.v.clone();

                for i in 0..Self::DIMENSIONS {
                    v[i] *= other.v[i];
                }

                Self { v }
            }

            fn length(&self) -> f64 {
                self.v.norm()
            }

            fn normalized(&self) -> Self {
                let v = self.v / self.v.norm();
                Self { v }
            }
        }

        impl ops::Add for $dst {
            type Output = $dst;

            fn add(self, other: $dst) -> Self::Output {
                $dst { v: self.v + other.v }
            }
        }

        impl ops::AddAssign for $dst {
            fn add_assign(&mut self, other: Self) {
                self.v = self.v + other.v;
            }
        }

        impl ops::Sub for $dst {
            type Output = $dst;

            fn sub(self, other: $dst) -> Self::Output {
                $dst { v: self.v - other.v }
            }
        }

        impl ops::Mul<f64> for $dst {
            type Output = $dst;

            fn mul(self, other: f64) -> Self::Output {
                let v = self.v * other;

                $dst { v }
            }
        }

        impl ops::Div<f64> for $dst {
            type Output = $dst;

            fn div(self, other: f64) -> Self::Output {
                let v = self.v / other;

                $dst { v }
            }
        }

        #[pymethods]
        impl $dst {
            pub const SMALL_N: f64 = 1e-8;
            /// copy($self)
            /// --
            ///
            #[doc = concat!("copy(self: ", stringify!($dst), ") -> ", stringify!($dst))]
            #[doc = "make a copy"]
            pub fn copy(&self) -> Self {
                Vector::copy(self)
            }

            /// dot($self, other)
            /// --
            ///
            #[doc = concat!("dot(self: ", stringify!($dst), ", other: ", stringify!($dst), ") -> float")]
            #[doc = "calculate the dot product of two Vectors"]
            pub fn dot(&self, other: &Self) -> f64 {
                Vector::dot(self, &other)
            }


            /// normalized($self)
            /// --
            ///
            #[doc = concat!("normalized(self: ", stringify!($dst), ") -> ", stringify!($dst))]
            #[doc = "get a unit-sized vector"]
            pub fn normalized(&self) -> Self {
                Vector::normalized(&self)
            }

            /// length($self)
            /// --
            ///
            #[doc = concat!("length(self: ", stringify!($dst), ") -> float")]
            #[doc = "get the length of a Vector"]
            pub fn length(&self) -> f64 {
                Vector::length(self)
            }

            pub fn tolist(&self) -> [f64; Self::DIMENSIONS] {
                self.v.into()
            }

            fn __json__(&self) -> [f64; Self::DIMENSIONS] {
                self.v.into()
            }

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
                &self,
                other: PyRef<$dst>,
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

            fn __add__(&self, other: Self) -> Self {
                *self + other
            }

            fn __sub__(&self, other: Self) -> Self {
                *self - other
            }

            fn __mul__(&self, value: f64) -> Self {
                *self * value
            }

            fn __truediv__(&self, value: f64) -> Self {
                *self / value
            }

            fn __getitem__(&self, idx: isize) -> PyResult<f64> {
                match (usize::try_from(idx)) {
                    Ok(index) => {
                        if index < Self::DIMENSIONS {
                            return Ok(self.v[index]);
                        }
                    }
                    Err(_) => {}
                }

                Err(PyIndexError::new_err("index out of range"))
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
    pub fn __new__(v: [f64; 2]) -> Self {
        let v = na::Vector2::new(v[0], v[1]);
        Self { v }
    }

    #[staticmethod]
    pub fn zero() -> Self {
        Self {
            v: na::Vector2::zeros(),
        }
    }

    #[staticmethod]
    pub fn scalar(v: f64) -> Self {
        let v = na::Vector2::new(v, v);
        Self { v }
    }

    /// angle($self)
    /// --
    ///
    /// angle(self: Vector2D) -> float
    /// calculate the angle angle relative to the x-axis and y-axis from a Vector2D
    pub fn angle(&self) -> f64 {
        f64::atan2(self.v[1], self.v[0])
    }

    /// cross($self, other)
    /// --
    ///
    /// cross(self: Vector2D, other: Vector2D) -> float
    /// calculate the cross product of two Vector2D vectors
    pub fn cross(&self, other: &Self) -> f64 {
        self.v[0] * other.v[1] - other.v[0] * self.v[1]
    }
}

#[pymethods]
impl Vector3D {
    pub const DIMENSIONS: usize = 3;
    #[new]
    pub fn __new__(v: [f64; 3]) -> Self {
        let v = na::Vector3::new(v[0], v[1], v[2]);
        Self { v }
    }

    #[staticmethod]
    pub fn zero() -> Self {
        Self {
            v: na::Vector3::zeros(),
        }
    }

    #[staticmethod]
    pub fn scalar(v: f64) -> Self {
        let v = na::Vector3::new(v, v, v);
        Self { v }
    }

    /// cross($self, other)
    /// --
    ///
    /// cross(self: Vector3D, other: Vector3D) -> Vector3D
    /// calculate the cross product of two Vector3D vectors
    pub fn cross(&self, other: &Self) -> Self {
        let v = self.v.cross(&other.v);
        Self { v }
    }
}

#[pyclass]
#[derive(Clone, Copy)]
pub struct CutResult {
    #[pyo3(get)]
    pub ik_1: f64,
    #[pyo3(get)]
    pub ik_2: f64,
    #[pyo3(get)]
    pub point: Vector2D,
}

#[pymethods]
impl CutResult {
    fn __repr__(&self) -> String {
        format!(
            "CutResult: {}/{} ({}, {})",
            self.ik_1, self.ik_2, self.point.v[0], self.point.v[1]
        )
    }
}

pub fn cut_2d(
    l1_p1: &Vector2D,
    l1_p2: &Vector2D,
    l2_p1: &Vector2D,
    l2_p2: &Vector2D,
) -> Option<CutResult> {
    // Line AB represented as a1x + b1y = c1
    let a1 = l1_p2.v[1] - l1_p1.v[1];
    let b1 = l1_p1.v[0] - l1_p2.v[0];
    let c1 = a1 * l1_p1.v[0] + b1 * l1_p1.v[1];

    // Line CD represented as a2x + b2y = c2
    let a2 = l2_p2.v[1] - l2_p1.v[1];
    let b2 = l2_p1.v[0] - l2_p2.v[0];
    let c2 = a2 * l2_p1.v[0] + b2 * l2_p1.v[1];

    let determinant = a1 * b2 - a2 * b1;

    if determinant.abs() < Vector2D::SMALL_N {
        return None;
    } else {
        let x = (b2 * c1 - b1 * c2) / determinant;
        let y = (a1 * c2 - a2 * c1) / determinant;

        let point = Vector2D::__new__([x, y]);

        let diff1 = *l1_p2 - *l1_p1;
        let diff2 = *l2_p2 - *l2_p1;

        Some(CutResult {
            ik_1: (point - *l1_p1).dot(&diff1) / diff1.dot(&diff1),
            ik_2: (point - *l2_p1).dot(&diff2) / diff2.dot(&diff2),
            point,
        })
    }
}
