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

    let num_nodes = 6;
    let mut shape:Vec<Vector2> = Vec::new();

    let step = 360.0 / num_nodes as f32;
    let radius = mass.multiply(7);

    for i in 0..num_nodes {
        let deg = i as f32 * step;
        let angle = deg.to_radians();

        let x = radius as f32 * angle.sin() + rng.gen::<f32>() * 10 as f32 * (if rng.gen::<bool>() { 1 as f32 } else { -1 as f32 });
        println!("{}", x);

        let y = radius as f32 * angle.cos() + rng.gen::<f32>() * 10 as f32 * (if rng.gen::<bool>() { 1 as f32 } else { -1 as f32 });
        println!("{}", y);

        shape.push(Vector2{x, y});
    }

    shape
    // vec![
    //     Vector2{x: -20., y: -40.}, // 1
    //     Vector2{x: 20., y: -40.}, // 2
    //     Vector2{x: 40., y: -6.}, // 3
    //     Vector2{x: 8., y: 6.}, // 4
    //     Vector2{x: 16., y: 40.}, // 5
    //     Vector2{x: -24., y: 34.}, // 6
    //     Vector2{x: -26., y: 6.}, // 7
    //     Vector2{x: -40., y: 4.}, // 8
    //     Vector2{x: -40., y: -20.}, // 9
    // ]
}

impl Renderable for Asteroid {
    fn get_polygon(&self) -> &Vec<Vector2> {
        &self.entity.rotated_poly
    }

    fn get_color(&self) -> Color {
        self.entity.current_color
    }
}
