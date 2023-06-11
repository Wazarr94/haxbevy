use bevy::{math::DVec2, prelude::*};
use serde_json::Value;
use std::collections::HashMap;

use super::{
    disc::{Disc, DiscRaw},
    hx_trait::Trait,
    utils::CollisionFlag,
};

#[derive(Debug, Clone)]
pub struct Ball(Disc);

impl Default for Ball {
    fn default() -> Self {
        let ball_disc = Disc {
            position: DVec2::ZERO,
            speed: DVec2::ZERO,
            gravity: DVec2::ZERO,
            radius: 10.0,
            inv_mass: 1.0,
            damping: 0.99,
            b_coef: 0.5,
            color: Color::WHITE,
            c_group: CollisionFlag::BALL,
            c_mask: CollisionFlag::ALL,
        };
        Ball(ball_disc)
    }
}

pub fn handle_ball(
    ball: &Option<Value>,
    discs: &mut Vec<Disc>,
    traits: &HashMap<String, Trait>,
) -> Ball {
    match ball.as_ref() {
        None => Ball::default(),
        Some(Value::String(s)) => {
            if s == "disc0" {
                let disc = discs.remove(0);
                Ball(disc)
            } else {
                panic!("ball must be either \"disc0\" or a disc object")
            }
        }
        Some(Value::Object(o)) => {
            // ball_physics never contains a "pos" field, which is mandatory
            // for DiscRaw. We add it here.
            let mut o_mut = o.clone();
            o_mut.insert(
                "pos".to_string(),
                Value::Array(vec![0.0.into(), 0.0.into()]),
            );
            let disc_raw: DiscRaw = serde_json::from_value(Value::Object(o_mut)).unwrap();
            let disc = disc_raw.to_disc(traits);
            Ball(disc)
        }
        _ => panic!("ball must be either \"disc0\" or a disc object"),
    }
}
