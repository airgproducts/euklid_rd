use pyo3::prelude::*;

use crate::vector::vector::*;

const small_d: f64 = 1e-10;

macro_rules! define_polyline {
    ($dst: ident, $vecClass: ident) => {
        #[pyclass]
        pub struct $dst {
            nodes: Vec<$vecClass>,
        }

        #[pymethods]
        impl $dst {
            #[new]
            fn __new__(nodes: Vec<$vecClass>) -> PyResult<Self> {
                Ok(Self { nodes })
            }

            #[staticmethod]
            fn from_list(lst: Vec<[f64; $vecClass::DIMENSIONS]>) -> PyResult<Self> {
                let mut nodes = Vec::new();

                for coords in lst {
                    let vec = $vecClass::__new__(coords).unwrap();
                    nodes.push(vec)
                }
                Ok(Self { nodes })
            }

            fn tolist(&self) -> Vec<[f64; 2]> {
                let mut result = Vec::new();
                result.reserve(self.nodes.len());

                for node in &self.nodes {
                    result.push([node.v[0], node.v[1]])
                }

                result
            }

            fn get(&self, ik: f64) -> $vecClass {
                let ik_floor = ik.floor() as i32;
                let mut i = match usize::try_from(ik_floor) {
                    Ok(val) => val,
                    Err(_) => 0,
                };

                let node_num = self.nodes.len();

                let diff: $vecClass;

                // catch direct (int) values
                if f64::abs(ik - i as f64) < 1e-10 && 0. <= ik && ik < node_num as f64 {
                    return self.nodes[i];
                }

                if i >= node_num - 1 {
                    i = node_num - 1;
                    diff = self.nodes[i] - &self.nodes[i - 1];
                } else {
                    diff = self.nodes[i + 1] - &self.nodes[i];
                }

                let k: f64 = ik - i as f64;
                let p1 = self.nodes[i];

                p1 + &(diff * k)
            }

            fn get_positions(&self, ik_start: f64, ik_end: f64) -> Vec<f64> {
                let mut result = Vec::new();
                let mut direction: isize = 1;
                let mut forward = true;

                if ik_end < ik_start {
                    direction = -1;
                    forward = false;
                }

                // add first point
                result.push(ik_start as f64);

                let ik_start_floor = ik_start.floor() as isize;
                let mut ik = isize::max(ik_start_floor, 0);

                ik = isize::min(ik, (self.nodes.len() - 2) as isize);

                if forward {
                    ik += 1;
                }

                // todo: maybe check the length diff?
                if f64::abs(ik_start - ik as f64) < 1e-8 {
                    ik += direction;
                }

                while direction as f64 * (ik_end - ik as f64) > 1e-8
                    && 0 < ik
                    && ik < self.nodes.len() as isize - 1
                {
                    result.push(ik as f64);
                    ik += direction;
                }

                result.push(ik_end);

                return result;
            }

            fn get_section(&self, ik_start: f64, ik_end: f64) -> Self {
                let positions = self.get_positions(ik_start, ik_end);

                let mut nodes = Vec::new();
                nodes.reserve(positions.len());

                for ik in positions {
                    nodes.push(self.get(ik));
                }

                Self { nodes }
            }

            fn get_segments(&self) -> Vec<$vecClass> {
                let mut result = Vec::new();
                result.reserve(self.nodes.len() - 1);

                for i in 0..self.nodes.len() {
                    result.push(self.nodes[i + 1] - &self.nodes[i]);
                }

                result
            }

            fn get_tangents(&self) -> Vec<$vecClass> {
                let mut result = Vec::new();

                if self.nodes.len() < 2 {
                    return result;
                }

                result.push((self.nodes[1] - &self.nodes[0]).normalized());

                for i in 0..self.nodes.len() - 2 {
                    let first = (self.nodes[i + 1] - &self.nodes[i]).normalized();
                    let second = (self.nodes[i + 2] - &self.nodes[i + 1]).normalized();
                    let tangent = first + &second;

                    let length = tangent.length();

                    if length <= small_d {
                        result.push(first);
                    } else {
                        result.push(tangent);
                    }
                }

                result.push(
                    (self.nodes[self.nodes.len() - 1] - &self.nodes[self.nodes.len() - 2])
                        .normalized(),
                );

                result
            }
        }
    };
}

define_polyline!(PolyLine2D, Vector2D);
define_polyline!(PolyLine3D, Vector3D);
