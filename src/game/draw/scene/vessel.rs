use crate::game::{
    draw::{
        common::draw_vector_with_icon,
        palette::Palette,
        ui::icons::{draw_heading_icon, draw_prograde_icon},
    },
    orbit::EllipticOrbit,
    vessel::Vessel,
    world_object_model::WOMCelestialBody,
};
use std::time::Duration;

pub fn draw_vessel(
    vger: &mut vger::Vger,
    palette: &Palette,
    vessel: &Vessel,
    vessel_parent_body: &WOMCelestialBody,
    orbit: &EllipticOrbit,
    gravitational_constant: f32,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    if orbit.ellipse().x().is_finite() && orbit.ellipse().y().is_finite() {
        assert!(orbit.ellipse().a().is_finite());
        assert!(orbit.ellipse().b().is_finite());
        assert!(orbit.ellipse().r().is_finite());
        assert!(orbit.ellipse().i().is_finite());
    }

    let p = orbit.ellipse().point_on_ellipse(orbit.anomaly());
    if !(p.x().is_finite() && p.y().is_finite()) {
        return;
    }

    let vel = orbit.tangential_velocity(vessel_parent_body, gravitational_constant);

    assert!(vel.x().is_finite());
    assert!(vel.y().is_finite());

    draw_vector_with_icon(
        vger,
        draw_heading_icon,
        p,
        vessel.kinematic_body.heading(),
        palette.ui_stroke(),
        compensatory_scale,
        duration_since_start,
    );

    draw_vector_with_icon(
        vger,
        draw_prograde_icon,
        p,
        vel,
        palette.ui_prograde_retrograde(),
        compensatory_scale,
        duration_since_start,
    );
}
