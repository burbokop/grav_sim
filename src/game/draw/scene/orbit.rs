use crate::game::{
    draw::common::draw_fading_ellipse,
    orbit::EllipticOrbit,
    world_object_model::{WOMCelestialBody, WOMRoot},
};
use burbomath::{NonNeg, Pi, Point, Two, Vector};
use core::f32;
use vger::Color;

pub fn draw_elliptic_orbit(
    vger: &mut vger::Vger,
    world: &WOMRoot,
    orbit: &EllipticOrbit,
    color: Color,
    compensatory_scale: f32,
) {
    draw_fading_ellipse(
        vger,
        &orbit.ellipse(),
        orbit.anomaly().radians() / (NonNeg::<f32>::two() * NonNeg::<f32>::pi()),
        color,
        compensatory_scale,
    );
}
