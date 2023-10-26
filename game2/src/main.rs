use std::time::Duration;

use model::gameobjects::GameObject;
use sdl2::{event::Event, keyboard::Keycode};

mod view;
use view::playfield;

mod model;

fn main() {
    let mut playfield: playfield::Renderer = playfield::Renderer::new(800, 600);

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let mut player = GameObject::new(
        playfield.screen_width,
        playfield.screen_height,
        (playfield.screen_width / 2) as i16,
        (playfield.screen_height / 2) as i16);

    while running {

        // FIXME: Use states like this:
        // const Uint8* keystates = SDL_GetKeyboardState(NULL);

        // ...

        // if(keystates[SDL_SCANCODE_LEFT])
        //     player.moving = player.left;
        // else if(keystates[SDL_SCANCODE_RIGHT])
        //     player.moving = player.right;

        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => player.rotate_counterclockwise(),
                        Keycode::Right => player.rotate_clockwise(),
                        Keycode::Up => player.increase_thrust(),
                        _ => {}
                    }
                },
                Event::KeyUp { keycode: Some(keycode), ..} => {
                    match keycode {
                        Keycode::Up => {
                            player.decrease_thrust();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        player.update();
        playfield.render(&player);

        // don't know how this works exactly, but SDL2 docs say this is the way to limit to 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
