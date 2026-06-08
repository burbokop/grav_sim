use crate::game::{
    draw::{
        common::{draw_circle, draw_text, draw_text_centered, draw_vector_with_icon},
        palette::{
            Palette, UI_MANEUVER_COLOR, UI_NEUTRAL_TEXT_COLOR, UI_PROGRADE_RETROGRADE_COLOR,
            UI_STROKE_COLOR, UI_TODO_COLOR,
        },
        ui::icons::{
            draw_active_maneuver_icon, draw_active_prograde_icon, draw_active_radial_in_icon,
            draw_active_radial_out_icon, draw_active_retrograde_icon, draw_maneuver_icon,
            draw_prograde_icon, draw_radial_in_icon, draw_radial_out_icon, draw_retrograde_icon,
        },
    },
    event_handler::AutoRotationTarget,
    utils::{into_vger_point, into_vger_rect},
};
use burbomath::{NonNeg, Rect, Vector, time::RelativeDuration};
use std::time::Duration;
use vger::Color;

pub struct NavCircleData {
    pub prograde: Vector<f32>,
    pub retrograde: Vector<f32>,
    pub radial_in: Vector<f32>,
    pub radial_out: Vector<f32>,
    pub maneuver: Option<Vector<f32>>,
    pub auto_rotation_mode: Option<AutoRotationTarget>,
}

pub fn draw_nav_circle(
    vger: &mut vger::Vger,
    palette: &Palette,
    bb: Rect<f32>,
    data: &NavCircleData,
    duration_since_start: Duration,
) {
    let center = bb.center();

    let radius = f32::min(*bb.w(), *bb.h()) / 2.;

    vger.fill_circle(into_vger_point(center), radius, palette.ui_background());

    draw_circle(vger, center, radius, 1., palette.ui_background());

    vger.stroke_segment(
        (bb.left(), *bb.center().y()),
        (bb.right(), *bb.center().y()),
        1.,
        palette.ui_stroke(),
    );

    vger.stroke_segment(
        (*bb.center().x(), bb.top()),
        (*bb.center().x(), bb.bottom()),
        1.,
        palette.ui_stroke(),
    );

    vger.stroke_segment(
        (*bb.center().x(), *bb.center().y()),
        (
            *bb.center().x() + bb.w() / 8.,
            *bb.center().y() - bb.h() / 8.,
        ),
        1.,
        palette.ui_stroke(),
    );

    vger.stroke_segment(
        (*bb.center().x(), *bb.center().y()),
        (
            *bb.center().x() - bb.w() / 8.,
            *bb.center().y() - bb.h() / 8.,
        ),
        1.,
        palette.ui_stroke(),
    );

    draw_vector_with_icon(
        vger,
        if data.auto_rotation_mode == Some(AutoRotationTarget::Prograde) {
            draw_active_prograde_icon
        } else {
            draw_prograde_icon
        },
        center,
        data.prograde.norm() * radius,
        palette.ui_prograde_retrograde(),
        1.,
        duration_since_start,
    );

    draw_vector_with_icon(
        vger,
        if data.auto_rotation_mode == Some(AutoRotationTarget::Retrograde) {
            draw_active_retrograde_icon
        } else {
            draw_retrograde_icon
        },
        center,
        data.retrograde.norm() * radius,
        palette.ui_prograde_retrograde(),
        1.,
        duration_since_start,
    );

    draw_vector_with_icon(
        vger,
        if data.auto_rotation_mode == Some(AutoRotationTarget::RadialIn) {
            draw_active_radial_in_icon
        } else {
            draw_radial_in_icon
        },
        center,
        data.radial_in.norm() * radius,
        palette.ui_radial(),
        1.,
        duration_since_start,
    );

    draw_vector_with_icon(
        vger,
        if data.auto_rotation_mode == Some(AutoRotationTarget::RadialOut) {
            draw_active_radial_out_icon
        } else {
            draw_radial_out_icon
        },
        center,
        data.radial_out.norm() * radius,
        palette.ui_radial(),
        1.,
        duration_since_start,
    );

    if let Some(maneuver) = data.maneuver {
        draw_vector_with_icon(
            vger,
            if data.auto_rotation_mode == Some(AutoRotationTarget::Maneuver) {
                draw_active_maneuver_icon
            } else {
                draw_maneuver_icon
            },
            center,
            maneuver.norm() * radius,
            palette.ui_maneuver(),
            1.,
            duration_since_start,
        );
    }
}

pub struct FlightInfoData {
    pub velocity: NonNeg<f32>,
    pub apoapsis: NonNeg<f32>,
    pub periapsis: f32,
    pub delta_v_capacity: f32,
    pub delta_v_needed_for_manuever: f32,
    pub time_to_next_transition_point: f32,
}

pub fn draw_flight_info(
    vger: &mut vger::Vger,
    palette: &Palette,
    bb: Rect<f32>,
    data: &FlightInfoData,
) {
    vger.fill_rect(into_vger_rect(bb), 0., palette.ui_background());

    vger.stroke_rect(
        into_vger_point(bb.left_top()),
        into_vger_point(bb.right_bottom()),
        0.,
        1.,
        palette.ui_stroke(),
    );

    let left_margin = 8.;
    let right_margin = 8.;
    let row_count = 6;

    let row_height = bb.h() / row_count as f32;
    let mut draw_text = |index: usize, text: &str, color: Color| {
        draw_text(
            vger,
            (
                left_margin + *bb.x(),
                bb.y() + row_height + row_height * index as f32,
            )
                .into(),
            text,
            12,
            color,
            Some(*bb.w() - left_margin - right_margin),
        );
    };

    draw_text(
        5,
        &format!("Velocity: {:.2} m/s", data.velocity),
        UI_PROGRADE_RETROGRADE_COLOR,
    );

    draw_text(
        4,
        &format!("Apoapsis: {:.2} m", data.apoapsis),
        UI_PROGRADE_RETROGRADE_COLOR,
    );

    draw_text(
        3,
        &format!("Periapsis: {:.2} m", data.periapsis),
        UI_PROGRADE_RETROGRADE_COLOR,
    );

    draw_text(
        2,
        &format!("Δv capacity: {:.2} m/s", data.delta_v_capacity),
        UI_PROGRADE_RETROGRADE_COLOR,
    );

    draw_text(
        1,
        &format!(
            "Manuever Δv: {:.2} m/s (TODO)",
            data.delta_v_needed_for_manuever
        ),
        UI_MANEUVER_COLOR,
    );

    draw_text(
        0,
        &format!(
            "Time to trans: {:.2} s (TODO)",
            data.time_to_next_transition_point
        ),
        UI_MANEUVER_COLOR,
    );
}

pub struct VesselInfoData {
    pub thrust: f32,
    pub thrust_acceleration: f32,
    pub mass: f32,
    pub todo2: f32,
    pub todo3: f32,
    pub todo4: f32,
}

pub fn draw_vessel_info(
    vger: &mut vger::Vger,
    palette: &Palette,
    bb: Rect<f32>,
    data: &VesselInfoData,
) {
    vger.fill_rect(into_vger_rect(bb), 0., palette.ui_background());

    vger.stroke_rect(
        into_vger_point(bb.left_top()),
        into_vger_point(bb.right_bottom()),
        0.,
        1.,
        palette.ui_stroke(),
    );

    let left_margin = 8.;
    let right_margin = 8.;
    let row_count = 6;

    let row_height = bb.h() / row_count as f32;
    let mut draw_text = |index: usize, text: &str, color: Color| {
        draw_text(
            vger,
            (
                left_margin + *bb.x(),
                bb.y() + row_height + row_height * index as f32,
            )
                .into(),
            text,
            12,
            color,
            Some(*bb.w() - left_margin - right_margin),
        );
    };

    draw_text(
        5,
        &format!("Thrust: {:.2} N", data.thrust),
        UI_PROGRADE_RETROGRADE_COLOR,
    );

    draw_text(
        4,
        &format!("Thrust acceleration: {:.2} m/c^2", data.thrust_acceleration),
        UI_PROGRADE_RETROGRADE_COLOR,
    );

    draw_text(3, &format!("Mass: {:.2} kg", data.mass), UI_STROKE_COLOR);

    draw_text(2, &format!("todo2: {:.2}", data.todo2), UI_TODO_COLOR);

    draw_text(1, &format!("todo3: {:.2}", data.todo3), UI_TODO_COLOR);

    draw_text(0, &format!("todo4: {:.2}", data.todo4), UI_TODO_COLOR);
}

pub struct ControlsInfoData {
    pub manuever_mode: bool,
}

pub fn draw_controls_info(
    vger: &mut vger::Vger,
    palette: &Palette,
    bb: Rect<f32>,
    data: &ControlsInfoData,
) {
    vger.fill_rect(into_vger_rect(bb), 0., palette.ui_background());

    vger.stroke_rect(
        into_vger_point(bb.left_top()),
        into_vger_point(bb.right_bottom()),
        0.,
        1.,
        palette.ui_stroke(),
    );

    let margin = 8.;

    draw_text(
        vger,
        bb.left_bottom() + Vector::from((margin, -margin)),
        &if data.manuever_mode {
            [
                "Enter/Exit manuever mode: M",
                "Auto rotate to manuever: Alt + M",
                "Add manuever prograde vel: Up arrow",
                "Add manuever retrograde vel: Down arrow",
                "Add manuever radial out vel: Left arrow",
                "Add manuever radial in vel: Right arrow",
                "Move manuever start anomaly forward: >",
                "Move manuever start anomaly backward: <",
            ]
            .join("\n")
        } else {
            [
                "Move camera: Wheel, Shift + Wheel, Ctrl + Wheel",
                "Center on vessel: C",
                "Turn left/right: A/D (Stop rotation: X)",
                "Throttle up/down: W/S",
                "Auto rotate to prograde: Alt + Up arrow",
                "Auto rotate to retrograde: Alt + Down arrow",
                "Auto rotate to radial out: Alt + Left arrow",
                "Auto rotate to radial in: Alt + Right arrow",
                "Enter/Exit manuever mode: M",
            ]
            .join("\n")
        },
        10,
        UI_NEUTRAL_TEXT_COLOR,
        Some(bb.w() - margin * 2.),
    )
}

pub struct ManueverInfoData {
    pub manuever_mode: bool,
    pub time_to_manuever: RelativeDuration,
    pub manuever_duration: Duration,
    pub delta_v_needed: NonNeg<f32>,
    pub todo0: f32,
    pub todo1: f32,
}

pub fn draw_manuever_info(
    vger: &mut vger::Vger,
    palette: &Palette,
    bb: Rect<f32>,
    data: &ManueverInfoData,
) {
    vger.fill_rect(into_vger_rect(bb), 0., palette.ui_background());

    vger.stroke_rect(
        into_vger_point(bb.left_top()),
        into_vger_point(bb.right_bottom()),
        0.,
        1.,
        palette.ui_stroke(),
    );

    if data.manuever_mode {
        let left_margin = 8.;
        let right_margin = 8.;
        let row_count = 6;

        let row_height = bb.h() / row_count as f32;
        let mut draw_text = |index: usize, text: &str, color: Color| {
            draw_text(
                vger,
                (
                    left_margin + *bb.x(),
                    bb.y() + row_height + row_height * index as f32,
                )
                    .into(),
                text,
                12,
                color,
                Some(*bb.w() - left_margin - right_margin),
            );
        };

        draw_text(
            5,
            &format!(
                "time_to_manuever: {:.2} s",
                data.time_to_manuever.as_secs_f32()
            ),
            UI_MANEUVER_COLOR,
        );

        draw_text(
            4,
            &format!(
                "time_to_trust: {:.2} s",
                data.time_to_manuever.as_secs_f32() - data.manuever_duration.as_secs_f32() / 2.
            ),
            UI_MANEUVER_COLOR,
        );

        draw_text(
            3,
            &format!(
                "manuever_duration: {:.2} s",
                data.manuever_duration.as_secs_f32()
            ),
            UI_MANEUVER_COLOR,
        );

        draw_text(
            2,
            &format!("delta_v_needed: {:.2} m/c", data.delta_v_needed),
            UI_MANEUVER_COLOR,
        );

        draw_text(1, &format!("todo0: {:.2}", data.todo0), UI_TODO_COLOR);

        draw_text(0, &format!("todo1: {:.2}", data.todo1), UI_TODO_COLOR);
    } else {
        draw_text_centered(
            vger,
            bb.center(),
            "Manuever mode OFF",
            20,
            UI_MANEUVER_COLOR,
            None,
        );
    }
}

pub struct ThrottleBarData {
    pub throttle: f32,
}

pub fn draw_throttle_bar(
    vger: &mut vger::Vger,
    palette: &Palette,
    bb: Rect<f32>,
    data: &ThrottleBarData,
) {
    vger.fill_rect(into_vger_rect(bb), 0., palette.ui_background());

    vger.stroke_rect(
        into_vger_point(bb.left_top()),
        into_vger_point(bb.right_bottom()),
        0.,
        1.,
        palette.ui_stroke(),
    );

    let margin = 8.;

    vger.stroke_rect(
        into_vger_point(bb.left_top() + Vector::from((margin, margin))),
        into_vger_point(bb.right_bottom() - Vector::from((margin, margin))),
        0.,
        1.,
        palette.ui_stroke(),
    );

    let fill_bb: Rect<_> = (
        *bb.x() + margin + 1.,
        *bb.y() + margin + 1.,
        *bb.w() - 2. * margin - 2.,
        (*bb.h() - 2. * margin - 2.) * data.throttle,
    )
        .into();

    vger.fill_rect(
        into_vger_rect(fill_bb),
        0.,
        palette.ui_prograde_retrograde(),
    );
}

pub struct TimeInfoData {
    pub time_speed: f32,
}

pub fn draw_time_info(
    vger: &mut vger::Vger,
    palette: &Palette,
    bb: Rect<f32>,
    data: &TimeInfoData,
) {
    vger.fill_rect(into_vger_rect(bb), 0., palette.ui_background());

    vger.stroke_rect(
        into_vger_point(bb.left_top()),
        into_vger_point(bb.right_bottom()),
        0.,
        1.,
        palette.ui_stroke(),
    );

    draw_text_centered(
        vger,
        bb.center(),
        &format!("Time speed: {}X", data.time_speed),
        14,
        UI_MANEUVER_COLOR,
        None,
    );
}
