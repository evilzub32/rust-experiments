use model::gameobjects::GameObject;
use sdl2::{event::Event, keyboard::Keycode};

mod view;
use view::playfield;

mod model;

fn main() {
    let mut playfield: playfield::Renderer = playfield::Renderer::new(800, 600);

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let mut player = GameObject::new(playfield.screen_width / 2, playfield.screen_height / 2 );

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Left => player.rotate_counterclockwise(),
                        Keycode::Right => player.rotate_clockwise(),
                        _ => {}
                    }
                    println!("Rotation angle: {}", player.angle_deg);
                },
                // Event::MouseMotion { x, y, xrel, yrel, .. } => {
                //     // println!("Mouse: {},{}", x, y);
                //     // println!("Relative: {},{}", xrel, yrel);
                // },
                _ => {}
            }
        }

        playfield.render(&player);
    }
}
