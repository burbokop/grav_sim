use crate::{
    game::orbit::EllipticOrbit,
    utils::{color::Color, misc::ellipse_from_apo},
};
use burbomath::{
    Angle, Point,
    physics::{Kg, KgPerM3, M},
};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
    time::Duration,
};

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
    pub satellites: RefCell<Vec<WOMSatellite>>,
}

impl WOMCelestialBody {
    fn proceed(&self, dt: Duration) {
        for sat in self.satellites.borrow_mut().deref_mut() {
            sat.orbit.proceed(self, dt);
            sat.body.proceed(dt);
        }
    }
}

pub struct WOMSatellite {
    pub body: Rc<WOMCelestialBody>,
    pub orbit: EllipticOrbit,
}

pub struct WOMRoot {
    pub central_body: Rc<WOMCelestialBody>,
}

impl WOMRoot {
    pub(crate) fn proceed(&self, dt: Duration) {
        self.central_body.proceed(dt);
    }
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
            satellites: RefCell::new(satellites),
        }
    }
}

impl WOMSatellite {
    pub fn from_apo(
        body: Rc<WOMCelestialBody>,
        apoapsis: M<f32>,
        periapsis: M<f32>,
        periapsis_heading: Angle<f32>,
        anomaly: Angle<f32>,
    ) -> Self {
        Self {
            body,
            orbit: EllipticOrbit::new(
                ellipse_from_apo(apoapsis, periapsis, periapsis_heading),
                anomaly,
            ),
        }
    }
}

fn trace_global_body_center(
    current: &WOMCelestialBody,
    target: &WOMCelestialBody,
    origin: Point<f32>,
) -> Option<Point<f32>> {
    if current as *const WOMCelestialBody == target as *const WOMCelestialBody {
        return Some(origin);
    }

    for satellite in current.satellites.borrow().deref() {
        match trace_global_body_center(
            &satellite.body,
            target,
            satellite.orbit.position().absolute(origin),
        ) {
            Some(center) => return Some(center),
            None => continue,
        }
    }

    None
}

pub(crate) fn orbit_to_global(
    world: &WOMRoot,
    orbit_parent_body: &WOMCelestialBody,
    orbit: EllipticOrbit,
) -> EllipticOrbit {
    let origin =
        trace_global_body_center(&world.central_body, orbit_parent_body, (0., 0.).into()).unwrap();

    let f0 = orbit.f0();
    orbit.map_center(|center| center.absolute(origin).relative(f0))
}
