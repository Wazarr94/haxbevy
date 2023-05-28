use super::utils::Team;
use bevy::math::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoalRaw {
    p0: [f64; 2],
    p1: [f64; 2],
    team: String,
}

impl GoalRaw {
    pub fn to_goal(&self) -> Goal {
        Goal {
            p0: DVec2::from(self.p0),
            p1: DVec2::from(self.p1),
            team: match self.team.as_str() {
                "red" => Team::Red,
                "blue" => Team::Blue,
                _ => panic!("Invalid team name"),
            },
        }
    }
}

#[derive(Debug)]
pub struct Goal {
    pub p0: DVec2,
    pub p1: DVec2,
    pub team: Team,
}
