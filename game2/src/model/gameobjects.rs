use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

pub struct GameObject {
    // choosing u32 for no particular reason
    pub x: u32,
    pub y: u32,

    pub angle_deg: i16,
}

impl GameObject {

    // static new method as "constructor"
    pub fn new(x: u32, y: u32) -> GameObject {
        GameObject { x: x, y: y, angle_deg: 0 }
    }

    pub fn rotate_clockwise(&mut self) {
        self.angle_deg += 1;
        if self.angle_deg >= 360 {
            self.angle_deg = self.angle_deg - 360;
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.angle_deg -= 1;
        if self.angle_deg < 0 {
            self.angle_deg = 360 + self.angle_deg;
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.circle(
            self.x as i16,
            self.y as i16,
            10,
            Color::YELLOW).unwrap();
    }
}
