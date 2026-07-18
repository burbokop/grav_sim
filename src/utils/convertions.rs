use crate::utils::color::Color;
use burbomath::{Point, Rect, Size, Vector};

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

pub const fn into_vger_color(c: Color) -> vger::Color {
    let a = c.a as f32 / 255.;
    let r = c.r as f32 / 255.;
    let g = c.g as f32 / 255.;
    let b = c.b as f32 / 255.;
    vger::Color { r, g, b, a }
}
