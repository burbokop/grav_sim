use crate::utils::{color::Color, misc::ellipse_from_apo};
use burbomath::{
    Angle, Ellipse,
    physics::{Kg, KgPerM3, M},
};
use std::rc::Rc;

pub enum WOMCelestialBodyType {
    SolidRock,
    GasGiant,
    Star,
    BlackHole,
}

pub struct WOMCelestialBody {
    pub tp: WOMCelestialBodyType,
    pub mass: Kg<f32>,
    pub solid_radius: M<f32>,
    pub atmosphere_thickness: M<f32>,
    pub solid_color: Color,
    pub atmosphere_color: Color,
    pub satellites: Vec<WOMSatellite>,
}

pub struct WOMSatellite {
    pub body: Rc<WOMCelestialBody>,
    pub ellipse: Ellipse<f32>,
    pub anomaly: Angle<f32>,
}

pub struct WOMRoot {
    pub central_body: Rc<WOMCelestialBody>,
}

impl WOMCelestialBody {
    pub fn from_density(
        tp: WOMCelestialBodyType,
        density: KgPerM3<f32>,
        solid_radius: M<f32>,
        atmosphere_thickness: M<f32>,
        solid_color: Color,
        atmosphere_color: Color,
        satellites: Vec<WOMSatellite>,
    ) -> Self {
        let volume = solid_radius.cube() * (4. / 3. * std::f32::consts::PI);
        let mass = volume * density;
        Self {
            tp,
            mass,
            solid_radius,
            atmosphere_thickness,
            solid_color,
            atmosphere_color,
            satellites,
        }
    }
}

impl WOMSatellite {
    pub fn from_(
        body: Rc<WOMCelestialBody>,
        apoapsis: M<f32>,
        periapsis: M<f32>,
        periapsis_heading: Angle<f32>,
        anomaly: Angle<f32>,
    ) -> Self {
        Self {
            body,
            ellipse: ellipse_from_apo(apoapsis, periapsis, periapsis_heading),
            anomaly,
        }
    }
}
