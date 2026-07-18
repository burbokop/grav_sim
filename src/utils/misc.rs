use crate::utils::convertions::into_vger_vec;
use burbomath::Matrix;
use std::ops::{Add, Div, Mul, Sub};

pub fn apply_transformation(vger: &mut vger::Vger, mat: Matrix<f32>) {
    vger.translate(into_vger_vec(mat.translation()));
    vger.scale(into_vger_vec(mat.scale_vec()));
}

pub fn map_range<X, Y>(val: X, in_min: X, in_max: X, out_min: Y, out_max: Y) -> Y
where
    X: Sub<Output = X> + Div<Output = X> + Clone,
    Y: Sub<Output = Y> + Mul<X, Output = Y> + Add<Output = Y> + Clone,
{
    let t: X = (val - in_min.clone()) / (in_max - in_min);
    (out_max - out_min.clone()) * t + out_min
}

pub(crate) fn for_each_pair_cycled<T: Clone, F: FnMut(T, T)>(
    iter: impl Iterator<Item = T>,
    mut f: F,
) {
    let mut first = None;
    let mut prev = None;
    for i in iter {
        if first.is_none() {
            first = Some(i.clone());
        }

        if let Some(prev) = prev {
            f(prev, i.clone());
        }

        prev = Some(i);
    }

    if let (Some(prev), Some(first)) = (prev, first) {
        f(prev, first);
    }
}
