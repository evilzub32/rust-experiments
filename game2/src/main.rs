use std::time::Duration;
use model::asteroid::{Asteroid, Size};
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

    let mut asteroid = Asteroid::new(Size::Medium, playfield.screen_width, playfield.screen_height);

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

        if player.get_bounding_box().collides(&asteroid.entity.get_bounding_box()) {
            player.set_colliding(true);
            asteroid.entity.set_colliding(true);
        } else {
            player.set_colliding(false);
            asteroid.entity.set_colliding(false);
        }

        // check_collisions(vec![Box::new(asteroid), Box::new(player)]);

        playfield.render(&vec![&asteroid, &player]);

        // don't know how this works exactly, but SDL2 docs say this is the way to limit to 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
