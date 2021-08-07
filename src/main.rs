mod vector;

struct GameState {
    x: i32,
    y: i32,
    v: vector::Vector,
}

struct Game {
    width: i32,
    height; i32,
    circle_x: i32,
    circle_y: i32,
    circle_r: i32
}

impl Game {
    fn new(width: i32, height: i32, circle_x: i32, circle_y: i32, circle_r: i32) {
        Game {
            width,
            height,
            circle_x,
            circle_y,
            circle_r
        }
    }
}

const WIDTH: i32 = 14;
const HEIGHT: i32 = 14;
const CIRCLE_X: i32 = 7;
const CIRCLE_Y: i32 = 7;
const CIRCLE_R: i32 = 3;

fn main() {
    println!("Hello, world!");
    let game = Game::new(WIDTH, HEIGHT, CIRCLE_X, CIRCLE_Y, CIRCLE_R);
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
