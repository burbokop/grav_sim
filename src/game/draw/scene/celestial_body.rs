use crate::game::{orbit::CelestialBody, utils::into_vger_point};
use burbomath::Point;

pub fn draw_celestial_body(vger: &mut vger::Vger, body: &CelestialBody, center: Point<f32>) {
    if center.x().is_finite() && center.y().is_finite() {
        assert!(body.atmosphere_radius.0.is_finite());
        assert!(body.solid_radius.0.is_finite());

        let atmosphere_color = vger.color_paint(body.atmosphere_color);
        let solid_color = vger.color_paint(body.solid_color);

        vger.fill_circle(
            into_vger_point(center),
            body.atmosphere_radius.0,
            atmosphere_color,
        );
        vger.fill_circle(into_vger_point(center), body.solid_radius.0, solid_color);
    }
}
