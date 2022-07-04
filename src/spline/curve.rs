use pyo3::prelude::*;

use crate::polyline::PolyLine2D;
use crate::spline::basis::*;
use crate::vector::Vector2D;

macro_rules! define_curve {
    ($dst: ident, $basis: ident) => {
        // use a module-namespace to shadow arg-enums

        // Define PolyLine
        #[pyclass]
        pub struct $dst {
            #[pyo3(get)]
            pub controlpoints: PolyLine2D,

            basis: $basis,
        }

        #[pymethods]
        impl $dst {
            #[new]
            pub fn new(controlpoints: PolyLine2D) -> Self {
                let basis = $basis::new(controlpoints.__len__());

                Self {
                    basis,
                    controlpoints,
                }
            }

            pub fn set_controlpoints(&mut self, controlpoints: PolyLine2D) {
                if self.basis.dimension() != controlpoints.__len__() {
                    self.basis = $basis::new(controlpoints.__len__());
                }
            }

            pub fn get(&mut self, x: f64) -> Vector2D {
                let mut result = Vector2D::zero();

                for i in 0..self.basis.dimension() {
                    let basis_factor = self.basis.get(i, x);

                    if basis_factor > 0. {
                        result += self
                            .controlpoints
                            .__getitem__(i.try_into().unwrap())
                            .unwrap()
                            * basis_factor
                    }
                }

                result
            }

            pub fn get_sequence(&mut self, n: usize) -> PolyLine2D {
                let mut nodes = Vec::new();

                for i in 0..n {
                    nodes.push(self.get(i as f64 / n as f64));
                }

                PolyLine2D { nodes }
            }

            pub fn copy(&self) -> Self {
                Self {
                    basis: self.basis.copy(),
                    controlpoints: self.controlpoints.copy(),
                }
            }
        }
    };
}

define_curve!(BezierCurve, BezierBasis);
type BSpline3Basis = BSplineBasis<3>;
define_curve!(BSplineCurve3, BSpline3Basis);
