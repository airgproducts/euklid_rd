use nalgebra as na;
use pyo3::prelude::*;

use crate::vector::_vector;

#[pyclass]
#[derive(Clone, Copy, Debug)]
pub struct Transformation {
    matrix: na::base::Matrix4<f64>,
    inverse: Option<na::base::Matrix4<f64>>,
}

impl Transformation {
    pub fn new(matrix: na::base::Matrix4<f64>) -> Self {
        let inverse = matrix.try_inverse();

        Self { matrix, inverse }
    }
}

#[pymethods]
impl Transformation {
    /// apply($self, other)
    /// --
    ///
    /// apply(self: euklid_rs.vector.Transformation, other: euklid_rs.vector.Vector3D) -> euklid_rs.vector.Vector3D
    /// apply the transformation to a Vector3D
    pub fn apply(&self, vec: &_vector::Vector3D) -> _vector::Vector3D {
        let p = self.matrix.transform_point(&vec.v.into());

        _vector::Vector3D { v: p.coords }
    }

    pub fn apply_inverse(&self, vec: &_vector::Vector3D) -> Option<_vector::Vector3D> {
        match self.inverse {
            Some(inverse) => {
                let p = inverse.transform_point(&vec.v.into());
                Some(_vector::Vector3D { v: p.coords })
            }
            None => None,
        }
    }

    /// chain($self, other)
    /// --
    ///
    /// chain(self: euklid_rs.vector.Transformation, other: euklid_rs.vector.Transformation) -> euklid_rs.vector.Transformation
    /// get a chained transformation
    pub fn chain(&self, other: &Transformation) -> Transformation {
        let matrix = self.matrix * other.matrix;
        Transformation::new(matrix)
    }

    /// translation(vec)
    /// --
    ///
    /// translation(vec: euklid_rs.vector.Vector3D) -> euklid_rs.vector.Transformation
    /// create a translation
    #[staticmethod]
    pub fn translation(vec: &_vector::Vector3D) -> Self {
        //let translation = na::geometry::Translation3::from(vec.v);
        let translation = na::geometry::Translation3::new(vec.v[0], vec.v[1], vec.v[2]);
        let matrix = translation.to_homogeneous();

        Transformation::new(matrix)
    }

    #[staticmethod]
    /// rotation(angle, axis)
    /// --
    ///
    /// rotation(angle: float, axis: euklid_rs.vector.Vector3D) -> euklid_rs.vector.Transformation
    pub fn rotation(angle: f64, axis: &_vector::Vector3D) -> Self {
        let scaled_axis = axis.normalized().v * angle;
        let rotation = na::Rotation3::from_scaled_axis(scaled_axis);
        let matrix = rotation.to_homogeneous();

        Transformation::new(matrix)
    }

    #[staticmethod]
    /// scale(scale)
    /// --
    ///
    /// scale(scale: float) -> euklid_rs.vector.Transformation
    /// create a scaling transformation
    pub fn scale(scale: f64) -> Self {
        let scale3 = na::Scale3::new(scale, scale, scale);
        let matrix = scale3.to_homogeneous();

        Transformation::new(matrix)
    }
}
