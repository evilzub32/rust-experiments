use std::time::Duration;

use model::asteroids::Asteroid;
use model::lib::{Point, Vector2};
use model::player::{Player, Rotation};
use model::movingobject::*;
use sdl2::{event::Event, keyboard::Keycode};

mod view;
use view::playfield;

mod model;

fn main() {
    let mut playfield: playfield::Renderer = playfield::Renderer::new(800, 600);

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let mut player = Player::new(
        playfield.screen_width,
        playfield.screen_height,
        (playfield.screen_width / 2) as i16,
        (playfield.screen_height / 2) as i16);

    let mut asteroid = Asteroid {
        screen_width: playfield.screen_width,
        screen_height: playfield.screen_height,
        position: Point{x: 0, y: 0},
        velocity_vector: Vector2{ x: 1. , y: 1.},
    };

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

        playfield.render(&player, &asteroid);

        // don't know how this works exactly, but SDL2 docs say this is the way to limit to 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
