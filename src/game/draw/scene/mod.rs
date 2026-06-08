use crate::game::{
    constants::G,
    draw::{
        palette::{MANUEVER_ORBIT_COLOR, ORBIT_COLOR, Palette},
        scene::{
            celestial_body::draw_celestial_body, orbit::draw_elliptic_orbit, vessel::draw_vessel,
        },
    },
    event_handler::EventHandler,
    game_logic::GameLogic,
    utils::{apply_transformation, into_vger_point, into_vger_rect},
};
use burbomath::{Rect, camera::Camera};
use std::time::Duration;

mod celestial_body;
mod orbit;
mod vessel;

pub(crate) fn draw_scene(
    vger: &mut vger::Vger,
    palette: &Palette,
    game_logic: &GameLogic,
    event_handler: &EventHandler,
    camera: &Camera<f32>,
    view_port_rect: Rect<f32>,
    duration_since_start: Duration,
) {
    // println!("vpr: {:?}", view_port_rect);

    vger.fill_rect(into_vger_rect(view_port_rect), 0., palette.background());

    vger.save();
    apply_transformation(vger, camera.transformation());

    let compensatory_scale = 1. / camera.transformation().average_scale();
    let body = game_logic.body();

    draw_celestial_body(vger, &body, game_logic.vessel_orbit().f0());

    draw_elliptic_orbit(
        vger,
        game_logic.vessel_orbit(),
        ORBIT_COLOR,
        compensatory_scale,
    );

    draw_vessel(
        vger,
        palette,
        game_logic.vessel(),
        game_logic.vessel_orbit(),
        G,
        compensatory_scale,
        duration_since_start,
    );

    if event_handler.manuever_planner_mode() {
        if let Some(manuever) = &game_logic.manuever() {
            let manuever_point = manuever.delta_v_point(game_logic.vessel_orbit());

            vger.fill_circle(
                into_vger_point(manuever_point),
                4. * compensatory_scale,
                palette.manuever_orbit(),
            );

            draw_elliptic_orbit(
                vger,
                &manuever.orbit,
                MANUEVER_ORBIT_COLOR,
                compensatory_scale,
            );
        }
    }

    vger.restore();
}
