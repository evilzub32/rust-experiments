use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::model::lib::Vector2;

pub trait Renderable {
    fn get_polygon(&self) -> &Vec<Vector2>;
    fn get_color(&self) -> Color;
    fn render(&self, canvas: &Canvas<Window>) {
        // SDL2 methods with aa_...: means "anti alias" :)

        let mut poly_x = Vec::new();
        let mut poly_y = Vec::new();

        for point in self.get_polygon().iter() {
            poly_x.push(point.x.round() as i16);
            poly_y.push(point.y.round() as i16);
        }

        canvas.polygon(&poly_x, &poly_y, self.get_color()).unwrap();
    }
}