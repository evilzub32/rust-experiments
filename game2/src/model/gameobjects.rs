pub struct GameObject {
    // choosing u32 for no particular reason
    pub x: u32,
    pub y: u32,

    pub angle_deg: i16,
}

impl GameObject {

    fn make_i16(n: u32) -> i16 {
        i16::try_from(n).unwrap()
    }

    // static new method as "constructor"
    pub fn new(x: u32, y: u32) -> GameObject {
        GameObject { x: x, y: y, angle_deg: 0 }
    }

    // Why do sdl2::gfx::DrawRenderer primitives take i16?
    // Would it better to also use i16 for my own struct instead of u32?
    pub fn x_i16(&self) -> i16 {
        GameObject::make_i16(self.x)
    }

    pub fn y_i16(&self) -> i16 {
        GameObject::make_i16(self.y)
    }

    pub fn rotate_clockwise(&mut self) {
        self.angle_deg += 1;
        if self.angle_deg >= 360 {
            self.angle_deg = self.angle_deg - 360;
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.angle_deg -= 1;
        if self.angle_deg < 0 {
            self.angle_deg = 360 + self.angle_deg;
        }
    }
}
