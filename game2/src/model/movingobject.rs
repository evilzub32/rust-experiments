use sdl2::{render::Canvas, video::Window, pixels::Color};
use sdl2::gfx::primitives::DrawRenderer;

use super::lib::{Vector2, Point, Rotation, BoundingBox};

pub struct MovingObject {
    // need screen dimension to calculate wrap-around
    pub screen_width: u32,
    pub screen_height: u32,

    pub position: Point,
    pub angle_deg: f32,
    pub rotation: Rotation,
    pub turnrate: f32,

    pub thrust_vector: Vector2,
    pub thrust: f32,

    pub velocity_vector: Vector2,

    pub max_speed: f32,
    pub max_thrust: f32,

    pub shape: Vec<Vector2>,
    rotated_poly: Vec<Vector2>,
}

impl MovingObject {
    // static new method as "constructor"
    pub fn new(screen_width: u32, screen_height: u32,
        polygon: Vec<Vector2>,
        x: Option<i16>,
        y: Option<i16>,
        turnrate: Option<f32>,
        max_speed: Option<f32>) -> MovingObject {

        MovingObject {
            screen_width: screen_width,
            screen_height: screen_height,
            shape: polygon.clone(),
            position: Point{
                x: x.unwrap_or((screen_width / 2) as i16),
                y: y.unwrap_or((screen_height / 2) as i16)
            },
            angle_deg: 0.,
            rotation: Rotation::None,
            turnrate: turnrate.unwrap_or(8.),
            thrust: 0.,
            thrust_vector: Vector2::new(),
            velocity_vector: Vector2::new(),
            max_speed: max_speed.unwrap_or(12.),
            max_thrust: 0.2,
            rotated_poly: polygon.clone(),
        }
    }

    fn wrap_around(&mut self) {
        let i16_screen_width = self.screen_width as i16;
        let i16_screen_height = self.screen_height as i16;

        if self.position.x > i16_screen_width {
            self.position.x = self.position.x - i16_screen_width;
        } else if self.position.x < 0 {
            self.position.x = i16_screen_width + self.position.x;
        }

        if self.position.y > i16_screen_height {
            self.position.y = self.position.y - i16_screen_height;
        } else if self.position.y < 0 {
            self.position.y = i16_screen_height + self.position.y;
        }
    }

    fn update_polygon(&mut self) {
        let mut v: Vec<Vector2> = Vec::new();
        for point in self.shape.iter() {
            let mut rnode = point.rotate(self.angle_deg);
            rnode.x += self.position.x as f32;
            rnode.y += self.position.y as f32;
            v.push(rnode);
        }
        self.rotated_poly = v;
    }

    pub fn increase_thrust(&mut self) {
        self.thrust = self.max_thrust;
    }

    pub fn decrease_thrust(&mut self) {
        self.thrust = 0.;
    }

    pub fn update(&mut self) {
        // Step 1: Calculate rotation angle
        //TODO: Maybe this can be simplified with numeric value for turnrate
        match self.rotation {
            Rotation::Clockwise => {
                self.angle_deg += self.turnrate;

                if self.angle_deg >= 360. {
                    self.angle_deg = self.angle_deg - 360.;
                }
            },
            Rotation::Counterclockwise => {
                self.angle_deg -= self.turnrate;

                if self.angle_deg < 0. {
                    self.angle_deg = 360. + self.angle_deg;
                }
            },
            _ => {}
        }

        // thrust value should be positive but initial orientation should be upwards -> negative y
        self.thrust_vector = Vector2{x: 0., y: -self.thrust}.rotate(self.angle_deg);

        // Step 2: add thrust vector to current velocity vector
        self.velocity_vector = self.velocity_vector.add(&self.thrust_vector);

        // Step 3: observe the speed limit
        let speed = self.velocity_vector.magnitude();
        if speed > self.max_speed {
            let limiter = self.max_speed / speed;
            self.velocity_vector = self.velocity_vector.multiply(limiter);
        }

        // Step 4: update center position
        self.position.x += self.velocity_vector.x.round() as i16;
        self.position.y += self.velocity_vector.y.round() as i16;

        // step 5: Calculate wrap-around for center position
        // TODO: This does not work well for large, slow objects (asteroids)
        self.wrap_around();

        self.update_polygon();
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, color: Color) {
        // SDL2 methods with aa_...: means "anti alias" :)

        let mut poly_x = Vec::new();
        let mut poly_y = Vec::new();

        for point in self.rotated_poly.iter() {
            poly_x.push(point.x.round() as i16);
            poly_y.push(point.y.round() as i16);
        }

        canvas.polygon(&poly_x, &poly_y, color).unwrap();
    }

    pub fn get_bounding_box(&self) -> BoundingBox {
        let mut bbox = BoundingBox::new();

        for node in self.rotated_poly.iter() {
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
