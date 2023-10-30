use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::model::asteroids::Asteroid;
use crate::model::movingobject::MovingObject;
use crate::model::player::Player;


pub struct Renderer {
    pub screen_width: u32,
    pub screen_height: u32,
    canvas: Canvas<Window>,
    pub sdl_context: Sdl,

}

impl Renderer {
    pub fn new(screen_width: u32, screen_height: u32) -> Renderer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Game2", screen_width, screen_height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        Self {
            screen_width: screen_width,
            screen_height: screen_height,
            canvas: window.into_canvas().build().unwrap(),
            sdl_context: sdl_context,
        }
    }



    pub fn render(&mut self, player: &Player, asteroid: &Asteroid) {
        self.canvas.set_draw_color(Color::RGB(0,0,0));
        self.canvas.clear();

        asteroid.render(&mut self.canvas);

        player.render(&mut self.canvas);

        self.canvas.present();
    }
}
