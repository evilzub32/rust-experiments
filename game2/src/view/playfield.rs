use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::gfx::primitives::DrawRenderer;

use crate::model::gameobjects::GameObject;


pub struct Renderer {
    canvas: Canvas<Window>,
    pub sdl_context: Sdl,
}

impl Default for Renderer {
    fn default() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Game2", 800, 600)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        Self {
            canvas: window.into_canvas().build().unwrap(),
            sdl_context: sdl_context
        }
    }
}

impl Renderer {
    pub fn render(&mut self, player: &GameObject) {
        self.canvas.set_draw_color(Color::RGB(0,0,0));
        self.canvas.clear();
        self.canvas.aa_circle(player.x, player.y, 10, Color::RGB(255,255,255)).unwrap();
        // self.canvas.dr
        self.canvas.present();
    }
}
