#[derive(Clone, Copy)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub struct BoundingBox {
    pub x_max: f32,
    pub y_max: f32,
    pub x_min: f32,
    pub y_min: f32,
}

#[derive(Clone, Copy)]
pub enum Rotation {
    None,
    Clockwise,
    Counterclockwise
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { x: 0., y: 0. }
    }

    pub fn rotate(&self, angle_deg: f32) -> Vector2 {
        // first, convert to rad
        let angle = angle_deg;
        let angle_rad = angle.to_radians();
        Vector2 {
            x: angle_rad.cos() * self.x - angle_rad.sin() * self.y,
            y: angle_rad.sin() * self.x + angle_rad.cos() * self.y,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn multiply(&self, num: f32) -> Vector2 {
        if num.is_nan() {
            Vector2 { x: self.x, y: self.y }
        } else {
            Vector2 { x: self.x * num, y: self.y * num }
        }
    }
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        BoundingBox { x_max: f32::MIN, y_max: f32::MIN, x_min: f32::MAX, y_min: f32::MAX }
    }

    pub fn is_left_of(&self, other: &BoundingBox) -> bool {
        self.x_max < other.x_min
    }

    pub fn is_right_of(&self, other: &BoundingBox) -> bool {
        self.x_min > other.x_max
    }

    pub fn is_above(&self, other: &BoundingBox) -> bool {
        self.y_min > other.y_max
    }

    pub fn is_below(&self, other: &BoundingBox) -> bool {
        self.y_max < other.y_min
    }

    pub fn collides(&self, other: &BoundingBox) -> bool {
        !(
            self.is_above(other)
            || self.is_below(other)
            || self.is_left_of(other)
            || self.is_right_of(other)
        )
    }
}