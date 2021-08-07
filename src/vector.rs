#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vector {
    pub vx: i32,
    pub vy: i32,
}

impl Vector {
    pub fn zero() -> Vector {
        Vector { vx: 0, vy: 0 }
    }
    pub fn children(&self) -> Vec<Vector> {
        let Vector { vx, vy } = *self;
        return [
            Vector { vx: vx, vy: vy },
            Vector { vx: vx + 1, vy: vy },
            Vector {
                vx: vx + 1,
                vy: vy - 1,
            },
            Vector { vx: vx, vy: vy - 1 },
            Vector {
                vx: vx - 1,
                vy: vy - 1,
            },
            Vector { vx: vx - 1, vy: vy },
            Vector {
                vx: vx - 1,
                vy: vy + 1,
            },
            Vector { vx: vx, vy: vy + 1 },
            Vector {
                vx: vx + 1,
                vy: vy + 1,
            },
        ]
        .iter()
        .copied()
        .collect();
    }
}
