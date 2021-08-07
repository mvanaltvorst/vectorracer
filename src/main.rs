mod vector;
use vector::Vector;

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn add(&self, vector: &Vector) -> Coordinate {
        Coordinate{ x: self.x + vector.vx, y: self.y + vector.vy }
    }
}

struct GameState {
    c: Coordinate,
    v: Vector,
}

impl GameState {
    fn new(x: i32, y: i32) -> GameState {
        GameState {
            c: Coordinate { x, y },
            v: Vector::zero(),
        }
    }

    fn children(&self, width: i32, height: i32) -> Vec<GameState> {
        self.v
            .children(width, height)
            .iter()
            .map(|vector| GameState {
                c: self.c.add(vector),
                v: *vector,
            })
            .collect()
    }
}

struct Game {
    width: i32,
    height: i32,
    start_x: i32,
    start_y: i32,
    circle_x: i32,
    circle_y: i32,
    circle_r: i32,
}

impl Game {
    fn new(width: i32, height: i32, start_x: i32, start_y: i32, circle_x: i32, circle_y: i32, circle_r: i32) -> Game {
        Game {
            width,
            height,
            start_x,
            start_y,
            circle_x,
            circle_y,
            circle_r,
        }
    }

    fn solve(&self) -> Vec<Coordinate> {
        let out: Vec<Coordinate> = Vec::new();
        let mut stack: Vec<GameState> = Vec::new();
        stack.push(
            GameState::new(self.start_x, self.start_y)
        );
        return out;
    }
}

const WIDTH: i32 = 14;
const HEIGHT: i32 = 14;
const START_X: i32 = 7;
const START_Y: i32 = 12;
const CIRCLE_X: i32 = 7;
const CIRCLE_Y: i32 = 7;
const CIRCLE_R: i32 = 3;

fn main() {
    let game = Game::new(
        WIDTH,
        HEIGHT, 
        START_X,
        START_Y,
        CIRCLE_X,
        CIRCLE_Y,
        CIRCLE_R
    );

    let path = game.solve();
    println!("{:?}", path);
}

// #include <iostream>

// struct state {
//     int x;
//     int y;
//     int vx;
//     int vy;
// };

// const int WIDTH = 14;
// const int HEIGHT = 14;
// const int CIRCLE_X = 7;
// const int CIRCLE_Y = 7;
// const int CIRCLE_RADIUS = 3;

// bool intersectsCircle(int sx, int sy, int ex, int ey) {
//     sx -= CIRCLE_X;
//     sy -= CIRCLE_Y;
//     ex -= CIRCLE_X;
//     ey -= CIRCLE_Y;
//     int a = (ex - sx)*(ex - sx) + (ey - sx)*(ey - sy);
//     int b = 2

// }

// int main() {

// }
