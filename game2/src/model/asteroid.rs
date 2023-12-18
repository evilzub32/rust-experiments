use sdl2::pixels::Color;

use super::{movingobject::MovingObject, lib::Vector2, lib::Point, lib::Rotation};

pub struct Asteroid {
    pub entity: MovingObject,
}

impl Asteroid {
    pub fn new(screen_width: u32, screen_height: u32) -> Asteroid {
        let mut me = Asteroid {
            entity: MovingObject::new (
                screen_width,
                screen_height,
                generate_shape(),
            )
        };

        me.entity.position = Point{x: 0, y: 0};
        me.entity.default_color = Color::GRAY;
        me.entity.turnrate = 0.4;
        me.entity.velocity_vector = Vector2{x: 1., y: 1.};
        me.entity.rotation = Rotation::Clockwise;

        me
    }
}

fn generate_shape() -> Vec<Vector2> {
    vec![
        Vector2{x: -20., y: -40.}, // 1
        Vector2{x: 20., y: -40.}, // 2
        Vector2{x: 40., y: -6.}, // 3
        Vector2{x: 8., y: 6.}, // 4
        Vector2{x: 16., y: 40.}, // 5
        Vector2{x: -24., y: 34.}, // 6
        Vector2{x: -26., y: 6.}, // 7
        Vector2{x: -40., y: 4.}, // 8
        Vector2{x: -40., y: -20.}, // 9
    ]
}
