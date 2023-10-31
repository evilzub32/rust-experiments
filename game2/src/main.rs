use std::time::Duration;

use model::lib::{Vector2, Rotation};
use model::movingobject::MovingObject;
use sdl2::{event::Event, keyboard::Keycode};

mod view;
use view::playfield;

mod model;

fn main() {
    let mut playfield: playfield::Renderer = playfield::Renderer::new(800, 600);

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let mut player = MovingObject::new (
        playfield.screen_width,
        playfield.screen_height,
        vec![
            Vector2{x: 0., y: -20.},
            Vector2{x: 15., y: 20.},
            Vector2{x: -15., y: 20.},
        ],
        None, None, Some(8.), Some(12.));

    let mut asteroid = MovingObject::new (
        playfield.screen_width,
        playfield.screen_height,
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
        ],
        Some(0), Some(0), Some(0.4), None);
    asteroid.velocity_vector = Vector2{x: 1., y: 1.};
    asteroid.rotation = Rotation::Clockwise;

    while running {
        // Events will arrive erraticly so use state in gameobject for smooth handling
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => player.rotation = Rotation::Counterclockwise,
                        Keycode::Right => player.rotation = Rotation::Clockwise,
                        Keycode::Up => player.increase_thrust(),

                        Keycode::Escape => {
                            running = false;
                        },
                        _ => {}
                    }
                },
                Event::KeyUp { keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Left => player.rotation = Rotation::None,
                        Keycode::Right => player.rotation = Rotation::None,
                        Keycode::Up => {
                            player.decrease_thrust();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        asteroid.update();
        player.update();

        let collision = player.get_bounding_box().collides(&asteroid.get_bounding_box());
        if collision {
            println!("BOING!");
        }

        playfield.render(&player, &asteroid);

        // don't know how this works exactly, but SDL2 docs say this is the way to limit to 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
