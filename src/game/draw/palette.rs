use crate::utils::{color::Color, convertions::into_vger_color};
use vger::PaintIndex;

pub const BACKGROUND_COLOR: vger::Color = into_vger_color(Color::from_u32(0xff130a17));
pub const ORBIT_COLOR: vger::Color = into_vger_color(Color::from_u32(0xff5472d6));
pub const MANUEVER_ORBIT_COLOR: vger::Color = into_vger_color(Color::from_u32(0xfff28b3d));

pub const UI_STROKE_COLOR: vger::Color = into_vger_color(Color::from_u32(0xaacccccc));
pub const UI_BACKGROUND_COLOR: vger::Color = into_vger_color(Color::from_u32(0x33000000));
pub const UI_NEUTRAL_TEXT_COLOR: vger::Color = into_vger_color(Color::from_u32(0xffffffff));
pub const UI_PROGRADE_RETROGRADE_COLOR: vger::Color = into_vger_color(Color::from_u32(0xffd7fe00));
pub const UI_RADIAL_COLOR: vger::Color = into_vger_color(Color::from_u32(0xff00d6d6));
pub const UI_MANEUVER_COLOR: vger::Color = into_vger_color(Color::from_u32(0xff0000d6));

pub const UI_TODO_COLOR: vger::Color = into_vger_color(Color::from_u32(0xffff0026));

pub struct Palette {
    background: PaintIndex,
    orbit: PaintIndex,
    manuever_orbit: PaintIndex,
    ui_stroke: PaintIndex,
    ui_background: PaintIndex,
    ui_prograde_retrograde: PaintIndex,
    ui_radial: PaintIndex,
    ui_maneuver: PaintIndex,
}

impl Palette {
    pub fn bind(vger: &mut vger::Vger) -> Self {
        Self {
            background: vger.color_paint(BACKGROUND_COLOR),
            orbit: vger.color_paint(ORBIT_COLOR),
            manuever_orbit: vger.color_paint(MANUEVER_ORBIT_COLOR),
            ui_stroke: vger.color_paint(UI_STROKE_COLOR),
            ui_background: vger.color_paint(UI_BACKGROUND_COLOR),
            ui_prograde_retrograde: vger.color_paint(UI_PROGRADE_RETROGRADE_COLOR),
            ui_radial: vger.color_paint(UI_RADIAL_COLOR),
            ui_maneuver: vger.color_paint(UI_MANEUVER_COLOR),
        }
    }

    pub fn background(&self) -> PaintIndex {
        self.background
    }

    pub fn orbit(&self) -> PaintIndex {
        self.orbit
    }

    pub fn manuever_orbit(&self) -> PaintIndex {
        self.manuever_orbit
    }

    pub fn ui_stroke(&self) -> PaintIndex {
        self.ui_stroke
    }

    pub fn ui_background(&self) -> PaintIndex {
        self.ui_background
    }

    pub fn ui_prograde_retrograde(&self) -> PaintIndex {
        self.ui_prograde_retrograde
    }

    pub fn ui_radial(&self) -> PaintIndex {
        self.ui_radial
    }

    pub fn ui_maneuver(&self) -> PaintIndex {
        self.ui_maneuver
    }
}
