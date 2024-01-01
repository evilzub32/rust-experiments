use std::time::Duration;
// use model::asteroid::{Asteroid, Size};
use model::entity::{Entity, KeyListener, self};
use model::lib::{Vector2, Rotation};
use model::player::Player;
use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode};

mod view;
use view::playfield;
use view::renderable::Renderable;

mod model;

fn main() {
    let mut playfield: playfield::Renderer = playfield::Renderer::new(1024, 768);

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let realplayer = Box::new(Player {
        position: Vector2 {
            x: playfield.screen_width as f32 / 2.,
            y: playfield.screen_height as f32 / 2.,
        },
        angle_deg: 0.,
        turnrate: 0.,
        thrust_vector: Vector2::new(),
        thrust: 0.,
        velocity_vector: Vector2::new(),
        shape: vec![
            Vector2{x: 0., y: -20.},
            Vector2{x: 15., y: 20.},
            Vector2{x: -15., y: 20.},
        ],
        rotated_poly: vec![
            Vector2{x: 0., y: -20.},
            Vector2{x: 15., y: 20.},
            Vector2{x: -15., y: 20.},
        ],
        default_color: Color::WHITE,
        current_color: Color::WHITE,
        is_colliding: false,
    });

    // let mut player: Box<dyn Entity> = Box::new(Player::new (
    //     Vector2 {
    //         x: playfield.screen_width as f32 / 2.,
    //         y: playfield.screen_height as f32 / 2.,
    //     },
    //     vec![
    //         Vector2{x: 0., y: -20.},
    //         Vector2{x: 15., y: 20.},
    //         Vector2{x: -15., y: 20.},
    // ]));
    // let bla = *player;
    // *player.default_color = Color::YELLOW;

    // let mut asteroid = Asteroid::new(Size::Medium, playfield.screen_width, playfield.screen_height);

    let mut entities = vec![realplayer];//, asteroid.entity];

    // FIXME: Borrow-Checker does not think this is a good idea...
    // playfield.renderables.push(&asteroid);
    // playfield.renderables.push(&player);

    while running {
        for entity in entities.iter_mut() {
            
            // Events will arrive erratically so use state in game-object for smooth handling
            for event in event_queue.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        running = false;
                    },
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        entity.key_down(keycode);
                        // match keycode {
                        //     Keycode::Left => player.rotation = Rotation::Counterclockwise,
                        //     Keycode::Right => player.rotation = Rotation::Clockwise,
                        //     Keycode::Up => player.thrust = player.max_thrust,

                        //     Keycode::Escape => {
                        //         running = false;
                        //     },
                        //     _ => {}
                        // }
                    },
                    Event::KeyUp { keycode: Some(keycode), ..} => {
                        entity.key_up(keycode);
                        // match keycode {
                        //     Keycode::Left => player.rotation = Rotation::None,
                        //     Keycode::Right => player.rotation = Rotation::None,
                        //     Keycode::Up => {
                        //         player.thrust = 0.;
                        //     },
                        //     _ => {}
                        // }
                    },
                    _ => {}
                }
            }

            entity.update();
        }

        // if player.get_bounding_box().collides(&asteroid.entity.get_bounding_box()) {
        //     player.set_colliding(true);
        //     asteroid.entity.set_colliding(true);
        // } else {
        //     player.set_colliding(false);
        //     asteroid.entity.set_colliding(false);
        // }

        // check_collisions(vec![Box::new(asteroid), Box::new(player)]);
        for entity in entities.iter() {
            playfield.render(entity);
        }

        // don't know how this works exactly, but SDL2 docs say this is the way to limit to 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
