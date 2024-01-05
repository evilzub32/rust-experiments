use std::time::Duration;
use model::asteroid::{Asteroid, Size};
use model::entity::Entity;
use model::lib::Vector2;
use model::player::Player;
use sdl2::event::Event;

mod view;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use view::playfield;

mod model;

fn main() {
    let mut playfield: playfield::Renderer = playfield::Renderer::new(1024, 768);

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let player = Player::new (
        Vector2 {
            x: playfield.screen_width as f32 / 2.,
            y: playfield.screen_height as f32 / 2.,
        },
        vec![
            Vector2{x: 0., y: -20.},
            Vector2{x: 15., y: 20.},
            Vector2{x: -15., y: 20.},
        ]
    );

    let mut entities: Vec<Box<dyn Entity>> = vec![
        Box::new(player),
        Box::new(Asteroid::new(Size::Large)),
        Box::new(Asteroid::new(Size::Large)),
        Box::new(Asteroid::new(Size::Large)),
        Box::new(Asteroid::new(Size::Large)),
    ];

    while running {
        for i in 0..entities.len() {
            let (left, right) = entities.split_at_mut(i);

            let me = &mut right[0];
            // Events will arrive erratically so use state in game-object for smooth handling
            for event in event_queue.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        running = false;
                    },
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        me.key_down(keycode);
                        match keycode {
                            Keycode::Escape => {
                                running = false;
                            },
                            _ => {}
                        }
                    },
                    Event::KeyUp { keycode: Some(keycode), ..} => {
                        me.key_up(keycode);
                    },
                    _ => {}
                }
            }

            me.set_current_color(me.get_default_color());
            for other in left {
                if me.get_bounding_box().collides(&other.get_bounding_box()) {
                    me.set_current_color(Color::RED);
                    other.set_current_color(Color::RED);
                }
            }

            me.update();
        }

        // check_collisions(vec![Box::new(asteroid), Box::new(player)]);
        playfield.render(&entities);

        // don't know how this works exactly, but SDL2 docs say this is the way to limit to 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
