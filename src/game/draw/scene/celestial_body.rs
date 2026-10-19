use std::ops::Deref;

use crate::{
    game::{draw::common::draw_fading_ellipse, world_object_model::WOMCelestialBody},
    utils::convertions::{into_vger_color, into_vger_point},
};
use burbomath::Point;

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
    f0: Point<f32>,
    compensatory_scale: f32,
) {
    draw_celestial_body(vger, body, f0);

    for satellite in body.satellites.borrow().deref() {
        draw_fading_ellipse(
            vger,
            &satellite.orbit.ellipse(),
            satellite.orbit.anomaly(),
            into_vger_color(satellite.body.solid_color),
            compensatory_scale,
        );

        draw_celestial_body_recursive(
            vger,
            &satellite.body,
            satellite.orbit.position().absolute(f0),
            compensatory_scale,
        );
    }
}
