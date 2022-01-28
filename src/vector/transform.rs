use nalgebra as na;
use pyo3::prelude::*;

use crate::vector::vector;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Transformation {
    matrix: na::base::Matrix4<f64>,
}

#[pymethods]
impl Transformation {
    /// apply($self, other)
    /// --
    ///
    /// apply(self: euklid_rs.vector.Transformation, other: euklid_rs.vector.Vector3D) -> euklid_rs.vector.Vector3D
    /// apply the transformation to a Vector3D
    pub fn apply(&self, vec: &vector::Vector3D) -> vector::Vector3D {
        let mut v = na::Vector3::<f64>::new(0., 0., 0.);

        for i in 0..3 {
            let mut value: f64 = self.matrix[(i, 3)];

            for j in 0..3 {
                value += self.matrix[(i, j)] * vec.v[j];
            }

            v[i] = value;
        }

        vector::Vector3D { v }
    }

    /// chain($self, other)
    /// --
    ///
    /// chain(self: euklid_rs.vector.Transformation, other: euklid_rs.vector.Transformation) -> euklid_rs.vector.Transformation
    /// get a chained transformation
    pub fn chain(&self, other: &Transformation) -> Transformation {
        let matrix = self.matrix * other.matrix;
        Transformation { matrix }
    }

    /// translation(vec)
    /// --
    ///
    /// translation(vec: euklid_rs.vector.Vector3D) -> euklid_rs.vector.Transformation
    /// create a translation
    #[staticmethod]
    pub fn translation(vec: &vector::Vector3D) -> Self {
        //let translation = na::geometry::Translation3::from(vec.v);
        let translation = na::geometry::Translation3::new(vec.v[0], vec.v[1], vec.v[2]);
        let matrix = translation.to_homogeneous();

        Self { matrix }
    }

    #[staticmethod]
    /// rotation(angle, axis)
    /// --
    ///
    /// rotation(angle: float, axis: euklid_rs.vector.Vector3D) -> euklid_rs.vector.Transformation
    pub fn rotation(angle: f64, axis: &vector::Vector3D) -> Self {
        let scaled_axis = axis.normalized().v * angle;
        let rotation = na::Rotation3::from_scaled_axis(scaled_axis);
        let matrix = rotation.to_homogeneous();

        Self { matrix }
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

        Self { matrix }
    }
}
