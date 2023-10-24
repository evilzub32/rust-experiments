use model::gameobjects::GameObject;
use sdl2::event::Event;

mod view;
use view::playfield;

mod model;

fn main() {
    // let screen_width = 800;
    // let screen_height = 600;

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem.window("Rust!", screen_width, screen_height)
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas()
    //     .build()
    //     .unwrap();

    // let clear_color = Color::RGB(64, 192, 255);
    // canvas.set_draw_color(clear_color);

    // let screen_area = sdl2::rect::Rect::new(0,0, screen_width, screen_height);

    let mut playfield: playfield::Renderer = playfield::Renderer::default();

    let mut running = true;
    let mut event_queue = playfield.sdl_context.event_pump().unwrap();

    let mut player = GameObject {
        x: 400,
        y: 300
    };

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
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
