use crate::vector::_vector;
use crate::vector::transform;
use nalgebra as na;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Plane {
    #[pyo3(get, set)]
    pub p0: _vector::Vector3D,

    #[pyo3(get, set)]
    pub x_vector: _vector::Vector3D,

    #[pyo3(get, set)]
    pub y_vector: _vector::Vector3D,

    #[pyo3(get, set)]
    pub normvector: _vector::Vector3D,

    transformation: transform::Transformation,
}

#[pymethods]
impl Plane {
    #[new]
    fn __new__(p0: _vector::Vector3D, v1: _vector::Vector3D, v2: _vector::Vector3D) -> Self {
        let n = v1.cross(&v2);
        let mut matrix = na::Matrix4::<f64>::new(
            0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        );

        for i in 0..3 {
            matrix[(i, 0)] = v1.v[i]; // x
            matrix[(i, 1)] = v2.v[i]; // y
            matrix[(i, 2)] = n.v[i]; // z

            matrix[(i, 3)] = p0.v[i];
        }

        Plane::setup(transform::Transformation::new(matrix))
    }

    #[staticmethod]
    fn setup(transformation: transform::Transformation) -> Self {
        let p0 = transformation.apply(&_vector::Vector3D {
            v: na::Vector3::<f64>::new(0., 0., 0.),
        });

        let x_vector = _vector::Vector3D {
            v: transformation
                .apply(&_vector::Vector3D {
                    v: na::Vector3::<f64>::new(1., 0., 0.),
                })
                .v
                - p0.v,
        };
        let y_vector = _vector::Vector3D {
            v: transformation
                .apply(&_vector::Vector3D {
                    v: na::Vector3::<f64>::new(0., 1., 0.),
                })
                .v
                - p0.v,
        };
        let normvector = _vector::Vector3D {
            v: transformation
                .apply(&_vector::Vector3D {
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
            transformation,
        }
    }

    fn project(&self, vec: _vector::Vector3D) -> _vector::Vector2D {
        let diff = vec - self.p0;

        let x = self.x_vector.dot(&diff);
        let y = self.y_vector.dot(&diff);

        let v = na::Vector2::new(x, y);
        _vector::Vector2D { v }
    }

    fn align(&self, vec: _vector::Vector2D) -> _vector::Vector3D {
        let v = na::Vector3::<f64>::new(vec.v[0], vec.v[1], 0.);
        let vec_3d = _vector::Vector3D { v };

        self.transformation.apply(&vec_3d)
    }
}
