use std::ops::Range;

use rand::Rng;

use sdl2::pixels::Color;

use crate::view::renderable::Renderable;
use super::{lib::Vector2, entity::{Entity, KeyListener}};

pub enum Size {
    Large,
    Medium,
    Small,
}

impl Size {
    fn multiply(&self, value: i32) -> i32 {
        match self {
            Size::Large => value * 11,
            Size::Medium => value * 7,
            Size::Small => value * 3,
        }
    }
}

pub struct Asteroid {
    pub mass: Size,
    pub shape: Vec<Vector2>,
    pub rotated_poly: Vec<Vector2>,

    pub position: Vector2,
    pub angle_deg: f32,
    pub turnrate: f32,

    pub thrust_vector: Vector2,
    pub thrust: f32,

    pub velocity_vector: Vector2,

    pub default_color: Color,
    pub current_color: Color,
    
    pub is_colliding: bool,
}

impl Asteroid {
    pub fn new(mass: Size) -> Asteroid {
        let mut rng = rand::thread_rng();
        let shape = generate_shape(&mass);

        Asteroid {
            mass,
            shape: shape.clone(),
            rotated_poly: shape,

            position: Vector2 { x: rng.gen_range(0.0..1024.0), y: rng.gen_range(0.0..768.0) },
            angle_deg: 0.,
            turnrate: gen_range_posneg(0.3..1.0),

            thrust_vector: Vector2::new(),
            thrust: 0.,

            velocity_vector: Vector2{x: gen_range_posneg(0.4..1.0), y: gen_range_posneg(0.4..1.0)},

            default_color: Color::GRAY,
            current_color: Color::GRAY,
            is_colliding: false,
        }
    }
}

// need this to have a range of positive and negativ values that does not include 0
fn gen_range_posneg(pos_range: Range<f32>) -> f32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(pos_range);
    num * (if rng.gen::<bool>() { 1.0 } else { -1.0 })
}

fn generate_shape(mass: &Size) -> Vec<Vector2> {
    let num_nodes = 8;
    let mut shape:Vec<Vector2> = Vec::new();

    let step = 360.0 / num_nodes as f32;
    let radius = mass.multiply(7) as f32;

    for i in 0..num_nodes {
        let deg = i as f32 * step;
        let rad = deg.to_radians();

        // asteroid shape generation:
        // radius * sin/cos paints a circle
        // adding random makes circle irregular
        // multiplying random number with random 1/-1 makes it even more irregular
        let x = radius * rad.sin() + gen_range_posneg(10.0..20.0);
        let y = radius * rad.cos() + gen_range_posneg(10.0..20.0);

        shape.push(Vector2{x, y});
    }

    shape
}

impl Entity for Asteroid {
    fn get_turnrate(&self) -> f32 {
        self.turnrate
    }

    fn set_turnrate(&mut self, turnrate: f32) {
        self.turnrate = turnrate;
    }

    fn get_thrust(&self) -> f32 {
        self.thrust
    }

    fn set_thrust(&mut self, thrust: f32) {
        self.thrust = thrust;
    }

    fn get_angle_deg(&self) -> f32 {
        self.angle_deg
    }

    fn set_angle_deg(&mut self, angle_deg: f32) {
        self.angle_deg = angle_deg;
    }

    fn get_velocity_vector(&self) -> Vector2 {
        self.velocity_vector
    }

    fn set_velocity_vector(&mut self, velocity_vector: Vector2) {
        self.velocity_vector = velocity_vector;
    }

    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn get_original_shape(&self) -> &Vec<Vector2> {
        &self.shape
    }

    fn set_original_shape(&mut self, shape: Vec<Vector2>) {
        self.shape = shape;
    }

    fn get_rotated_shape(&self) -> &Vec<Vector2> {
        &self.rotated_poly
    }

    fn set_rotated_shape(&mut self, shape: Vec<Vector2>) {
        self.rotated_poly = shape;
    }

    fn is_colliding(&self) -> bool {
        self.is_colliding
    }

    fn set_colliding(&mut self, colliding: bool) {
        self.is_colliding = colliding;
    }

    fn get_default_color(&self) -> Color {
        self.default_color
    }

    fn set_default_color(&mut self, color: Color) {
        self.default_color= color;
    }

    fn get_current_color(&self) -> Color {
        self.current_color
    }

    fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    
}

impl KeyListener for Asteroid {
    fn key_down(&mut self, keycode: sdl2::keyboard::Keycode) {}

    fn key_up(&mut self, keycode: sdl2::keyboard::Keycode) {}
}

impl Renderable for Asteroid {
    fn get_polygon(&self) -> &Vec<Vector2> {
        &self.rotated_poly
    }

    fn get_color(&self) -> Color {
        self.current_color
    }
}
