use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

use super::lib::{Vector2, Point};
use super::movingobject::MovingObject;

const TURNING_SPEED: u32 = 8;
const THRUST: f32 = -0.2;

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

pub struct Player {
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

impl Player {

    // static new method as "constructor"
    pub fn new(screen_width: u32, screen_height: u32, x: i16, y: i16) -> Player {
        Player {
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

impl MovingObject for Player {
    fn get_screen_width(&self) -> u32 {
        self.screen_width
    }

    fn get_screen_height(&self) -> u32 {
        self.screen_height
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(& mut self, position: Point) {
        self.position = position;
    }

    fn get_velocity_vector(&self) -> &Vector2 {
        &self.velocity_vector
    }

    fn set_velocity_vector(&mut self, velocity: Vector2) {
        self.velocity_vector = velocity;
    }

    fn update(&mut self) {
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

        self.set_velocity_vector(self.velocity_vector.add(&self.thrust_vector));

        self.update_position();
    }
}