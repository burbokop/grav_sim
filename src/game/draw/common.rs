use crate::utils::{
    convertions::into_vger_point,
    misc::{for_each_pair_cycled, map_range},
};
use burbomath::{Angle, Complex, Ellipse, NonNeg, Point, Vector};
use core::f32;
use std::{f32::consts::PI, time::Duration};
use vger::{Color, PaintIndex};

pub(crate) fn draw_circle(
    vger: &mut vger::Vger,
    center: Point<f32>,
    radius: f32,
    stroke_width: f32,
    color: PaintIndex,
) {
    vger.stroke_arc(
        into_vger_point(center),
        radius,
        stroke_width,
        0.,
        f32::consts::PI,
        color,
    );
}

pub(crate) fn draw_text(
    vger: &mut vger::Vger,
    origin: Point<f32>,
    text: &str,
    font_size: u32,
    color: Color,
    max_width: Option<f32>,
) {
    vger.save();
    vger.translate(vger::defs::LocalVector::new(*origin.x(), *origin.y()));
    vger.text(text, font_size, color, max_width);
    vger.restore();
}

pub(crate) fn draw_text_centered(
    vger: &mut vger::Vger,
    center: Point<f32>,
    text: &str,
    font_size: u32,
    color: Color,
    max_width: Option<f32>,
) {
    let bounds = vger.text_bounds(text, font_size, max_width);

    vger.save();
    vger.translate(vger::defs::LocalVector::new(
        *center.x() - bounds.width() / 2.,
        *center.y() + bounds.height() / 2.,
    ));
    vger.text(text, font_size, color, max_width);
    vger.restore();
}

pub(crate) fn draw_colored_points(
    vger: &mut vger::Vger,
    points: impl Iterator<Item = (Point<f32>, Color)>,
    stroke_width: f32,
) {
    for_each_pair_cycled(points, |(p0, c), (p1, _)| {
        let paint_index = vger.color_paint(c);
        vger.stroke_segment(
            into_vger_point(p0),
            into_vger_point(p1),
            stroke_width,
            paint_index,
        )
    })
}

/// t - from 0 to 1
pub(crate) fn draw_fading_ellipse(
    vger: &mut vger::Vger,
    ellipse: &Ellipse<f32>,
    t: NonNeg<f32>,
    color: Color,
    compensatory_scale: f32,
) {
    if ellipse.x().is_finite() && ellipse.y().is_finite() {
        assert!(ellipse.a().is_finite());
        assert!(ellipse.b().is_finite());
        assert!(ellipse.r().is_finite());
        assert!(ellipse.i().is_finite());
        assert!(t.into_inner().is_finite());
        assert!(compensatory_scale.is_finite());

        let radius_x = *ellipse.a();
        let radius_y = *ellipse.b();

        let num_points: usize = 1000; // Resolution of the ellipse

        // Generate points and colors
        let points = (0..=num_points).map(|i| {
            // Angle from 0 to 2*PI
            let angle =
                Angle::from_radians(map_range(i as f32, 0., num_points as f32, 0.0, PI * 2.0));

            // Ellipse formula

            let pos = Complex::from_uneven_polar((radius_x, radius_y).into(), angle)
                * Complex::from_cartesian(*ellipse.r(), *ellipse.i())
                * Complex::from_cartesian(0., 1.)
                + Complex::from_cartesian(*ellipse.x(), *ellipse.y());

            // Color changes with angle (0.0 to 1.0)
            // [See Nannou HSL color documentation](https://docs.rs)
            let point_time = i as f32 / num_points as f32;
            let time = (-t.into_inner() - point_time).rem_euclid(1.);
            let color = Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: time,
            };

            (Point::from((*pos.real(), *pos.imag())), color)
        });

        draw_colored_points(vger, points, 1. * compensatory_scale);
    }
}

pub fn draw_vector_with_icon(
    vger: &mut vger::Vger,
    icon: fn(
        vger: &mut vger::Vger,
        center: Point<f32>,
        color: PaintIndex,
        radius: f32,
        compensatory_scale: f32,
        duration_since_start: Duration,
    ),
    position: Point<f32>,
    vec: Vector<f32>,
    color: PaintIndex,
    compensatory_scale: f32,
    duration_since_start: Duration,
) {
    if position.x().is_finite()
        && position.y().is_finite()
        && vec.x().is_finite()
        && vec.y().is_finite()
    {
        let icon_radius = 8. * compensatory_scale;

        let icon_radius = f32::min(vec.len().into_inner() / 2., icon_radius);

        let point0 = position;
        let point1 = position + vec - vec.norm() * icon_radius;

        assert!(point0.x().is_finite());
        assert!(point0.y().is_finite());
        assert!(point1.x().is_finite());
        assert!(point1.y().is_finite());
        assert!(compensatory_scale.is_finite());

        vger.stroke_segment(
            into_vger_point(point0),
            into_vger_point(point1),
            1. * compensatory_scale,
            color,
        );

        icon(
            vger,
            position + vec,
            color,
            icon_radius / compensatory_scale,
            compensatory_scale,
            duration_since_start,
        );
    }
}
