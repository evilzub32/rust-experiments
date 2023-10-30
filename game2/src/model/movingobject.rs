use super::lib::{Vector2, Point, MAX_SPEED};

pub trait MovingObject {
    fn get_screen_width(&self) -> u32;
    fn get_screen_height(&self) -> u32;

    fn get_position(&self) -> Point;
    fn set_position(&mut self, position: Point);

    fn get_velocity_vector(&self) -> &Vector2;
    fn set_velocity_vector(&mut self, velocity: Vector2);

    // if value exceeds boundary, wrap it to the "left" side of the range and the other way round
    fn wrap_around(&mut self) {
        let i16_screen_width = self.get_screen_width() as i16;
        let i16_screen_height = self.get_screen_height() as i16;

        let mut pos = self.get_position();
        if pos.x > i16_screen_width {
            pos.x = pos.x - i16_screen_width;
        } else if pos.x < 0 {
            pos.x = i16_screen_width + pos.x;
        }

        if pos.y > i16_screen_height {
            pos.y = pos.y - i16_screen_height;
        } else if pos.y < 0 {
            pos.y = i16_screen_height + pos.y;
        }

        self.set_position(pos);
    }

    fn update_position(&mut self) {
        let velo = self.get_velocity_vector();

        // observe the speed limit
        let speed = velo.magnitude();
        if speed > MAX_SPEED {
            let limiter = MAX_SPEED / speed;
            self.set_velocity_vector(velo.multiply(limiter));
        }

        let mut pos = self.get_position();
        pos.x += self.get_velocity_vector().x.round() as i16;
        pos.y += self.get_velocity_vector().y.round() as i16;
        self.set_position(pos);

        self.wrap_around();
    }

    fn update(&mut self) -> ();
}