use crate::game::{
    constants::G,
    draw::{
        palette::Palette,
        scene::draw_scene,
        ui::{
            UIData, draw_ui,
            panels::{
                ControlsInfoData, FlightInfoData, ManueverInfoData, NavCircleData, ThrottleBarData,
                TimeInfoData, VesselInfoData,
            },
        },
    },
    event_handler::EventHandler,
    game_logic::GameLogic,
};
use burbomath::{Complex, Pi, Rect, Size, camera::Camera, non_neg, time::RelativeDuration};
use std::time::Duration;
use winit::event::WindowEvent;

pub struct AppComponents {
    game_logic: GameLogic,
    event_handler: EventHandler,
    camera: Camera<f32>,
}

impl AppComponents {
    pub fn new() -> Self {
        Self {
            game_logic: GameLogic::new(),
            event_handler: Default::default(),
            camera: Camera::default(),
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.event_handler.exit_requested()
    }

    pub fn handle_event(&mut self, event: WindowEvent, window_size: Size<u32>) {
        self.event_handler
            .handle_event(&mut self.game_logic, &mut self.camera, event, window_size);
    }

    pub fn update(&mut self, window_rect: Rect<f32>, dt: Duration) {
        self.game_logic.update(&self.event_handler, dt);

        if self.event_handler.center_on_vessel_mode() {
            let target_point = self.game_logic.vessel_orbit().position();
            let window_center = window_rect.center();
            self.camera.translate_to_target(target_point, window_center);
        }
    }

    pub fn render(
        &self,
        vger: &mut vger::Vger,
        view_port_rect: Rect<f32>,
        duration_since_start: Duration,
    ) {
        let palette = Palette::bind(vger);

        draw_scene(
            vger,
            &palette,
            &self.game_logic,
            &self.event_handler,
            &self.camera,
            view_port_rect,
            duration_since_start,
        );

        let ui_data = produce_ui_data(&self.game_logic, &self.event_handler);
        draw_ui(
            vger,
            &palette,
            view_port_rect,
            &ui_data,
            duration_since_start,
        );
    }
}

fn produce_ui_data(game_logic: &GameLogic, event_handler: &EventHandler) -> UIData {
    let throttle_bar_data = ThrottleBarData {
        throttle: game_logic.vessel().kinematic_body.thrust().into_inner()
            / game_logic.vessel().kinematic_body.max_thrust().into_inner(),
    };

    let tangential_velocity = game_logic.vessel_orbit().tangential_velocity(G);

    let heading = game_logic.vessel().kinematic_body.complex_heading();

    let top_axis = !heading * Complex::from_cartesian(0., 1.);
    let bottom_axis = top_axis * Complex::from_polar(1., Pi::pi());
    let left_axis = top_axis * Complex::from_cartesian(0., -1.);
    let right_axis = top_axis * Complex::from_cartesian(0., 1.);

    let nav_data = NavCircleData {
        prograde: tangential_velocity.norm() * top_axis,
        retrograde: tangential_velocity.norm() * bottom_axis,
        radial_in: tangential_velocity.norm() * left_axis,
        radial_out: tangential_velocity.norm() * right_axis,
        maneuver: if event_handler.manuever_planner_mode() {
            let manuever = game_logic.manuever().as_ref().unwrap();
            let delta_v = manuever.delta_v(&game_logic.vessel_orbit(), G);

            Some(delta_v * top_axis)
        } else {
            None
        },
        auto_rotation_mode: event_handler.auto_rotation_mode(),
    };

    let manuever_info_data = ManueverInfoData {
        manuever_mode: event_handler.manuever_planner_mode(),
        time_to_manuever: game_logic
            .manuever()
            .as_ref()
            .map(|m| game_logic.vessel_orbit().time_to(m.delta_v_anomaly))
            .unwrap_or(RelativeDuration::from_secs(0)),
        manuever_duration: game_logic
            .manuever()
            .as_ref()
            .map(|m| m.duration(game_logic.vessel().kinematic_body.max_acceleration()))
            .unwrap_or(Duration::from_secs(0)),
        delta_v_needed: game_logic
            .manuever()
            .as_ref()
            .map(|m| m.relative_delta_v.len())
            .unwrap_or(non_neg!(0.)),
        todo0: 0.,
        todo1: 0.,
    };

    let flight_info_data = FlightInfoData {
        velocity: tangential_velocity.len(),
        apoapsis: game_logic.vessel_orbit().apoapsis(),
        periapsis: game_logic.vessel_orbit().periapsis(),
        delta_v_capacity: 100.,
        delta_v_needed_for_manuever: 100.,
        time_to_next_transition_point: 1000.,
    };

    let vessel_info_data = VesselInfoData {
        thrust: game_logic.vessel().kinematic_body.thrust().into_inner(),
        thrust_acceleration: game_logic
            .vessel()
            .kinematic_body
            .acceleration()
            .into_inner(),
        mass: game_logic.vessel().kinematic_body.mass().into_inner(),
        todo2: 0.,
        todo3: 0.,
        todo4: 0.,
    };

    let time_info_data = TimeInfoData {
        time_speed: game_logic.time_speed(),
    };

    let controls_info_data = ControlsInfoData {
        manuever_mode: event_handler.manuever_planner_mode(),
    };

    UIData {
        nav_circle: nav_data,
        flight_info: flight_info_data,
        manuever_info: manuever_info_data,
        throttle_bar: throttle_bar_data,
        vessel_info: vessel_info_data,
        time_info: time_info_data,
        controls_info: controls_info_data,
    }
}
