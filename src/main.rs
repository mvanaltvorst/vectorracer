mod vector;
use std::collections::HashMap;
use vector::Vector;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn add(&self, vector: &Vector) -> Coordinate {
        Coordinate{ x: self.x + vector.vx, y: self.y + vector.vy }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct GameState {
    c: Coordinate,
    v: Vector,
    checkpoint: bool,
}

impl GameState {
    fn new(x: i32, y: i32) -> GameState {
        GameState {
            c: Coordinate { x, y },
            v: Vector::zero(),
            checkpoint: false
        }
    }

    fn children(&self, width: i32, height: i32) -> Vec<GameState> {
        self.v
            .children()
            .iter()
            .map(|vector| GameState {
                c: self.c.add(vector),
                v: *vector,
                checkpoint: self.checkpoint || (self.c.y <= 4)
            })
            .filter(|vector| vector.c.x >= 0 && vector.c.y >= 0 && vector.c.x < width && vector.c.y < height)
            .collect()
    }
}

struct StateCache {
    previous: HashMap<GameState, GameState>
}

impl StateCache {
    fn new() -> StateCache {
        StateCache {
            previous: HashMap::new()
        }
    }

    fn set(&self, key: &GameState, value: &GameState) {

    }

    fn get(&self, key: &GameState) -> Option<&GameState> {
        self.previous.get(key)
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
    state_cache: StateCache,
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
            state_cache: StateCache::new()
        }
    }

    fn is_legal(gs: &GameState) -> bool {
        // checks whether a given gamestate is legal
        // only checks last move
        // e.g. go one move back, check if move was possible
        // we already assume that start and end coordinates are legal,
        // only check circle
        true
    }

    fn solve(&self) -> Vec<Coordinate> {
        let out: Vec<Coordinate> = Vec::new();
        let mut stack: Vec<GameState> = Vec::new();
        stack.push(
            GameState::new(self.start_x, self.start_y)
        );
        while let Some(top) = stack.pop() {
            println!("{}", stack.len());
            if top.checkpoint {
                println!("{:?}", top);
            }
            if top.checkpoint && top.c.x == self.start_x && top.c.y == self.start_y {
                break;
            }
            stack.extend(
                top.children(self.width, self.height)
                   .iter()
                   .copied()
                   .filter(Game::is_legal)
            )
        }
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

// bool intersectsCircle(int sx, int sy, int ex, int ey) {
//     sx -= CIRCLE_X;
//     sy -= CIRCLE_Y;
//     ex -= CIRCLE_X;
//     ey -= CIRCLE_Y;
//     int a = (ex - sx)*(ex - sx) + (ey - sx)*(ey - sy);
//     int b = 2
