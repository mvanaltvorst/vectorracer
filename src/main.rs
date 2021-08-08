mod utils;
mod achievements;
mod game;
use game::Game;

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
    for coord in path {
        println!("{}, {}", coord.x, coord.y);
    }
}