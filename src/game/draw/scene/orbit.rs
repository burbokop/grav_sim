use crate::game::{draw::common::draw_fading_ellipse, orbit::EllipticOrbit};
use core::f32;
use vger::Color;

pub fn draw_elliptic_orbit(
    vger: &mut vger::Vger,
    orbit: &EllipticOrbit,
    color: Color,
    compensatory_scale: f32,
) {
    draw_fading_ellipse(
        vger,
        &orbit.ellipse(),
        orbit.anomaly(),
        color,
        compensatory_scale,
    );
}
