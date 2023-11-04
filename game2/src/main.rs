use std::time::Duration;

use model::lib::{Vector2, Rotation};
use model::movingobject::MovingObject;
use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode};

mod view;
use view::playfield;

mod model;

fn main() {
    let mut playfield: playfield::Renderer = playfield::Renderer::new(1024, 768);

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let mut player = MovingObject::new (
        playfield.screen_width,
        playfield.screen_height,
        vec![
            Vector2{x: 0., y: -20.},
            Vector2{x: 15., y: 20.},
            Vector2{x: -15., y: 20.},
    ]);
    player.default_color = Color::YELLOW;
    player.turnrate = 9.;
    player.max_speed = 12.;

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
        ]);
    asteroid.position = model::lib::Point{x: 0, y: 0};
    asteroid.default_color = Color::GRAY;
    asteroid.turnrate = 0.4;
    asteroid.velocity_vector = Vector2{x: 1., y: 1.};
    asteroid.rotation = Rotation::Clockwise;

    // FIXME: Borrow-Checker does not think this is a good idea...
    // playfield.renderables.push(&asteroid);
    // playfield.renderables.push(&player);

    while running {
        // Events will arrive erratically so use state in game-object for smooth handling
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

        if player.get_bounding_box().collides(&asteroid.get_bounding_box()) {
            player.set_colliding(true);
            asteroid.set_colliding(true);
        } else {
            player.set_colliding(false);
            asteroid.set_colliding(false);
        }

        // check_collisions(vec![Box::new(asteroid), Box::new(player)]);

        playfield.render(&vec![&asteroid, &player]);

        // don't know how this works exactly, but SDL2 docs say this is the way to limit to 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
