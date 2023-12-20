use rand::Rng;

use sdl2::pixels::Color;

use crate::view::renderable::Renderable;
use super::{movingobject::MovingObject, lib::Vector2, lib::Point, lib::Rotation};

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
    pub entity: MovingObject,
}

impl Asteroid {
    pub fn new(mass: Size, screen_width: u32, screen_height: u32) -> Asteroid {
        let shape = generate_shape(&mass);
        let mut me = Asteroid {
            mass,
            entity: MovingObject::new (
                screen_width,
                screen_height,
                shape,
            )
        };

        me.entity.position = Point{x: 0, y: 0};
        me.entity.default_color = Color::GRAY;
        me.entity.turnrate = 0.4;
        me.entity.velocity_vector = Vector2{x: 1., y: 1.};
        me.entity.rotation = Rotation::Clockwise;

        me
    }

    pub fn update(&mut self) {
        self.entity.update()
    }
}

fn generate_shape(mass: &Size) -> Vec<Vector2> {
    let mut rng = rand::thread_rng();

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
        let x = radius * rad.sin() + rng.gen_range(10.0..20.0) * (if rng.gen::<bool>() { 1_f32 } else { -1_f32 });
        let y = radius * rad.cos() + rng.gen_range(10.0..20.0) * (if rng.gen::<bool>() { 1_f32 } else { -1_f32 });

        shape.push(Vector2{x, y});
    }

    shape
}

impl Renderable for Asteroid {
    fn get_polygon(&self) -> &Vec<Vector2> {
        &self.entity.rotated_poly
    }

    fn get_color(&self) -> Color {
        self.entity.current_color
    }
}
