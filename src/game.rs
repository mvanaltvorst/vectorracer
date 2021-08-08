use std::collections::VecDeque;
use std::collections::HashMap;
use crate::utils::{Coordinate, Vector};
use crate::achievements::{AchievementLogger};


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct GameState {
    c: Coordinate,
    v: Vector,
    checkpoints: AchievementLogger,
}

impl GameState {
    fn new(x: i32, y: i32) -> GameState {
        GameState {
            c: Coordinate { x, y },
            v: Vector::zero(),
            checkpoints: AchievementLogger::new()
        }
    }

    fn children(&self, width: i32, height: i32) -> Vec<GameState> {
        let Vector { vx, vy } = self.v;
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
        .map(|vector| {
            GameState {
                c: self.c.add(&vector),
                v: vector,
                checkpoints: self.checkpoints.log(self.c)
            }
        })
        .filter(|gs| gs.c.x >= 0 && gs.c.y >= 0 && gs.c.x < width && gs.c.y < height)
        .collect();
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

    fn set(&mut self, key: GameState, value: GameState) {
        self.previous.insert(key, value);
    }

    fn get(&self, key: &GameState) -> Option<&GameState> {
        self.previous.get(key)
    }
}

pub struct Game {
    width: i32,
    height: i32,
    start_x: i32,
    start_y: i32,
    circle_x: i32,
    circle_y: i32,
    circle_r: i32,
}

impl Game {
    pub fn new(width: i32, height: i32, start_x: i32, start_y: i32, circle_x: i32, circle_y: i32, circle_r: i32) -> Game {
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

    fn is_legal(&self, gs: &GameState) -> bool {
        // checks whether a given gamestate is legal
        // only checks last move
        // e.g. go one move back, check if move was possible
        // we already assume that start and end coordinates are legal,
        // only check circle
        let e: Coordinate = gs.c;
        let s: Coordinate = Coordinate { x: gs.c.x - gs.v.vx, y: gs.c.y - gs.v.vy };

        // gamestate ends in circle
        if (e.x - self.circle_x)*(e.x - self.circle_x) + (e.y - self.circle_y)*(e.y - self.circle_y) < self.circle_r*self.circle_r {
            return false;
        }

        let dx: f32 = (e.x - s.x) as f32;
        let dy: f32 = (e.y - s.y) as f32;

        let d = ((e.x - s.x) * (s.y - self.circle_y) - (s.x - self.circle_x) * (e.y - s.y)).abs() as f32
                / (dx*dx + dy*dy).sqrt();
        if d < self.circle_r as f32 {
            return false;
        }

        true
    }

    pub fn solve(&self) -> Vec<Coordinate> {
        let mut out: Vec<Coordinate> = Vec::new();
        let mut queue: VecDeque<GameState> = VecDeque::new();
        let mut state_cache: StateCache = StateCache::new();
        queue.push_back(
            GameState::new(self.start_x, self.start_y)
        );
        let mut cur: Option<GameState> = None;
        while let Some(top) = queue.pop_front() {
            if top.checkpoints.all_achievements_met() && top.c.x == self.start_x && top.c.y == self.start_y {
                cur = Some(top);
                break;
            }

            let children = top.children(self.width, self.height);
            for child in children {
                if !self.is_legal(&child) {
                    continue;
                }
                if state_cache.get(&child).is_none() {
                    state_cache.set(child.clone(), top);
                    queue.push_back(child);
                }
            }
        }
        while let Some(current_gamestate) = cur {
            out.push(current_gamestate.c);
            cur = match state_cache.get(&current_gamestate) {
                Some(gmst) => {
                    if gmst.c.x == self.start_x && gmst.c.y == self.start_y && gmst.v.vx == 0 && gmst.v.vy == 0 {
                        break;
                    }
                    Some(*gmst)
                },
                None => None
            }
        }
        out.reverse();
        out
    }
}