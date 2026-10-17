use crate::utils::convertions::into_vger_vec;
use burbomath::{
    Angle, Complex, Cos, Ellipse, IsNeg, Matrix, NonNeg, One, Pi, Point, Sin, Sq, Sqrt, Two, Zero,
    lerp, physics::M,
};
use std::ops::{Add, Div, Mul, Neg, Sub};

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

pub fn ellipse_from_apo<T>(
    apoapsis: M<T>,
    periapsis: M<T>,
    periapsis_heading: Angle<T>,
) -> Ellipse<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sq<Output = NonNeg<T>>
        + Neg<Output = T>
        + Cos<Output = T>
        + Sin<Output = T>
        + IsNeg
        + One
        + Two
        + Zero
        + Clone
        + Pi,
    NonNeg<T>: Sqrt<Output = NonNeg<T>>,
{
    let c = apoapsis.0.clone() - periapsis.0.clone();
    let a = (apoapsis.0.clone() + periapsis.0.clone()) / T::two();
    let b = NonNeg::new(a.clone().sq() - c.sq()).ok().unwrap().sqrt();

    let f0 = Point::from((periapsis.0 - a.clone() / T::two(), T::zero()));
    let f1 = Point::from((apoapsis.0 - a.clone() / T::two(), T::zero()));

    let center = lerp(f0.clone(), f1, T::one() / T::two());

    let rot: Complex<T> = (f0 - center.clone()).rotor();

    let rot: Complex<T> =
        rot * Complex::from_polar(T::one(), Angle::from_radians(-T::pi() / T::two()));

    let (x, y) = center.into();
    let (r, i) = rot.into();

    Ellipse::from_raw(x, y, a, b.into_inner(), r, i)
}
