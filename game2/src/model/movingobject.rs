use sdl2::{render::Canvas, video::Window};

use super::lib::{Vector2, Point, Rotation};

pub trait MovingObject {
    fn get_screen_width(&self) -> u32;
    fn get_screen_height(&self) -> u32;

    fn get_position(&self) -> Point;
    fn set_position(&mut self, position: Point);

    fn get_velocity_vector(&self) -> Vector2;
    fn set_velocity_vector(&mut self, velocity: Vector2);

    fn get_rotation(&self) -> Rotation;
    fn set_rotation(&mut self, rotation: Rotation);

    fn get_angle_deg(&self) -> f32;
    fn set_angle_deg(&mut self, angle_deg: f32);

    fn get_turnrate(&self) -> f32;
    fn set_turnrate(&mut self, trunrate: f32);

    fn get_thrust_vector(&self) -> Vector2;
    fn set_thrust_vector(&mut self, thrust_vector: Vector2);

    fn get_thrust(&self) -> f32;
    fn set_thrust(&mut self, thrust: f32);

    fn get_max_speed(&self) -> f32;
    fn set_max_speed(&mut self, max_speed: f32);

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

    fn update(&mut self) {
        match self.get_rotation() {
            Rotation::Clockwise => {
                let mut angle_deg = self.get_angle_deg();
                angle_deg += self.get_turnrate();

                if angle_deg >= 360. {
                    angle_deg = angle_deg - 360.;
                }

                self.set_angle_deg(angle_deg);
            },
            Rotation::Counterclockwise => {
                let mut angle_deg = self.get_angle_deg();
                angle_deg -= self.get_turnrate();

                if angle_deg < 0. {
                    angle_deg = 360. + angle_deg;
                }

                self.set_angle_deg(angle_deg);
            },
            _ => {}
        }
        self.set_thrust_vector(Vector2{x: 0., y: self.get_thrust()}.rotate(self.get_angle_deg()));
        let mut velo = self.get_velocity_vector();
        velo = velo.add(&self.get_thrust_vector());

        // observe the speed limit
        let speed = velo.magnitude();
        if speed > self.get_max_speed() {
            let limiter = self.get_max_speed() / speed;
            velo = velo.multiply(limiter);
        }
        self.set_velocity_vector(velo);

        let mut pos = self.get_position();
        pos.x += velo.x.round() as i16;
        pos.y += velo.y.round() as i16;
        self.set_position(pos);

        self.wrap_around();
    }

    fn render(&self, canvas: &mut Canvas<Window>);
}