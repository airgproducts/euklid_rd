use crate::polyline::polyline::polyline_2d::{InitArgs, PolyLine2D};
extern crate pyo3;
use crate::vector::vector::*;
use pyo3::prelude::*;

const CUT_TOLERANCE: f64 = 1e-5;

#[pymethods]
impl PolyLine2D {
    fn segment_normals(&self) -> Self {
        let segments = self.get_segments();

        let mut normals = Vec::new();
        normals.reserve(segments.len());

        for segment in &segments {
            let normal = Vector2D::__new__([segment.v[1], -segment.v[0]]).normalized();

            normals.push(normal);
        }

        Self::new(InitArgs::VecList(normals))
    }

    fn normvectors(&self) -> Self {
        let segments = self.get_segments();
        let segment_normals = self.segment_normals().nodes;

        let mut normvectors = Vec::new();

        normvectors.push(segment_normals[0]);

        for i in 0..segment_normals.len() - 1 {
            let normal = segment_normals[i] + segment_normals[i + 1];

            if normal.length() > Vector2D::SMALL_N {
                normvectors.push(normal.normalized());
            } else {
                // n1 and n2 are opposite -> add normalized segment
                let index = usize::max(0, i - 1);
                normvectors.push(segments[index].normalized());
            }
            //let normal = segment_normals[i]->copy();
        }

        normvectors.push(segment_normals.last().unwrap().copy());

        Self::new(InitArgs::VecList(normvectors))
    }

    fn offset_simple(&self, amount: f64) -> Self {
        let normvectors = self.normvectors().nodes;
        let mut nodes = Vec::new();
        nodes.reserve(normvectors.len());

        for i in 0..self.nodes.len() {
            nodes.push(self.nodes[i] + normvectors[i] * amount);
        }

        Self { nodes }
    }

    fn offset(&self, amount: f64) -> Self {
        let segments = self.get_segments();
        let mut segments_normalized = Vec::new();
        segments_normalized.reserve(segments.len());
        let mut result = Vec::new();

        for segment in &segments {
            segments_normalized.push(segment.normalized());
        }

        let segment_normals = self.segment_normals().nodes;
        let mut offset_segments = Vec::new();

        for i in 0..self.nodes.len() - 1 {
            offset_segments.push([
                self.nodes[i] + segment_normals[i] * amount,
                self.nodes[i + 1] + segment_normals[i] * amount,
            ]);
        }

        result.push(self.nodes[0] + segment_normals[0] * amount);

        for i in 0..self.nodes.len() - 2 {
            let segment_1 = &segments_normalized[i];
            let segment_2 = &segments_normalized[i + 1];
            let sin_angle = segment_1.cross(&segment_2);

            if f64::abs(sin_angle) < 0.1 {
                result.push(offset_segments[i][1] + offset_segments[i + 1][0] * 0.5);
            } else if sin_angle * amount > 0. {
                // outside turn
                match cut_2d(
                    &offset_segments[i][0],
                    &offset_segments[i][1],
                    &offset_segments[i + 1][0],
                    &offset_segments[i + 1][1],
                ) {
                    Some(cut) => result.push(cut.point),
                    None => {} // TODO: raise
                }
                // todo: make a circle
            } else {
                // inside turn -> add both and cut later
                result.push(offset_segments[i][1]);
                result.push(offset_segments[i + 1][0]);
            }
        }

        result.push(*self.nodes.last().unwrap() + *segment_normals.last().unwrap() * amount);

        Self { nodes: result }
    }

    fn cut(&self, p1: &Vector2D, p2: &Vector2D) -> Vec<CutResult> {
        let mut results = Vec::new();

        if self.nodes.len() < 2 {
            return results;
        }

        // cut first segment (extrapolate front)
        let mut result = cut_2d(&self.nodes[0], &self.nodes[1], p1, p2);
        let mut last_result = result;

        if let Some(cut) = result {
            if cut.ik_1 <= CUT_TOLERANCE {
                results.push(cut);
            }
        }

        // try all segments
        for i in 0..self.nodes.len() - 1 {
            result = cut_2d(&self.nodes[i], &self.nodes[i + 1], &p1, &p2);

            if let Some(mut cut) = result {
                if CUT_TOLERANCE < cut.ik_1 && cut.ik_1 <= 1. - CUT_TOLERANCE {
                    cut.ik_1 = i as f64 + cut.ik_1;
                    results.push(cut);
                } else if let Some(cut2) = last_result {
                    // catch tolerance values (close to a knot vector)
                    if -CUT_TOLERANCE < cut.ik_1
                        && cut.ik_1 <= CUT_TOLERANCE
                        && 1. - CUT_TOLERANCE < cut2.ik_1
                        && cut2.ik_1 <= 1. + CUT_TOLERANCE
                    {
                        cut.ik_1 = i as f64 + cut.ik_1;
                        results.push(cut);
                    }
                }
                last_result = result;
            }
        }

        if let Some(mut cut) = result {
            // add value if for the last cut ik_1 is greater than 1 (extrapolate end)
            if cut.ik_1 > 1. - CUT_TOLERANCE {
                cut.ik_1 = (self.nodes.len() - 1) as f64 + cut.ik_1;
                results.push(cut);
            }
        }

        results
    }

    fn cut_nearest(&self, p1: &Vector2D, p2: &Vector2D, ik_start: f64) -> PyResult<CutResult> {
        let mut results = self.cut(&p1, &p2);

        if results.len() > 0 {
            results.sort_by(|cut1, cut2| {
                f64::abs(cut1.ik_1 - ik_start)
                    .partial_cmp(&f64::abs(cut2.ik_1 - ik_start))
                    .unwrap()
            });

            //let x = results[0];

            Ok(results.remove(0))
        } else {
            Err(pyo3::exceptions::PyValueError::new_err("no cut found"))
        }
    }

    fn cut_with_polyline(&self, other: &Self) -> Vec<[f64; 2]> {
        let mut result = Vec::new();

        for i in 0..other.nodes.len() - 1 {
            let cuts = self.cut(&other.nodes[i], &other.nodes[i + 1]);

            for cut in cuts {
                if -CUT_TOLERANCE < cut.ik_2
                    && cut.ik_2 < 1. + CUT_TOLERANCE
                    && -CUT_TOLERANCE < cut.ik_1
                    && cut.ik_1 < (self.nodes.len() - 1) as f64 + CUT_TOLERANCE
                {
                    result.push([cut.ik_1, i as f64 + cut.ik_2])
                }
            }
        }

        result
    }

    fn fix_errors(&self) -> Self {
        let length = self.__len__();

        if length < 4 {
            return self.copy();
        }

        // go through all segments
        for start in 0..length - 3 {
            let new_list_start = start + 2;

            let line2 = Self {
                nodes: self.nodes[new_list_start..].to_vec(),
            };
            let line2_length = line2.__len__() as f64;

            let cuts = line2.cut_nearest(&self.nodes[start], &self.nodes[start + 1], line2_length);

            for result in cuts {
                if 0. <= result.ik_1
                    && result.ik_1 < (line2_length - 1 as f64) - CUT_TOLERANCE
                    && 0. <= result.ik_2
                    && result.ik_2 < 1.
                {
                    let mut new_nodes = self.nodes[..start + 1].to_vec();

                    new_nodes.push(line2.get(result.ik_1));

                    let mut start_2 = result.ik_1.ceil();

                    if (result.ik_1 - start_2).abs() < CUT_TOLERANCE {
                        start_2 += 1.;
                    }

                    new_nodes.extend(line2.nodes[start_2 as usize..].to_vec());

                    return Self { nodes: new_nodes }.fix_errors();
                }
            }
        }

        let mut new_nodes = Vec::new();

        // Remove len-0 segment points
        let segments = self.get_segments();
        new_nodes.push(self.nodes[0]);

        for i in 0..segments.len() {
            if segments[i].length() > 1e-6 {
                new_nodes.push(self.nodes[i + 1])
            }
        }

        Self { nodes: new_nodes }
    }

    fn boundary(&self) -> [f64; 4] {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = -f64::INFINITY;
        let mut max_y = -f64::INFINITY;

        for &node in &self.nodes {
            min_x = f64::min(min_x, node.v[0]);
            min_y = f64::min(min_y, node.v[0]);
            max_x = f64::max(max_x, node.v[0]);
            max_y = f64::max(max_y, node.v[0]);
        }

        [min_x, min_y, max_x, max_y]
    }
}

/*


double PolyLine2D::get_area() const {
    double area = 0;
    unsigned int j;

    for (unsigned int i=0; i<self.nodes.size(); i++) {
        j = i+1;
        if (j >= self.nodes.size()) {
            j = 0;
        }

        area += self.nodes[i]->get_item(0) * self.nodes[j]->get_item(1);
        area -= self.nodes[i]->get_item(1) * self.nodes[j]->get_item(0);
    }

    return area/2;
}

bool PolyLine2D::contains(const Vector2D& p1) const {
    // todo check boundary before

    for (let vec: self.boundary()) {
        let diff = Vector2D(1,0);
        //let diff = (*vec-p1);
        if (diff.length() > 1e-3) {
            unsigned int valid_cuts = 0;
            let p2 = p1 + diff*2;

            let cuts = self.cut(p1, p2);

            if (cuts.size() == 0) {
                return false;
            }

            for (let cut: cuts) {
                if (cut.first < 0) {}
                else if (cut.first >= self.nodes.size()-1) {}
                else if (cut.second < 0) {}
                else {
                    valid_cuts += 1;
                }
            }

            return valid_cuts % 2 > 0;

        }

    }

    return false;
}

PolyLine2D PolyLine2D::mirror(Vector2D& p1, Vector2D& p2) const {
    let diff = p1 - p2;
    let normvector = Vector2D(-diff[1], diff[0]).normalized();
    std::vector<std::shared_ptr<Vector2D>> result;

    for (let node: self.nodes) {
        result.push_back(std::make_shared<Vector2D>(
            *node - normvector * (2 * normvector.dot(*node-p1))
        ));
    }

    return PolyLine2D(result);
}


PolyLine2D PolyLine2D::rotate(double radians, Vector2D& origin) const {
    std::vector<std::shared_ptr<Vector2D>> new_nodes;
    let rotation = Rotation2D(radians);

    for (let node: self.nodes) {
        new_nodes.push_back(std::make_shared<Vector2D>(origin + rotation.apply(*node - origin)));
    }

    return PolyLine2D(new_nodes);
}


PolyLine3D PolyLine2D::to_3d() const {
    std::vector<std::shared_ptr<Vector3D>> new_nodes;

    return PolyLine3D();


}
*/
