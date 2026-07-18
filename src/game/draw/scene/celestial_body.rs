use crate::{
    game::world_object_model::WOMCelestialBody,
    utils::convertions::{into_vger_color, into_vger_point},
};
use burbomath::{Point, Vector};

fn draw_celestial_body(vger: &mut vger::Vger, body: &WOMCelestialBody, center: Point<f32>) {
    if center.x().is_finite() && center.y().is_finite() {
        assert!(body.atmosphere_thickness.0.is_finite());
        assert!(body.solid_radius.0.is_finite());

        let atmosphere_color = vger.color_paint(into_vger_color(body.atmosphere_color));
        let solid_color = vger.color_paint(into_vger_color(body.solid_color));

        vger.fill_circle(
            into_vger_point(center),
            body.solid_radius.0 + body.atmosphere_thickness.0,
            atmosphere_color,
        );
        vger.fill_circle(into_vger_point(center), body.solid_radius.0, solid_color);
    }
}

pub fn draw_celestial_body_recursive(
    vger: &mut vger::Vger,
    body: &WOMCelestialBody,
    center: Point<f32>,
) {
    draw_celestial_body(vger, body, center);

    for satellite in &body.satellites {
        draw_celestial_body_recursive(
            vger,
            &satellite.body,
            center + Vector::from_polar(satellite.orbit_radius.0, satellite.orbit_angle),
        );
    }
}
