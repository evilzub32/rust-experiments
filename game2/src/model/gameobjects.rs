use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

const TURNING_SPEED: u32 = 8;

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn rotate(&self, angle_deg: i16) -> Vector2 {
        let angle = angle_deg as f32;
        let angle_rad = angle.to_radians();
        Vector2 {
            x: angle_rad.cos() * self.x - angle_rad.sin() * self.y,
            y: angle_rad.sin() * self.x + angle_rad.cos() * self.y,
        }
    }

    pub fn new_thrust_vector() -> Vector2 {
        // Thrustvector points upward in neutral and is rotated later
        Vector2 { x: 0., y: -1. }
    }
}

pub struct GameObject {
    // choosing u32 for no particular reason
    pub x: u32,
    pub y: u32,

    pub angle_deg: i16,
    thrust_vector: Vector2,
}

impl GameObject {

    // static new method as "constructor"
    pub fn new(x: u32, y: u32) -> GameObject {
        GameObject { x: x, y: y, angle_deg: 0, thrust_vector: Vector2 { x: 0.0, y: -1.0 } }
    }

    pub fn rotate_clockwise(&mut self) {
        self.angle_deg += TURNING_SPEED as i16;
        if self.angle_deg >= 360 {
            self.angle_deg = self.angle_deg - 360;
        }
        self.thrust_vector = Vector2::new_thrust_vector().rotate(self.angle_deg);
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.angle_deg -= TURNING_SPEED as i16;
        if self.angle_deg < 0 {
            self.angle_deg = 360 + self.angle_deg;
        }
        self.thrust_vector = Vector2::new_thrust_vector().rotate(self.angle_deg);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // Circle around center serves as symbolic "spaceship"
        canvas.circle(
            self.x as i16,
            self.y as i16,
            10,
            Color::YELLOW).unwrap();

        // paint thrust vector in green
        canvas.line(
            self.x as i16,
            self.y as i16,
            // multiply with scaling factor to make it visible
            self.x as i16 + (self.thrust_vector.x * 20.) as i16,
            self.y as i16 + (self.thrust_vector.y * 20.) as i16,
            Color::GREEN
        ).unwrap();
    }

}
