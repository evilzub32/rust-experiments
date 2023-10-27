use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

const TURNING_SPEED: u32 = 8;
const THRUST: f32 = -0.2;
const MAX_SPEED: f32 = 8.;

const SHIP_POLY: [Vector2; 3] = [
    Vector2{x: 0., y: -20.},
    Vector2{x: 15., y: 20.},
    Vector2{x: -15., y: 20.},
];

pub enum Rotation {
    None,
    Clockwise,
    Counterclockwise
}

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { x: 0., y: 0. }
    }

    pub fn rotate(&self, angle_deg: i16) -> Vector2 {
        // first, convert to rad
        let angle = angle_deg as f32;
        let angle_rad = angle.to_radians();
        Vector2 {
            x: angle_rad.cos() * self.x - angle_rad.sin() * self.y,
            y: angle_rad.sin() * self.x + angle_rad.cos() * self.y,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn multiply(&self, num: f32) -> Vector2 {
        if num.is_nan() {
            Vector2 { x: self.x, y: self.y }
        } else {
            Vector2 { x: self.x * num, y: self.y * num }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
pub struct GameObject {
    // choosing u32 for no particular reason

    // need screen dimension to calculate wrap-around
    screen_width: u32,
    screen_height: u32,

    pub position: Point,

    pub angle_deg: i16,
    pub rotation: Rotation,

    thrust: f32,
    thrust_vector: Vector2,
    velocity_vector: Vector2,
}

pub trait MovingObject {
    fn get_screen_width(&self) -> u32;
    fn get_screen_height(&self) -> u32;
    fn get_position(&self) -> &Point;
    fn set_position(&mut self, position: &mut Point);

    // if value exceeds boundary, wrap it to the "left" side of the range and the other way round
    fn wrap_around(&mut self) {
        let i16_screen_width = self.get_screen_width() as i16;
        let i16_screen_height = self.get_screen_height() as i16;

        let mut pos = self.get_position().clone();
        if pos.x > i16_screen_width {
            pos.x = pos.x - i16_screen_width;
        } else if pos.x < 0 {
            pos.x = i16_screen_width + pos.x;
        }

        if pos.y > i16_screen_height {
            pos.y = pos.y - i16_screen_height;
        } else if pos.y < 0 {
            pos.y = i16_screen_height + pos.y;
        }

        self.set_position(&mut pos);
    }
}

impl GameObject {

    // static new method as "constructor"
    pub fn new(screen_width: u32, screen_height: u32, x: i16, y: i16) -> GameObject {
        GameObject {
            screen_width: screen_width,
            screen_height: screen_height,
            position: Point{x: x, y: y},
            angle_deg: 0,
            rotation: Rotation::None,
            thrust: 0.,
            thrust_vector: Vector2::new(),
            velocity_vector: Vector2::new()
        }
    }

    pub fn increase_thrust(&mut self) {
        self.thrust = THRUST;
    }

    pub fn decrease_thrust(&mut self) {
        self.thrust = 0.;
    }

    pub fn update(&mut self) {
        match self.rotation {
            Rotation::Clockwise => {
                self.angle_deg += TURNING_SPEED as i16;
                if self.angle_deg >= 360 {
                    self.angle_deg = self.angle_deg - 360;
                }
            },
            Rotation::Counterclockwise => {
                self.angle_deg -= TURNING_SPEED as i16;
                if self.angle_deg < 0 {
                    self.angle_deg = 360 + self.angle_deg;
                }
            },
            _ => {}
        }
        self.thrust_vector = Vector2{x: 0., y: self.thrust as f32}.rotate(self.angle_deg);

        self.velocity_vector = self.velocity_vector.add(&self.thrust_vector);

        // observe the speed limit
        let speed = self.velocity_vector.magnitude();
        if speed > MAX_SPEED {
            let limiter = MAX_SPEED / speed;
            self.velocity_vector = self.velocity_vector.multiply(limiter);
        }

        let mut pos = self.position;
        pos.x += self.velocity_vector.x.round() as i16;
        pos.y += self.velocity_vector.y.round() as i16;
        self.set_position(&mut pos);

        self.wrap_around();
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // SDL2 methods with aa_...: means "anti alias" :)

        let pos = self.get_position();

        let mut ship_poly_x: [i16; 3] = [0; 3];
        let mut ship_poly_y: [i16; 3] = [0; 3];

        for n in 0..3 {
            let point = SHIP_POLY[n].rotate(self.angle_deg);
            ship_poly_x[n] = point.x.round() as i16 + pos.x;
            ship_poly_y[n] = point.y.round() as i16 + pos.y;
        }

        canvas.aa_polygon(&ship_poly_x, &ship_poly_y, Color::YELLOW).unwrap();

        // paint thrust vector in green
        let indicator = Vector2 { x: 0., y: 1.}.rotate(self.angle_deg);

        canvas.aa_line(
            pos.x as i16,
            pos.y as i16,
            // multiply with scaling factor to make it visible
            pos.x as i16 + (indicator.x * 20.) as i16,
            pos.y as i16 + (indicator.y * 20.) as i16,
            Color::GREEN
        ).unwrap();
    }

}

impl MovingObject for GameObject {
    fn get_screen_width(&self) -> u32 {
        self.screen_width
    }

    fn get_screen_height(&self) -> u32 {
        self.screen_height
    }

    fn get_position(&self) -> &Point {
        &self.position
    }

    fn set_position(& mut self, position: &mut Point) {
        self.position = *position;
    }
}