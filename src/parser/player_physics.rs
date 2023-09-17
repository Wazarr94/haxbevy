use bevy::math::DVec2;
use serde::{Deserialize, Serialize};

use super::utils::{parse_collision, CollisionFlag};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPhysicsRaw {
    pub gravity: Option<[f64; 2]>,
    pub radius: Option<f64>,
    pub inv_mass: Option<f64>,
    pub b_coef: Option<f64>,
    pub damping: Option<f64>,
    pub c_group: Option<Vec<String>>,
    pub acceleration: Option<f64>,
    pub kicking_acceleration: Option<f64>,
    pub kicking_damping: Option<f64>,
    pub kick_strength: Option<f64>,
    pub kickback: Option<f64>,
}

impl Default for PlayerPhysicsRaw {
    fn default() -> Self {
        Self {
            gravity: Some([0.0, 0.0]),
            radius: Some(15.0),
            inv_mass: Some(1.0),
            b_coef: Some(0.5),
            damping: Some(0.96),
            c_group: Some(vec![]),
            acceleration: Some(0.1),
            kicking_acceleration: Some(0.07),
            kicking_damping: Some(0.96),
            kick_strength: Some(5.0),
            kickback: Some(0.0),
        }
    }
}

impl PlayerPhysicsRaw {
    pub fn apply_default(&self) -> PlayerPhysicsRaw {
        let pp_def = PlayerPhysicsRaw::default();
        PlayerPhysicsRaw {
            gravity: self.gravity.or(pp_def.gravity),
            radius: self.radius.or(pp_def.radius),
            inv_mass: self.inv_mass.or(pp_def.inv_mass),
            b_coef: self.b_coef.or(pp_def.b_coef),
            damping: self.damping.or(pp_def.damping),
            c_group: self.c_group.as_ref().or(pp_def.c_group.as_ref()).cloned(),
            acceleration: self.acceleration.or(pp_def.acceleration),
            kicking_acceleration: self.kicking_acceleration.or(pp_def.kicking_acceleration),
            kicking_damping: self.kicking_damping.or(pp_def.kicking_damping),
            kick_strength: self.kick_strength.or(pp_def.kick_strength),
            kickback: self.kickback.or(pp_def.kickback),
        }
    }

    pub fn to_player_physics(&self) -> PlayerPhysics {
        let pp_def = self.apply_default();
        let gravity = DVec2::from(pp_def.gravity.unwrap());
        let radius = pp_def.radius.unwrap();
        let inv_mass = pp_def.inv_mass.unwrap();
        let b_coef = pp_def.b_coef.unwrap();
        let damping = pp_def.damping.unwrap();
        let c_group = parse_collision(&pp_def.c_group.unwrap());
        let acceleration = pp_def.acceleration.unwrap();
        let kicking_acceleration = pp_def.kicking_acceleration.unwrap();
        let kicking_damping = pp_def.kicking_damping.unwrap();
        let kick_strength = pp_def.kick_strength.unwrap();
        let kickback = pp_def.kickback.unwrap();
        PlayerPhysics {
            gravity,
            radius,
            inv_mass,
            b_coef,
            damping,
            c_group,
            acceleration,
            kicking_acceleration,
            kicking_damping,
            kick_strength,
            kickback,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPhysics {
    pub gravity: DVec2,
    pub radius: f64,
    pub inv_mass: f64,
    pub b_coef: f64,
    pub damping: f64,
    pub c_group: CollisionFlag,
    pub acceleration: f64,
    pub kicking_acceleration: f64,
    pub kicking_damping: f64,
    pub kick_strength: f64,
    pub kickback: f64,
}
