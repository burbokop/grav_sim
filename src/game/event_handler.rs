use crate::game::game_logic::GameLogic;
use burbomath::{Point, Size, camera::Camera};
use winit::{
    event::{ElementState, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AutoRotationTarget {
    Prograde,
    Retrograde,
    RadialIn,
    RadialOut,
    Maneuver,
}

pub struct EventHandler {
    control: bool,
    shift: bool,
    alt: bool,
    mouse_position: Point<i32>,
    mouse_position_in_world_space: Point<f32>,
    center_on_vessel_mode: bool,
    auto_rotation_mode: Option<AutoRotationTarget>,
    manuever_planner_mode: bool,
    w_pressed: bool,
    a_pressed: bool,
    s_pressed: bool,
    d_pressed: bool,
    x_pressed: bool,
    less_pressed: bool,
    greater_pressed: bool,
    left_arrow_pressed: bool,
    right_arrow_pressed: bool,
    up_arrow_pressed: bool,
    down_arrow_pressed: bool,
    exit_requested: bool,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            control: false,
            shift: false,
            alt: false,
            mouse_position: (0, 0).into(),
            mouse_position_in_world_space: (0., 0.).into(),
            center_on_vessel_mode: true,
            auto_rotation_mode: None,
            manuever_planner_mode: false,
            w_pressed: false,
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
            x_pressed: false,
            less_pressed: false,
            greater_pressed: false,
            left_arrow_pressed: false,
            right_arrow_pressed: false,
            up_arrow_pressed: false,
            down_arrow_pressed: false,
            exit_requested: false,
        }
    }
}

impl EventHandler {
    pub fn center_on_vessel_mode(&self) -> bool {
        self.center_on_vessel_mode
    }

    pub fn auto_rotation_mode(&self) -> Option<AutoRotationTarget> {
        self.auto_rotation_mode
    }

    pub fn manuever_planner_mode(&self) -> bool {
        self.manuever_planner_mode
    }

    pub fn w_pressed(&self) -> bool {
        self.w_pressed
    }

    pub fn a_pressed(&self) -> bool {
        self.a_pressed
    }

    pub fn s_pressed(&self) -> bool {
        self.s_pressed
    }

    pub fn d_pressed(&self) -> bool {
        self.d_pressed
    }

    pub fn x_pressed(&self) -> bool {
        self.x_pressed
    }

    pub fn less_pressed(&self) -> bool {
        self.less_pressed
    }

    pub fn greater_pressed(&self) -> bool {
        self.greater_pressed
    }

    pub fn left_arrow_pressed(&self) -> bool {
        self.left_arrow_pressed
    }

    pub fn right_arrow_pressed(&self) -> bool {
        self.right_arrow_pressed
    }

    pub fn up_arrow_pressed(&self) -> bool {
        self.up_arrow_pressed
    }

    pub fn down_arrow_pressed(&self) -> bool {
        self.down_arrow_pressed
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    pub fn handle_event(
        &mut self,
        game_logic: &mut GameLogic,
        camera: &mut Camera<f32>,
        event: WindowEvent,
        window_size: Size<u32>,
    ) {
        match event {
            WindowEvent::ActivationTokenDone { .. } => todo!(),
            WindowEvent::Resized(_) => todo!(),
            WindowEvent::Moved(_) => todo!(),
            WindowEvent::CloseRequested => todo!(),
            WindowEvent::Destroyed => todo!(),
            WindowEvent::DroppedFile(_) => todo!(),
            WindowEvent::HoveredFile(_) => todo!(),
            WindowEvent::HoveredFileCancelled => todo!(),
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(key_code) => match (key_code, event.state) {
                    (KeyCode::Escape, ElementState::Released) => self.exit_requested = true,
                    (KeyCode::Comma, ElementState::Pressed) => self.less_pressed = true,
                    (KeyCode::Comma, ElementState::Released) => self.less_pressed = false,
                    (KeyCode::Digit0, ElementState::Released) => todo!(),
                    (KeyCode::Digit1, ElementState::Released) => game_logic.set_time_speed(1.),
                    (KeyCode::Digit2, ElementState::Released) => game_logic.set_time_speed(2.),
                    (KeyCode::Digit3, ElementState::Released) => game_logic.set_time_speed(4.),
                    (KeyCode::Digit4, ElementState::Released) => game_logic.set_time_speed(16.),
                    (KeyCode::Digit5, ElementState::Released) => game_logic.set_time_speed(64.),
                    (KeyCode::Digit6, ElementState::Released) => game_logic.set_time_speed(256.),
                    (KeyCode::Digit7, ElementState::Released) => game_logic.set_time_speed(8192.),
                    (KeyCode::Digit8, ElementState::Released) => game_logic.set_time_speed(65536.),
                    (KeyCode::Digit9, ElementState::Released) => game_logic.set_time_speed(524288.),
                    (KeyCode::KeyA, ElementState::Pressed) => {
                        self.a_pressed = true;
                        self.auto_rotation_mode = None
                    }
                    (KeyCode::KeyA, ElementState::Released) => self.a_pressed = false,
                    (KeyCode::KeyB, ElementState::Released) => todo!(),
                    (KeyCode::KeyC, ElementState::Released) => self.center_on_vessel_mode = true,
                    (KeyCode::KeyD, ElementState::Pressed) => {
                        self.d_pressed = true;
                        self.auto_rotation_mode = None
                    }
                    (KeyCode::KeyD, ElementState::Released) => self.d_pressed = false,
                    (KeyCode::KeyM, ElementState::Pressed) if self.alt => {
                        self.auto_rotation_mode = Some(AutoRotationTarget::Maneuver)
                    }
                    (KeyCode::KeyM, ElementState::Released) if !self.alt => {
                        self.manuever_planner_mode = !self.manuever_planner_mode;
                        if self.manuever_planner_mode {
                            game_logic.init_manuever();
                        }
                    }
                    (KeyCode::KeyS, ElementState::Pressed) => self.s_pressed = true,
                    (KeyCode::KeyS, ElementState::Released) => self.s_pressed = false,
                    (KeyCode::KeyW, ElementState::Pressed) => self.w_pressed = true,
                    (KeyCode::KeyW, ElementState::Released) => self.w_pressed = false,
                    (KeyCode::KeyX, ElementState::Pressed) => {
                        self.x_pressed = true;
                        self.auto_rotation_mode = None
                    }
                    (KeyCode::KeyX, ElementState::Released) => self.x_pressed = false,
                    (KeyCode::Period, ElementState::Pressed) => self.greater_pressed = true,
                    (KeyCode::Period, ElementState::Released) => self.greater_pressed = false,
                    (KeyCode::AltLeft | KeyCode::AltRight, ElementState::Pressed) => {
                        self.alt = true
                    }
                    (KeyCode::AltLeft | KeyCode::AltRight, ElementState::Released) => {
                        self.alt = false
                    }
                    (KeyCode::ControlLeft | KeyCode::ControlRight, ElementState::Pressed) => {
                        self.control = true
                    }
                    (KeyCode::ControlLeft | KeyCode::ControlRight, ElementState::Released) => {
                        self.control = false
                    }
                    (KeyCode::ShiftLeft | KeyCode::ShiftRight, ElementState::Pressed) => {
                        self.shift = true
                    }
                    (KeyCode::ShiftLeft | KeyCode::ShiftRight, ElementState::Released) => {
                        self.shift = false
                    }
                    (KeyCode::ArrowDown, ElementState::Pressed) if self.alt => {
                        self.auto_rotation_mode = Some(AutoRotationTarget::Retrograde)
                    }
                    (KeyCode::ArrowDown, ElementState::Pressed) => self.down_arrow_pressed = true,
                    (KeyCode::ArrowDown, ElementState::Released) => self.down_arrow_pressed = false,

                    (KeyCode::ArrowLeft, ElementState::Pressed) if self.alt => {
                        self.auto_rotation_mode = Some(AutoRotationTarget::RadialOut)
                    }
                    (KeyCode::ArrowLeft, ElementState::Pressed) => self.left_arrow_pressed = true,
                    (KeyCode::ArrowLeft, ElementState::Released) => self.left_arrow_pressed = false,
                    (KeyCode::ArrowRight, ElementState::Pressed) if self.alt => {
                        self.auto_rotation_mode = Some(AutoRotationTarget::RadialIn)
                    }
                    (KeyCode::ArrowRight, ElementState::Pressed) => self.right_arrow_pressed = true,
                    (KeyCode::ArrowRight, ElementState::Released) => {
                        self.right_arrow_pressed = false
                    }
                    (KeyCode::ArrowUp, ElementState::Pressed) if self.alt => {
                        self.auto_rotation_mode = Some(AutoRotationTarget::Prograde)
                    }
                    (KeyCode::ArrowUp, ElementState::Pressed) => self.up_arrow_pressed = true,
                    (KeyCode::ArrowUp, ElementState::Released) => self.up_arrow_pressed = false,
                    _ => {}
                },
                PhysicalKey::Unidentified(_) => todo!(),
            },
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (
                    position.x as i32,
                    *window_size.h() as i32 - position.y as i32,
                )
                    .into();
                self.mouse_position_in_world_space =
                    &(!&camera.transformation()).unwrap() * &self.mouse_position.as_f32();
            }
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { delta, .. } => {
                let delta_to_y = |a: MouseScrollDelta| -> f32 {
                    match a {
                        MouseScrollDelta::LineDelta(_, y) => y,
                        MouseScrollDelta::PixelDelta(physical_position) => {
                            physical_position.y as f32 / 10.
                        }
                    }
                };

                let angle_delta_to_scale_division = |angle_delta: f32| {
                    let base: f32 = 1.2;

                    base.powf(angle_delta)
                };

                let angle_delta_to_translation_delta = |angle_delta: f32| {
                    let velocity: f32 = 10.; // px per step
                    return velocity * angle_delta;
                };

                let position = self.mouse_position.as_f32();
                let y = delta_to_y(delta);

                if self.control {
                    // zoom
                    if self.center_on_vessel_mode {
                        let target_point_in_view_port_space =
                            &camera.transformation() * &game_logic.vessel_orbit().position();

                        camera.concat_scale_centered(
                            angle_delta_to_scale_division(y),
                            target_point_in_view_port_space,
                            target_point_in_view_port_space,
                        );

                        return; // to prevent setting `center_on_vessel_mode` to false
                    } else {
                        camera.concat_scale_centered(
                            angle_delta_to_scale_division(y),
                            position,
                            position,
                        );
                    }
                } else if self.shift {
                    // scroll horizontally
                    camera.add_translation((angle_delta_to_translation_delta(y), 0.).into());
                } else {
                    // scroll vertically
                    camera.add_translation((0., angle_delta_to_translation_delta(y)).into());
                }

                self.center_on_vessel_mode = false
            }
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::PinchGesture { .. } => todo!(),
            WindowEvent::PanGesture { .. } => todo!(),
            WindowEvent::DoubleTapGesture { .. } => todo!(),
            WindowEvent::RotationGesture { .. } => todo!(),
            WindowEvent::TouchpadPressure { .. } => todo!(),
            WindowEvent::AxisMotion { .. } => todo!(),
            WindowEvent::Touch(_) => todo!(),
            WindowEvent::ScaleFactorChanged { .. } => todo!(),
            WindowEvent::ThemeChanged(_) => todo!(),
            WindowEvent::Occluded(_) => todo!(),
            WindowEvent::RedrawRequested => todo!(),
        }
    }
}
