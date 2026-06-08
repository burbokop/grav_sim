use burbomath::{
    Angle, Ellipse, NonNeg, Point, Vector,
    physics::{Kg, KgPerM3, M},
    time::RelativeDuration,
};
use core::f32;
use std::{rc::Weak, time::Duration};
use vger::Color;

use crate::game::constants::G;

#[derive(Debug)]
pub struct CelestialBody {
    pub mass: Kg<f32>,
    pub solid_radius: M<f32>,
    pub atmosphere_radius: M<f32>,
    pub solid_color: Color,
    pub atmosphere_color: Color,
}

impl CelestialBody {
    pub fn from_density(
        density: KgPerM3<f32>,
        solid_radius: M<f32>,
        atmosphere_radius: M<f32>,
        solid_color: Color,
        atmosphere_color: Color,
    ) -> Self {
        let volume = solid_radius.cube() * (4. / 3. * f32::consts::PI);
        let mass = volume * density;
        Self {
            mass,
            solid_radius,
            atmosphere_radius,
            solid_color,
            atmosphere_color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EllipticOrbit {
    body: Weak<CelestialBody>,
    ellipse: Ellipse<f32>,
    anomaly: Angle<f32>,
}

impl EllipticOrbit {
    pub fn new(body: Weak<CelestialBody>, ellipse: Ellipse<f32>, anomaly: Angle<f32>) -> Self {
        Self {
            body,
            ellipse,
            anomaly,
        }
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

    pub fn body(&self) -> &Weak<CelestialBody> {
        &self.body
    }

    pub fn tangential_velocity(&self, gravitational_constant: f32) -> Vector<f32> {
        let body = self.body.upgrade().unwrap();
        self.ellipse
            .tangential_velocity(self.anomaly, body.mass.clone(), gravitational_constant)
            .unwrap()
    }

    pub fn tangential_velocity_at(
        &self,
        anomaly: Angle<f32>,
        gravitational_constant: f32,
    ) -> Vector<f32> {
        let body = self.body.upgrade().unwrap();
        self.ellipse
            .tangential_velocity(anomaly, body.mass.clone(), gravitational_constant)
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

    pub fn time_to(&self, anomaly: Angle<f32>) -> RelativeDuration {
        let body = self.body.upgrade().unwrap();
        self.ellipse
            .relative_time_between_anomalies(self.anomaly, anomaly, body.mass, G)
    }

    /// Changes orbit and returns delta velocity
    pub fn accelerate(&mut self, acceleration: Vector<f32>, dt: Duration) -> Vector<f32> {
        let body = self.body.upgrade().unwrap();

        let position = self.ellipse.point_on_ellipse(self.anomaly);

        self.ellipse =
            self.ellipse
                .accelerated(self.anomaly, body.mass.clone(), G, dt, acceleration);

        self.anomaly = self.ellipse.anomaly(position);

        acceleration * dt.as_secs_f32()
    }

    pub fn proceed(&mut self, dt: Duration) {
        let body = self.body.upgrade().unwrap();

        let angular_velocity = self
            .ellipse
            .angular_velocity(self.anomaly, body.mass.clone(), G)
            .unwrap();

        self.anomaly += angular_velocity * dt.as_secs_f32();
    }

    pub fn accelerated_at_anomaly(
        &self,
        delta_v: Vector<f32>,
        delta_v_anomaly: Angle<f32>,
        gravitational_constant: f32,
    ) -> EllipticOrbit {
        let body = self.body.upgrade().unwrap();

        let f0 = self.ellipse.f0();
        let p = self.ellipse.point_on_ellipse(delta_v_anomaly);

        let vel = self
            .ellipse
            .tangential_velocity(delta_v_anomaly, body.mass.clone(), gravitational_constant)
            .unwrap();

        let (_excentricity, new_f1) = self.ellipse.f1_from_tangential_velocity(
            delta_v_anomaly,
            body.mass.clone(),
            gravitational_constant,
            vel + delta_v,
        );

        let new_ellipse = Ellipse::from_foci(f0, new_f1, p);
        let new_anomaly = Angle::from_radians(
            delta_v_anomaly.radians().into_inner() / self.ellipse.perimeter()
                * new_ellipse.perimeter(),
        );

        EllipticOrbit {
            body: self.body.clone(),
            ellipse: new_ellipse,
            anomaly: new_anomaly,
        }
    }
}
