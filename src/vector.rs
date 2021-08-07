pub struct Vector {
    vx: i32,
    vy: i32,
}

impl Vector {
    fn zero() -> Vector {
        Vector { vx: 0, vy: 0 }
    }
    fn neighbours(&self) -> [Vector; 9] {
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
        ];
    }
}
