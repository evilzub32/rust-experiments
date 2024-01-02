use sdl2::{pixels::Color, keyboard::Keycode};

use crate::view::renderable::Renderable;

use super::{lib::Vector2, entity::{Entity, KeyListener}};

#[derive(Debug)]
pub struct Player {
    pub position: Vector2,
    pub angle_deg: f32,
    pub turnrate: f32,

    pub thrust_vector: Vector2,
    pub thrust: f32,

    pub velocity_vector: Vector2,

    pub shape: Vec<Vector2>,
    pub rotated_poly: Vec<Vector2>,

    pub default_color: Color,
    pub current_color: Color,
    pub is_colliding: bool,
}

impl Entity for Player {
    fn get_turnrate(&self) -> f32 {
        self.turnrate
    }

    fn set_turnrate(&mut self, turnrate: f32) {
        self.turnrate = turnrate;
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
        self.rotated_poly = shape
    }

    fn get_thrust(&self) -> f32 {
        self.thrust
    }

    fn set_thrust(&mut self, thrust: f32) {
        self.thrust = thrust;
    }

    // fn new(start_position: Vector2, polygon: Vec<Vector2>) -> Self {
    //     Self {
    //         position: start_position,
    //         angle_deg: 0.,
    //         turnrate: 0.,
    //         thrust_vector: Vector2::new(),
    //         thrust: 0.,
    //         velocity_vector: Vector2::new(),
    //         shape: polygon.clone(),
    //         rotated_poly: polygon,
    //         default_color: Color::WHITE,
    //         current_color: Color::WHITE,
    //         is_colliding: false,
    //     }
    // }
}

impl KeyListener for Player {
    fn key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Left => self.set_turnrate(-self.get_max_turnrate()),
            Keycode::Right => self.set_turnrate(self.get_max_turnrate()),
            Keycode::Up => self.set_thrust(self.get_max_thrust()),
            
            // Keycode::Escape => {
            //     running = false;
            // },
            _ => {}
        }
    }

    fn key_up(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Left => self.set_turnrate(0.),
            Keycode::Right => self.set_turnrate(0.),
            Keycode::Up => self.set_thrust(0.),
            
            // Keycode::Escape => {
            //     running = false;
            // },
            _ => {}
        }        
    }
}

impl Renderable for Player {
    fn get_polygon(&self) -> &Vec<Vector2> {
        &self.rotated_poly
    }

    fn get_color(&self) -> Color {
        self.current_color
    }
}