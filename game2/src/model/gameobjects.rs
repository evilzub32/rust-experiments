use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

const TURNING_SPEED: u32 = 12;
const THRUST: f32 = 0.2;
const MAX_SPEED: f32 = 8.;

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { x: 0., y: 0. }
    }

    pub fn rotate(&self, angle_deg: i16) -> Vector2 {
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

pub struct GameObject {
    // choosing u32 for no particular reason

    // need screen dimension to calculate wrap-around
    screen_width: u32,
    screen_height: u32,

    pub x: i16,
    pub y: i16,

    pub angle_deg: i16,

    thrust: f32,
    thrust_vector: Vector2,
    velocity_vector: Vector2,
}

impl GameObject {

    // if value exceeds boundary, wrap it to the "left" side of the range and the other way round
    fn wrap_around(n: i16, boundary: i16) -> i16 {
        if n > boundary {
            return n - boundary
        } else if n < 0 {
            return boundary + n
        }
        n
    }

    // static new method as "constructor"
    pub fn new(screen_width: u32, screen_height: u32, x: i16, y: i16) -> GameObject {
        GameObject {
            screen_width: screen_width,
            screen_height: screen_height,
            x: x,
            y: y,
            angle_deg: 0,
            thrust: 0.,
            thrust_vector: Vector2::new(),
            velocity_vector: Vector2::new()
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.angle_deg += TURNING_SPEED as i16;
        if self.angle_deg >= 360 {
            self.angle_deg = self.angle_deg - 360;
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.angle_deg -= TURNING_SPEED as i16;
        if self.angle_deg < 0 {
            self.angle_deg = 360 + self.angle_deg;
        }
    }

    pub fn increase_thrust(&mut self) {
        self.thrust = THRUST;
    }

    pub fn decrease_thrust(&mut self) {
        self.thrust = 0.;
    }

    pub fn update(&mut self) {
        self.thrust_vector = Vector2{x: 0., y: self.thrust as f32}.rotate(self.angle_deg);

        self.velocity_vector = self.velocity_vector.add(&self.thrust_vector);

        // observe the speed limit
        let speed = self.velocity_vector.magnitude();
        if speed > MAX_SPEED {
            let limiter = MAX_SPEED / speed;
            println!("[{},{}] Limiter: {}, Speed: {}", self.velocity_vector.x, self.velocity_vector.y, limiter, speed);
            self.velocity_vector = self.velocity_vector.multiply(limiter);
        }

        self.x += self.velocity_vector.x.round() as i16;
        self.y += self.velocity_vector.y.round() as i16;

        self.x = GameObject::wrap_around(self.x, self.screen_width as i16);
        self.y = GameObject::wrap_around(self.y, self.screen_height as i16);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // Circle around center serves as symbolic "spaceship"
        canvas.circle(
            self.x as i16,
            self.y as i16,
            10,
            Color::YELLOW).unwrap();

        // paint thrust vector in green
        let indicator = Vector2 { x: 0., y: 1.}.rotate(self.angle_deg);

        canvas.line(
            self.x as i16,
            self.y as i16,
            // multiply with scaling factor to make it visible
            self.x as i16 + (indicator.x * 20.) as i16,
            self.y as i16 + (indicator.y * 20.) as i16,
            Color::GREEN
        ).unwrap();
    }

}
