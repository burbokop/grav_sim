use crate::game::{
    constants::G,
    event_handler::{AutoRotationTarget, EventHandler},
    manuever::Manuever,
    orbit::{CelestialBody, EllipticOrbit},
    utils::color_from_hex,
    vessel::{KinematicBody, Vessel},
};
use burbomath::{
    Angle, DeltaAngle, Ellipse, NonNeg, Pi as _,
    physics::{Kg, M, M3},
};
use std::{rc::Rc, time::Duration};

pub struct GameLogic {
    vessel: Vessel,
    body: Rc<CelestialBody>,
    vessel_orbit: EllipticOrbit,
    manuever: Option<Manuever>,
    time_speed: f32,
}

impl GameLogic {
    pub fn new() -> Self {
        let ellipse = Ellipse::from_angle(
            (-30., -100.).into(),
            (2000., 1900.).into(),
            Angle::from_degrees(0_f32),
        );

        let body = Rc::new(CelestialBody::from_density(
            Kg(5513.) / M3(1.),
            M(1000.),
            M(1010.),
            color_from_hex(0xffeb4034),
            color_from_hex(0x8891b8ff),
        ));

        Self {
            vessel: Vessel {
                kinematic_body: KinematicBody::new(
                    DeltaAngle::from_radians(1.),
                    NonNeg::new(0.1 * 100.).unwrap(),
                    NonNeg::new(0.1 * 100.).unwrap(),
                    NonNeg::new(0.05 * 100.).unwrap(),
                    NonNeg::new(1000.).unwrap(),
                ),
            },
            body: body.clone(),
            vessel_orbit: EllipticOrbit::new(
                Rc::downgrade(&body),
                ellipse,
                Angle::from_radians(0.),
            ),
            manuever: None,
            time_speed: 1.,
        }
    }

    pub fn set_time_speed(&mut self, t: f32) {
        self.time_speed = t
    }

    pub fn time_speed(&self) -> f32 {
        self.time_speed
    }

    pub fn body(&self) -> &Rc<CelestialBody> {
        &self.body
    }

    pub fn init_manuever(&mut self) {
        if self.manuever.is_none() {
            self.manuever = Some(Manuever {
                orbit: self.vessel_orbit.clone(),
                relative_delta_v: (0., 0.).into(),
                delta_v_anomaly: self.vessel_orbit.anomaly(),
            })
        }
    }

    pub fn manuever(&self) -> &Option<Manuever> {
        &self.manuever
    }

    pub fn vessel(&self) -> &Vessel {
        &self.vessel
    }

    pub fn vessel_orbit(&self) -> &EllipticOrbit {
        &self.vessel_orbit
    }

    pub fn update(&mut self, event_handler: &EventHandler, dt: Duration) {
        {
            let dt = Duration::from_secs_f32(dt.as_secs_f32() * self.time_speed);

            if event_handler.w_pressed() {
                self.vessel.kinematic_body.thrust_up(dt);
            } else if event_handler.s_pressed() {
                self.vessel.kinematic_body.thrust_down(dt);
            } else {
                self.vessel.kinematic_body.brake_thrust_change(dt);
            }

            let tangential_velocity = self.vessel_orbit.tangential_velocity(G);

            if event_handler.x_pressed() {
                self.vessel.kinematic_body.brake_rotation(dt);
            } else if event_handler.a_pressed() {
                if self.time_speed <= 1. {
                    self.vessel.kinematic_body.rotate_left(dt);
                }
            } else if event_handler.d_pressed() {
                if self.time_speed <= 1. {
                    self.vessel.kinematic_body.rotate_right(dt);
                }
            } else if let Some(auto_rotation_mode) = event_handler.auto_rotation_mode() {
                match auto_rotation_mode {
                    AutoRotationTarget::Prograde => self
                        .vessel
                        .kinematic_body
                        .rotate_to(tangential_velocity.angle(), dt),
                    AutoRotationTarget::Retrograde => self
                        .vessel
                        .kinematic_body
                        .rotate_to(tangential_velocity.angle() + DeltaAngle::<f32>::pi(), dt),
                    AutoRotationTarget::RadialIn => self.vessel.kinematic_body.rotate_to(
                        tangential_velocity.angle() - DeltaAngle::<f32>::pi() / 2.,
                        dt,
                    ),
                    AutoRotationTarget::RadialOut => self.vessel.kinematic_body.rotate_to(
                        tangential_velocity.angle() + DeltaAngle::<f32>::pi() / 2.,
                        dt,
                    ),
                    AutoRotationTarget::Maneuver => {
                        let target_angle = self
                            .manuever
                            .as_ref()
                            .expect(
                                "Should not enter AutoRotationTarget::Maneuver if manuever is None",
                            )
                            .delta_v(&self.vessel_orbit, G)
                            .angle();
                        self.vessel.kinematic_body.rotate_to(target_angle, dt)
                    }
                }
            }

            self.vessel.kinematic_body.proceed(dt);

            if self.vessel.kinematic_body.acceleration().into_inner() > f32::EPSILON {
                let delta_v = self
                    .vessel_orbit
                    .accelerate(self.vessel.kinematic_body.acceleration_vector(), dt);

                if let Some(manuever) = &mut self.manuever {
                    manuever.relative_delta_v -= delta_v;
                }
            }

            self.vessel_orbit.proceed(dt);

            if event_handler.manuever_planner_mode() {
                if let Some(manuever) = &mut self.manuever {
                    if event_handler.less_pressed() {
                        manuever.move_start_anomaly_backward(dt);
                    } else if event_handler.greater_pressed() {
                        manuever.move_start_anomaly_forward(dt);
                    } else if event_handler.left_arrow_pressed() {
                        manuever.accelerate_towards_radial_out(&self.vessel_orbit, dt, G);
                    } else if event_handler.right_arrow_pressed() {
                        manuever.accelerate_towards_radial_in(&self.vessel_orbit, dt, G);
                    } else if event_handler.up_arrow_pressed() {
                        manuever.accelerate_towards_prograde(&self.vessel_orbit, dt, G);
                    } else if event_handler.down_arrow_pressed() {
                        manuever.accelerate_towards_retrograde(&self.vessel_orbit, dt, G);
                    }

                    manuever.proceed(&self.vessel_orbit, G);
                }
            }
        }
    }
}
