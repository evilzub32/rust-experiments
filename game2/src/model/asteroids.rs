use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

use super::{lib::{Point, Vector2}, movingobject::MovingObject};

const POLY: [Vector2; 9] = [
    Vector2{x: -10., y: -20.}, // 1
    Vector2{x: 10., y: -20.}, // 2
    Vector2{x: 20., y: -3.}, // 3
    Vector2{x: 4., y: 3.}, // 4
    Vector2{x: 8., y: 20.}, // 5
    Vector2{x: -12., y: 17.}, // 6
    Vector2{x: -13., y: 3.}, // 7
    Vector2{x: -20., y: 2.}, // 8
    Vector2{x: -20., y: -10.}, // 9
];

pub struct Asteroid {
    // need screen dimension to calculate wrap-around
    pub screen_width: u32,
    pub screen_height: u32,

    pub position: Point,

    pub velocity_vector: Vector2,
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

    fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    fn get_velocity_vector(&self) -> &Vector2 {
        &self.velocity_vector
    }

    fn set_velocity_vector(&mut self, velocity: Vector2) {
        self.velocity_vector = velocity;
    }

    fn update(&mut self) -> () {
        self.update_position();
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        // SDL2 methods with aa_...: means "anti alias" :)

        let pos = self.get_position();

        let mut poly_x: [i16; POLY.len()] = [0; POLY.len()];
        let mut poly_y: [i16; POLY.len()] = [0; POLY.len()];

        for n in 0..POLY.len() {
            let point = POLY[n];
            poly_x[n] = point.x.round() as i16 + pos.x;
            poly_y[n] = point.y.round() as i16 + pos.y;
        }

        canvas.polygon(&poly_x, &poly_y, Color::GREY).unwrap();
    }
}
