extern crate enum_map;
use enum_map::{Enum, EnumMap};

use crate::utils::Coordinate;

#[derive(Enum)]
enum Achievement {
    TopRightCorner,
    TopLeftCorner
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct AchievementLogger {
    achieved: EnumMap<Achievement, bool>
}

impl AchievementLogger {
    pub fn new() -> AchievementLogger {
        AchievementLogger {
            achieved: EnumMap::default()
        }
    }
    pub fn all_achievements_met(&self) -> bool {
        self.achieved.iter().all(|(_, &visited)| visited)
    }

    pub fn log(&self, c: Coordinate) -> AchievementLogger {
        let mut achieved = self.achieved;
        if c.x >= 7 && c.y <= 4 {
            achieved[Achievement::TopRightCorner] = true;
        } else if c.x < 7 && c.y <= 4 {
            achieved[Achievement::TopLeftCorner] = true;
        }
        AchievementLogger {
            achieved
        }
    }
}