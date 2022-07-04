pub struct BezierBasis {
    bases: Vec<(f64, i32)>,
}

pub trait Basis {
    fn new(size: usize) -> Self;
    fn dimension(&self) -> usize;
    fn get(&mut self, index: usize, x: f64) -> f64;
    fn copy(&self) -> Self;
}

fn choose(n: u64, k: u64) -> u64 {
    // choose n over k
    //
    if k <= n {
        let mut ntok: u64 = 1;
        let mut ktok: u64 = 1;
        let mut n_new = n;
        let range: u64 = k.min(n - k) + 1;

        for t in 1..range {
            ntok *= n_new;
            ktok *= t;
            n_new -= 1;
        }
        return ntok / ktok;
    } else {
        return 0;
    }
}

impl Basis for BezierBasis {
    fn new(size: usize) -> Self {
        let mut bases = Vec::new();

        for i in 0..size as i32 {
            let k = choose((size - 1).try_into().unwrap(), i as u64) as f64;
            bases.push((k, i));
        }

        Self { bases }
    }

    fn copy(&self) -> Self {
        let bases = self.bases.clone();
        Self { bases }
    }

    fn dimension(&self) -> usize {
        self.bases.len()
    }

    fn get(&mut self, index: usize, x: f64) -> f64 {
        let size = self.bases.len();
        if index >= size {
            panic!("invalid index")
        }

        let (k, i) = self.bases[index];

        k * x.powi(i) * (1. - x).powi(size as i32 - 1 - i)
    }
}

pub struct BSplineBasis<const DEGREE: usize> {
    dimension: usize,
    knots: Vec<f64>,
}

impl<const DEGREE: usize> BSplineBasis<DEGREE> {
    fn get_basis(&self, degree: usize, index: usize, x: f64) -> f64 {
        if degree == 0 {
            if self.knots[index] < x && x <= self.knots[index + 1] {
                return 1.;
            }
            return 0.;
        } else {
            let next_basis_1 = self.get_basis(degree - 1, index, x);
            let next_basis_2 = self.get_basis(degree - 1, index + 1, x);

            if index == 0 && x <= 0. {
                return 1.;
            } else {
                let mut result = 0.;

                let t_this = self.knots[index];
                let t_next = self.knots[index + 1];
                let t_precog = self.knots[index + degree];
                let t_horizon = self.knots[index + degree + 1];

                let mut top = x - t_this;
                let mut bottom = t_precog - t_this;

                if bottom != 0. {
                    result = top / bottom * next_basis_1;
                }

                top = t_horizon - x;
                bottom = t_horizon - t_next;

                if bottom > 1e-8 {
                    result += top / bottom * next_basis_2;
                }

                result
            }
        }
    }
}

impl<const DEGREE: usize> Basis for BSplineBasis<DEGREE> {
    fn new(size: usize) -> Self {
        if size < 2 {
            panic!("not enough nodes for bspline");
        }

        // create knots
        let total_knots: usize = size + DEGREE + 1;
        let inner_knots: usize = total_knots - 2 * DEGREE;
        let mut knots = Vec::new();

        for _i in 0..DEGREE {
            knots.push(0.);
        }

        for i in 0..inner_knots {
            knots.push(i as f64 / (inner_knots - 1) as f64);
        }

        for _i in 0..DEGREE {
            knots.push(1.)
        }

        Self {
            knots,
            dimension: size,
        }
    }

    fn copy(&self) -> Self {
        let knots = self.knots.clone();
        Self {
            dimension: self.dimension,
            knots,
        }
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn get(&mut self, index: usize, x: f64) -> f64 {
        self.get_basis(DEGREE, index, x)
    }
}

/*




template<i64 degree>
f64 BSplineBase<degree>::get(i64 index, f64 value) const {
    if (index >= self.bases.size()) {
        throw std::exception();
    }
    return self.bases[index](value);
 }


template<i64 degree>
i64 BSplineBase<degree>::dimension() const {
    return self.bases.size();
}


template class BSplineBase<1>;
template class BSplineBase<2>;
template class BSplineBase<3>;
template class BSplineBase<4>;

} // namespace euklid::spline
*/
