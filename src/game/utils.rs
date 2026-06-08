use burbomath::{Matrix, Point, Rect, Size, Vector};
use std::ops::{Add, Div, Mul, Sub};

// pub(crate) fn matrix_to_mat3(x: Matrix<f32>) -> Mat3 {
//     let [a, b, c, d, e, f, g, h, i] = x.into();

//     // Mat3::from_cols(
//     //     (a,b,c).into(),
//     //     (d,e,f).into(),
//     //     (g,h,i).into()
//     // )

//     Mat3::from_cols((a, d, g).into(), (b, e, h).into(), (c, f, i).into())
// }

// pub(crate) fn matrix_to_mat4(x: Matrix<f32>) -> Mat4 {
//     let [a, b, c, d, e, f, g, h, i] = x.into();

//     Mat4::from_cols(
//         (a, b, 0., c).into(),
//         (d, e, 0., f).into(),
//         (g, h, i, 0.).into(),
//         (0., 0., 0., 1.).into(),
//     )
//     .transpose()
// }

pub fn into_vger_vec(v: Vector<f32>) -> vger::defs::LocalVector {
    let (x, y) = v.into();
    vger::defs::LocalVector::new(x, y)
}

pub fn into_vger_point(v: Point<f32>) -> vger::defs::LocalPoint {
    let (x, y) = v.into();
    vger::defs::LocalPoint::new(x, y)
}

pub fn into_vger_size(v: Size<f32>) -> vger::defs::LocalSize {
    let (w, h) = v.into();
    vger::defs::LocalSize::new(w, h)
}

pub fn into_vger_rect(v: Rect<f32>) -> vger::defs::LocalRect {
    let (origin, size) = v.into();
    vger::defs::LocalRect::new(into_vger_point(origin), into_vger_size(size))
}

pub fn apply_transformation(vger: &mut vger::Vger, mat: Matrix<f32>) {
    vger.translate(into_vger_vec(mat.translation()));
    vger.scale(into_vger_vec(mat.scale_vec()));
}

pub const fn color_from_hex(c: u32) -> vger::Color {
    let a = (c >> 24) as u8 as f32 / 255.;
    let r = (c >> 16) as u8 as f32 / 255.;
    let g = (c >> 08) as u8 as f32 / 255.;
    let b = (c >> 00) as u8 as f32 / 255.;
    vger::Color { r, g, b, a }
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
