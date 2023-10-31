use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

use super::lib::Rotation;
use super::{lib::{Point, Vector2}, movingobject::MovingObject};

const POLY: [Vector2; 9] = [
    Vector2{x: -20., y: -40.}, // 1
    Vector2{x: 20., y: -40.}, // 2
    Vector2{x: 40., y: -6.}, // 3
    Vector2{x: 8., y: 6.}, // 4
    Vector2{x: 16., y: 40.}, // 5
    Vector2{x: -24., y: 34.}, // 6
    Vector2{x: -26., y: 6.}, // 7
    Vector2{x: -40., y: 4.}, // 8
    Vector2{x: -40., y: -20.}, // 9
];



pub struct Asteroid {
    // need screen dimension to calculate wrap-around
    pub screen_width: u32,
    pub screen_height: u32,

    pub position: Point,
    pub angle_deg: f32,
    pub rotation: Rotation,
    pub turnrate: f32,

    pub thrust_vector: Vector2,
    pub thrust: f32,

    pub velocity_vector: Vector2,

    pub max_speed: f32,
}

impl MovingObject for Asteroid{
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

        let mut poly_x: [i16; POLY.len()] = [0; POLY.len()];
        let mut poly_y: [i16; POLY.len()] = [0; POLY.len()];

        for n in 0..POLY.len() {
            let point = POLY[n].rotate(self.angle_deg);
            poly_x[n] = point.x.round() as i16 + pos.x;
            poly_y[n] = point.y.round() as i16 + pos.y;
        }

        canvas.polygon(&poly_x, &poly_y, Color::GREY).unwrap();
    }
}
