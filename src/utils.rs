#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn add(&self, vector: &Vector) -> Coordinate {
        Coordinate{ x: self.x + vector.vx, y: self.y + vector.vy }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vector {
    pub vx: i32,
    pub vy: i32,
}

impl Vector {
    pub fn zero() -> Vector {
        Vector { vx: 0, vy: 0 }
    }
}
