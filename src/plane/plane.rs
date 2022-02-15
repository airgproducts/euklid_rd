use crate::vector::transform;
use crate::vector::vector;
use nalgebra as na;
use pyo3::class::basic::PyObjectProtocol;
use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
#[pyclass]
#[derive(Clone, Copy)]
pub struct Plane {
    pub p0: vector::Vector3D,
    pub x_vector: vector::Vector3D,
    pub y_vector: vector::Vector3D,
    pub normvector: vector::Vector3D,
}

#[pymethods]
impl Plane {
    #[new]
    fn __new__(p0: vector::Vector3D, v1: vector::Vector3D, v2: vector::Vector3D) -> PyResult<Self> {
        let n = v1.cross(&v2);
        let mut transformation = transform::Transformation {
            matrix: na::Matrix4::<f64>::new(
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            ),
        };

        for i in 0..3 {
            transformation.matrix[(i, 0)] = v1.v[i]; // x
            transformation.matrix[(i, 1)] = v2.v[i]; // y
            transformation.matrix[(i, 2)] = n.v[i]; // z

            transformation.matrix[(3, i)] = p0.v[i];
        }
        Ok(Plane::setup(transformation))
    }

    #[staticmethod]
    fn setup(transformation: transform::Transformation) -> Self {
        let p0 = transformation.apply(&vector::Vector3D {
            v: na::Vector3::<f64>::new(0., 0., 0.),
        });
        let x_vector = vector::Vector3D {
            v: transformation
                .apply(&vector::Vector3D {
                    v: na::Vector3::<f64>::new(1., 0., 0.),
                })
                .v
                - p0.v,
        };
        let y_vector = vector::Vector3D {
            v: transformation
                .apply(&vector::Vector3D {
                    v: na::Vector3::<f64>::new(0., 1., 0.),
                })
                .v
                - p0.v,
        };
        let normvector = vector::Vector3D {
            v: transformation
                .apply(&vector::Vector3D {
                    v: na::Vector3::<f64>::new(0., 0., 1.),
                })
                .v
                - p0.v,
        };
        Self {
            p0,
            x_vector,
            y_vector,
            normvector,
        }
    }

    fn project(&self, vec: vector::Vector3D) -> vector::Vector2D {
        let diff = vec - self.p0;

        let x = self.x_vector.dot(&diff);
        let y = self.y_vector.dot(&diff);

        let v = na::Vector2::new(x, y);
        vector::Vector2D { v }
    }
}

#[pyproto]
impl PyObjectProtocol for Plane {
    fn __getattr__(&self, name: &str) -> PyResult<String> {
        let temp_string = match name {
            "p0" => self.p0.__repr__(),
            "x_vector" => self.x_vector.__repr__(),
            "y_vector" => self.y_vector.__repr__(),
            "normvector" => self.normvector.__repr__(),
            _ => Err(PyAttributeError::new_err("attribute does not exits")),
        };
        Ok(temp_string.unwrap())
    }
}
