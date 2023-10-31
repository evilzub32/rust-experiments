use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

use super::lib::{Vector2, Point, Rotation};
use super::movingobject::MovingObject;

const THRUST: f32 = -0.2;

const SHIP_POLY: [Vector2; 3] = [
    Vector2{x: 0., y: -20.},
    Vector2{x: 15., y: 20.},
    Vector2{x: -15., y: 20.},
];

pub struct Player {
    // choosing u32 for no particular reason

    // need screen dimension to calculate wrap-around
    screen_width: u32,
    screen_height: u32,

    pub position: Point,

    pub angle_deg: f32,
    pub rotation: Rotation,
    pub turnrate: f32,

    thrust: f32,
    thrust_vector: Vector2,
    velocity_vector: Vector2,
    max_speed: f32,
}

impl Player {

    // static new method as "constructor"
    pub fn new(screen_width: u32, screen_height: u32, x: i16, y: i16) -> Player {
        Player {
            screen_width: screen_width,
            screen_height: screen_height,
            position: Point{x, y},
            angle_deg: 0.,
            rotation: Rotation::None,
            turnrate: 8.,
            thrust: 0.,
            thrust_vector: Vector2::new(),
            velocity_vector: Vector2::new(),
            max_speed: 12.,
        }
    }

    pub fn increase_thrust(&mut self) {
        self.thrust = THRUST;
    }

    pub fn decrease_thrust(&mut self) {
        self.thrust = 0.;
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

    fn get_velocity_vector(&self) -> Vector2 {
        self.velocity_vector
    }

    fn set_velocity_vector(&mut self, velocity: Vector2) {
        self.velocity_vector = velocity;
    }

    fn get_rotation(&self) -> Rotation {
        self.rotation
    }

    fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    fn get_angle_deg(&self) -> f32 {
        self.angle_deg
    }

    fn set_angle_deg(&mut self, angle_deg: f32) {
        self.angle_deg = angle_deg;
    }

    fn get_turnrate(&self) -> f32 {
        self.turnrate
    }

    fn set_turnrate(&mut self, turnrate: f32) {
        self.turnrate = turnrate
    }

    fn get_thrust_vector(&self) -> Vector2 {
        self.thrust_vector
    }

    fn set_thrust_vector(&mut self, thrust_vector: Vector2) {
        self.thrust_vector = thrust_vector
    }

    fn get_thrust(&self) -> f32 {
        self.thrust
    }

    fn set_thrust(&mut self, thrust: f32) {
        self.thrust = thrust
    }

    fn get_max_speed(&self) -> f32 {
        self.max_speed
    }

    fn set_max_speed(&mut self, max_speed: f32) {
        self.max_speed = max_speed;
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        // SDL2 methods with aa_...: means "anti alias" :)

        let pos = self.get_position();

        let mut ship_poly_x: [i16; 3] = [0; 3];
        let mut ship_poly_y: [i16; 3] = [0; 3];

        for n in 0..3 {
            let point = SHIP_POLY[n].rotate(self.angle_deg);
            ship_poly_x[n] = point.x.round() as i16 + pos.x;
            ship_poly_y[n] = point.y.round() as i16 + pos.y;
        }

        canvas.polygon(&ship_poly_x, &ship_poly_y, Color::YELLOW).unwrap();

        // paint thrust vector in green
        let indicator = Vector2 { x: 0., y: 1.}.rotate(self.angle_deg);

        canvas.line(
            pos.x as i16,
            pos.y as i16,
            // multiply with scaling factor to make it visible
            pos.x as i16 + (indicator.x * 20.) as i16,
            pos.y as i16 + (indicator.y * 20.) as i16,
            Color::GREEN
        ).unwrap();
    }
}