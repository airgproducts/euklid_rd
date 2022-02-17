use pyo3::class::sequence::PySequenceProtocol;
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

                for i in 0..self.nodes.len() - 1 {
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

            fn get_length(&self) -> f64 {
                let mut result: f64 = 0.;

                for segment in &self.get_segments() {
                    result += segment.length();
                }

                result
            }

            fn walk(&self, start: f64, distance: f64) -> f64 {
                if f64::abs(distance) < 1e-8 {
                    return start;
                }

                let direction: isize = if (distance < 0.) { -1 } else { 1 };

                let mut next_value = if direction > 0 {
                    start.floor() as isize
                } else {
                    start.ceil() as isize
                };

                if (f64::abs(start - next_value as f64) < 1e-5) {
                    next_value += direction;
                }

                let mut amount = f64::abs(distance);

                let mut current_segment_length =
                    (self.get(next_value as f64) - &self.get(start)).length();
                amount -= current_segment_length;

                let mut last_value = start;

                while (amount > 0.) {
                    if next_value > isize::try_from(self.nodes.len()).unwrap() && direction > 0 {
                        break;
                    }
                    if (next_value < 0 && direction < 0) {
                        break;
                    }

                    last_value = next_value as f64;
                    next_value += direction;

                    current_segment_length =
                        (self.get(next_value as f64) - &self.get(last_value)).length();

                    amount -= current_segment_length;
                }

                return next_value as f64
                    + (direction as f64 * amount) * f64::abs(next_value as f64 - last_value)
                        / current_segment_length;
            }

            fn resample(&self, num_points: usize) -> Self {
                let mut nodes = Vec::new();
                let mut ik = 0.;
                let distance = self.get_length() / ((num_points - 1) as f64);

                nodes.push(self.get(ik));

                for _i in 0..num_points - 2 {
                    ik = self.walk(ik, distance);
                    nodes.push(self.get(ik));
                }

                nodes.push(self.nodes.last().unwrap().copy());

                Self { nodes }
            }
        }

        #[pyproto]
        impl PySequenceProtocol for $dst {
            fn __len__(&self) -> usize {
                self.nodes.len()
            }

            fn __getitem__(&self, idx: isize) -> $vecClass {
                let idx2 = usize::try_from(idx).unwrap();

                self.nodes[idx2]
            }
        }
    };
}

define_polyline!(PolyLine2D, Vector2D);
define_polyline!(PolyLine3D, Vector3D);
