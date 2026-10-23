use crate::game::{constants::G, world_object_model::WOMCelestialBody};
use burbomath::{Angle, Ellipse, NonNeg, Point, Vector, time::RelativeDuration};
use core::f32;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EllipticOrbit {
    ellipse: Ellipse<f32>,
    anomaly: Angle<f32>,
}

impl EllipticOrbit {
    pub fn new(ellipse: Ellipse<f32>, anomaly: Angle<f32>) -> Self {
        Self { ellipse, anomaly }
    }

    pub fn anomaly(&self) -> Angle<f32> {
        self.anomaly
    }

    pub fn apoapsis(&self) -> NonNeg<f32> {
        self.ellipse.apoapsis()
    }

    pub fn periapsis(&self) -> f32 {
        self.ellipse.periapsis()
    }

    pub fn f0(&self) -> Point<f32> {
        self.ellipse.f0()
    }

    pub fn tangential_velocity(
        &self,
        parent_body: &WOMCelestialBody,
        gravitational_constant: f32,
    ) -> Vector<f32> {
        self.ellipse
            .tangential_velocity(self.anomaly, parent_body.mass, gravitational_constant)
            .unwrap()
    }

    pub fn tangential_velocity_at(
        &self,
        parent_body: &WOMCelestialBody,
        anomaly: Angle<f32>,
        gravitational_constant: f32,
    ) -> Vector<f32> {
        self.ellipse
            .tangential_velocity(anomaly, parent_body.mass, gravitational_constant)
            .unwrap()
    }

    pub fn ellipse(&self) -> Ellipse<f32> {
        self.ellipse
    }

    pub fn point_at(&self, anomaly: Angle<f32>) -> Point<f32> {
        self.ellipse.point_on_ellipse(anomaly)
    }

    pub fn position(&self) -> Point<f32> {
        self.ellipse.point_on_ellipse(self.anomaly)
    }

    pub fn time_to(&self, parent_body: &WOMCelestialBody, anomaly: Angle<f32>) -> RelativeDuration {
        self.ellipse
            .relative_time_between_anomalies(self.anomaly, anomaly, parent_body.mass, G)
    }

    /// Changes orbit and returns delta velocity
    pub fn accelerate(
        &mut self,
        parent_body: &WOMCelestialBody,
        acceleration: Vector<f32>,
        dt: Duration,
    ) -> Vector<f32> {
        let position = self.ellipse.point_on_ellipse(self.anomaly);

        self.ellipse =
            self.ellipse
                .accelerated(self.anomaly, parent_body.mass, G, dt, acceleration);

        self.anomaly = self.ellipse.anomaly(position);

        acceleration * dt.as_secs_f32()
    }

    pub fn proceed(&mut self, parent_body: &WOMCelestialBody, dt: Duration) {
        let angular_velocity = self
            .ellipse
            .angular_velocity(self.anomaly, parent_body.mass, G)
            .unwrap();

        self.anomaly += angular_velocity * dt.as_secs_f32();
    }

    pub fn accelerated_at_anomaly(
        &self,
        parent_body: &WOMCelestialBody,
        delta_v: Vector<f32>,
        delta_v_anomaly: Angle<f32>,
        gravitational_constant: f32,
    ) -> EllipticOrbit {
        let f0 = self.ellipse.f0();
        let p = self.ellipse.point_on_ellipse(delta_v_anomaly);

        let vel = self
            .ellipse
            .tangential_velocity(delta_v_anomaly, parent_body.mass, gravitational_constant)
            .unwrap();

        let (_excentricity, new_f1) = self.ellipse.f1_from_tangential_velocity(
            delta_v_anomaly,
            parent_body.mass,
            gravitational_constant,
            vel + delta_v,
        );

        let new_ellipse = Ellipse::from_foci(f0, new_f1, p);
        let new_anomaly = Angle::from_radians(
            delta_v_anomaly.radians().into_inner() / self.ellipse.perimeter()
                * new_ellipse.perimeter(),
        );

        EllipticOrbit {
            ellipse: new_ellipse,
            anomaly: new_anomaly,
        }
    }

    pub fn map_center<F>(self, f: F) -> Self
    where
        F: FnOnce(Point<f32>) -> Point<f32>,
    {
        Self {
            ellipse: self.ellipse.map_center(f),
            anomaly: self.anomaly,
        }
    }
}
