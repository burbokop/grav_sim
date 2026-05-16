use winit::event::WindowEvent;

pub struct GameState {
    circle_x: f32,
    direction: f32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            circle_x: 200.0,
            direction: 2.0,
        }
    }

    pub fn event(&mut self, event: WindowEvent) {
        println!("event: {:?}", event);
    }

    pub fn update(&mut self) {
        self.circle_x += self.direction;
        if self.circle_x > 600.0 || self.circle_x < 200.0 {
            self.direction *= -1.0;
        }
    }

    pub fn render(&self, vger: &mut vger::Vger, width: f32, height: f32) {
        let paint_index0 = vger.color_paint(vger::Color::new(0.1, 0.1, 0.15, 1.0));
        vger.fill_rect(
            vger::defs::LocalRect::new(
                vger::defs::LocalPoint::new(0.0, 0.0),
                vger::defs::LocalSize::new(width, height),
            ),
            0.0,
            paint_index0,
        );

        let paint_index1 = vger.color_paint(vger::Color::new(0.2, 0.6, 1.0, 1.0));
        vger.fill_rect(
            vger::defs::LocalRect::new(
                vger::defs::LocalPoint::new(100.0, 100.0),
                vger::defs::LocalSize::new(200.0, 150.0),
            ),
            12.0,
            paint_index1,
        );

        let paint_index2 = vger.color_paint(vger::Color::new(0.1, 0.9, 0.4, 1.0));
        vger.fill_circle(
            vger::defs::LocalPoint::new(self.circle_x, 400.0),
            40.0, // Radius
            paint_index2,
        );
    }
}
