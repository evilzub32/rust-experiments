use sdl2::{keyboard::Keycode, pixels::Color};

use crate::view::renderable::Renderable;

use super::lib::{Vector2, BoundingBox};

pub trait KeyListener {
    fn key_down(&mut self, keycode: Keycode);
    fn key_up(&mut self, keycode: Keycode);
}

pub trait Entity: Renderable + KeyListener {
    fn get_turnrate(&self) -> f32;
    fn set_turnrate(&mut self, turnrate: f32);

    fn get_thrust(&self) -> f32;
    fn set_thrust(&mut self, thrust: f32);

    fn get_angle_deg(&self) -> f32;
    fn set_angle_deg(&mut self, angle_deg: f32);

    fn get_velocity_vector(&self) -> Vector2;
    fn set_velocity_vector(&mut self, velocity_vector: Vector2);

    fn get_position(&self) -> Vector2;
    fn set_position(&mut self, position: Vector2);

    fn get_original_shape(&self) -> &Vec<Vector2>;
    fn set_original_shape(&mut self, shape: Vec<Vector2>);
    
    fn get_rotated_shape(&self) -> &Vec<Vector2>;
    fn set_rotated_shape(&mut self, shape: Vec<Vector2>);

    fn is_colliding(&self) -> bool;
    fn set_colliding(&mut self, colliding: bool);

    fn get_default_color(&self) -> Color;
    fn set_default_color(&mut self, color: Color);

    fn get_current_color(&self) -> Color;
    fn set_current_color(&mut self, color: Color);

    // TODO: get rid of that!
    fn get_screen_width(&self) -> f32 {
        1024.
    }
    
    fn get_screen_height(&self) -> f32 {
        768.
    }

    fn get_max_speed(&self) -> f32 {
        12.
    }

    fn get_max_thrust(&self) -> f32 {
        0.2
    }

    fn get_max_turnrate(&self) -> f32 {
        4.
    }

    // static new method as "constructor"
    // fn new(starting_position: Vector2, polygon: Vec<Vector2>) -> Self;

    fn turn(&mut self) {
        self.set_angle_deg(self.get_angle_deg() + self.get_turnrate());

        if self.get_angle_deg() >= 360. {
            self.set_angle_deg(self.get_angle_deg() - 360.);
        } else if self.get_angle_deg() < 0. {
            self.set_angle_deg(self.get_angle_deg() + 360.);
        }
    }

    fn screen_border_wrap_around(&mut self) {
        let mut pos = self.get_position();
        let screen_width = self.get_screen_width();
        let screen_height = self.get_screen_height();

        if pos.x > screen_width {
            pos.x -= screen_width;
        } else if pos.x < 0. {
            pos.x += screen_width;
        }

        if pos.y > screen_height {
            pos.y -= screen_height;
        } else if pos.y < 0. {
            pos.y += screen_height;
        }

        self.set_position(pos);
    }

    fn update_polygon(&mut self) {
        let mut rot_shape: Vec<Vector2> = Vec::new();
        let pos = self.get_position();

        for point in self.get_original_shape().iter() {
            let mut rnode = point.rotate(self.get_angle_deg());
            rnode = pos.add(&rnode);
            
            rot_shape.push(rnode);
        }

        self.set_rotated_shape(rot_shape);
    }

    fn update(&mut self) {
        // Step 1: Calculate rotation angle
        self.turn();

        // thrust value should be positive but initial orientation should be upwards -> negative y
        let thrust_vector = Vector2{x: 0., y: -self.get_thrust()}.rotate(self.get_angle_deg());

        // Step 2: add thrust vector to current velocity vector
        let mut velocity_vector = self.get_velocity_vector();
        velocity_vector = velocity_vector.add(&thrust_vector);
        self.set_velocity_vector(velocity_vector);

        // Step 3: observe the speed limit
        let speed = self.get_velocity_vector().magnitude();
        if speed > self.get_max_speed() {
            let limiter = self.get_max_speed() / speed;
            self.set_velocity_vector(self.get_velocity_vector().multiply(limiter));
        }

        // Step 4: update center position
        self.set_position(self.get_position().add(&velocity_vector));

        // step 5: Calculate wrap-around for center position
        // TODO: This does not work well for large, slow objects (asteroids)
        self.screen_border_wrap_around();

        self.update_polygon();
    }

    fn get_bounding_box(&self) -> BoundingBox {
        let mut bbox = BoundingBox::new();

        for node in self.get_rotated_shape().iter() {
            if node.x > bbox.x_max{
                bbox.x_max = node.x;
            }
            if node.x < bbox.x_min {
                bbox.x_min = node.x
            }
            if node.y > bbox.y_max {
                bbox.y_max = node.y
            }
            if node.y < bbox.y_min {
                bbox.y_min = node.y
            }
        }

        bbox
    }

    

}
