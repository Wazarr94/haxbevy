use super::utils::Team;
use bevy::math::DVec2;
use bevy::prelude::*;
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

#[derive(Debug, Clone)]
pub struct Goal {
    pub p0: DVec2,
    pub p1: DVec2,
    pub team: Team,
}

#[derive(Component, Debug, Clone)]
pub struct GoalComp {
    pub p0: DVec2,
    pub p1: DVec2,
    pub team: Team,
}

impl Goal {
    pub fn spawn(&self, stadium_parent: &mut ChildBuilder) {
        stadium_parent.spawn(GoalComp {
            p0: self.p0,
            p1: self.p1,
            team: self.team.clone(),
        });
    }
}
