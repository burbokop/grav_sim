use burbomath::{Angle, Point, Vector};
use std::time::Duration;
use vger::PaintIndex;

use crate::game::{draw::common::draw_circle, utils::into_vger_point};

pub fn draw_flickering_circle(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    let alpha = ((duration_since_start.as_secs_f32() * 4.).cos() + 1.) / 2.;

    if alpha > 0. {
        vger.fill_circle(into_vger_point(center), radius * compensatory_scale, color);
    }
}

pub fn draw_prograde_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    _duration_since_start: Duration,
) {
    draw_circle(
        vger,
        center,
        radius * compensatory_scale,
        1. * compensatory_scale,
        color,
    );

    vger.fill_circle(
        into_vger_point(center),
        radius / 4. * compensatory_scale,
        color,
    );
}

pub fn draw_active_prograde_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    draw_flickering_circle(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    );

    draw_prograde_icon(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    )
}

pub fn draw_retrograde_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    _duration_since_start: Duration,
) {
    let radius_sqrt = (4. * radius).sqrt();

    draw_circle(
        vger,
        center,
        radius * compensatory_scale,
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() + radius_sqrt, center.y() + radius_sqrt),
        (center.x() - radius_sqrt, center.y() - radius_sqrt),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() + radius_sqrt, center.y() - radius_sqrt),
        (center.x() - radius_sqrt, center.y() + radius_sqrt),
        1. * compensatory_scale,
        color,
    );
}

pub fn draw_active_retrograde_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    draw_flickering_circle(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    );

    draw_retrograde_icon(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    )
}

pub fn draw_radial_in_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    _duration_since_start: Duration,
) {
    let radius_sqrt = (4. * radius).sqrt();
    let inner_radius_sqrt = (radius / 2.).sqrt();

    draw_circle(
        vger,
        center,
        radius * compensatory_scale,
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() + radius_sqrt, center.y() + radius_sqrt),
        (
            center.x() + inner_radius_sqrt,
            center.y() + inner_radius_sqrt,
        ),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (
            center.x() - inner_radius_sqrt,
            center.y() - inner_radius_sqrt,
        ),
        (center.x() - radius_sqrt, center.y() - radius_sqrt),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() + radius_sqrt, center.y() - radius_sqrt),
        (
            center.x() + inner_radius_sqrt,
            center.y() - inner_radius_sqrt,
        )
            .into(),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (
            center.x() - inner_radius_sqrt,
            center.y() + inner_radius_sqrt,
        ),
        (center.x() - radius_sqrt, center.y() + radius_sqrt),
        1. * compensatory_scale,
        color,
    );
}

pub fn draw_active_radial_in_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    draw_flickering_circle(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    );

    draw_radial_in_icon(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    )
}

pub fn draw_radial_out_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    _duration_since_start: Duration,
) {
    let radius_sqrt = (4. * radius).sqrt();
    let outer_radius_sqrt = (8. * radius).sqrt();

    draw_circle(
        vger,
        center,
        radius * compensatory_scale,
        1. * compensatory_scale,
        color,
    );

    vger.fill_circle(
        into_vger_point(center),
        radius / 4. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() + radius_sqrt, center.y() + radius_sqrt).into(),
        (
            center.x() + outer_radius_sqrt,
            center.y() + outer_radius_sqrt,
        ),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (
            center.x() - outer_radius_sqrt,
            center.y() - outer_radius_sqrt,
        ),
        (center.x() - radius_sqrt, center.y() - radius_sqrt),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() + radius_sqrt, center.y() - radius_sqrt),
        (
            center.x() + outer_radius_sqrt,
            center.y() - outer_radius_sqrt,
        ),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (
            center.x() - outer_radius_sqrt,
            center.y() + outer_radius_sqrt,
        ),
        (center.x() - radius_sqrt, center.y() + radius_sqrt),
        1. * compensatory_scale,
        color,
    );
}

pub fn draw_active_radial_out_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    draw_flickering_circle(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    );

    draw_radial_out_icon(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    )
}

pub fn draw_maneuver_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    _duration_since_start: Duration,
) {
    // let radius_sqrt_x = (3_f32).sqrt() * 2. * radius;
    // let radius_sqrt_y = 1./2. * radius;
    // let inner_radius_sqrt_x = (3_f32).sqrt() * 2. * radius / 2.;
    // let inner_radius_sqrt_y = 1./2. * radius / 2. ;

    let outer0 = Vector::from_polar(radius * 1.5, Angle::from_degrees(30_f32));
    let outer1 = Vector::from_polar(radius * 1.5, Angle::from_degrees(150_f32));

    let inner0 = Vector::from_polar(radius / 2., Angle::from_degrees(30_f32));
    let inner1 = Vector::from_polar(radius / 2., Angle::from_degrees(150_f32));

    draw_circle(
        vger,
        center,
        radius * compensatory_scale,
        1. * compensatory_scale,
        color,
    );

    vger.fill_circle(
        into_vger_point(center),
        radius / 4. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (*center.x(), center.y() + radius * 1.5),
        (*center.x(), center.y() + radius / 2.),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() - inner0.x(), center.y() - inner0.y()),
        (center.x() - outer0.x(), center.y() - outer0.y()),
        1. * compensatory_scale,
        color,
    );

    vger.stroke_segment(
        (center.x() - inner1.x(), center.y() - inner1.y()),
        (center.x() - outer1.x(), center.y() - outer1.y()),
        1. * compensatory_scale,
        color,
    );
}

pub fn draw_active_maneuver_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    draw_flickering_circle(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    );

    draw_maneuver_icon(
        vger,
        center,
        color,
        radius,
        compensatory_scale,
        duration_since_start,
    )
}

pub fn draw_heading_icon(
    vger: &mut vger::Vger,
    center: Point<f32>,
    color: PaintIndex,
    radius: f32,
    compensatory_scale: f32,
    _duration_since_start: Duration,
) {
    vger.fill_circle(
        into_vger_point(center),
        radius / 4. * compensatory_scale,
        color,
    );
}
