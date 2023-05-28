use bevy::{math::DVec2, prelude::Color};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::{
    hx_trait::{Trait, Traitable},
    utils::{parse_collision, parse_color, CollisionFlag},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscRaw {
    pub pos: [f64; 2],
    pub speed: Option<[f64; 2]>,
    pub gravity: Option<[f64; 2]>,
    pub radius: Option<f64>,
    pub inv_mass: Option<f64>,
    pub damping: Option<f64>,
    pub b_coef: Option<f64>,
    pub color: Option<Value>,
    pub c_group: Option<Vec<String>>,
    pub c_mask: Option<Vec<String>>,
    #[serde(rename = "trait")]
    pub hx_trait: Option<String>,
}

impl Default for DiscRaw {
    fn default() -> Self {
        DiscRaw {
            pos: [0.0, 0.0],
            speed: Some([0.0, 0.0]),
            gravity: Some([0.0, 0.0]),
            radius: Some(8.0),
            inv_mass: Some(0.0),
            damping: Some(0.99),
            b_coef: Some(0.5),
            color: Some(Value::String("FFFFFF".to_string())),
            c_group: Some(vec!["all".to_string()]),
            c_mask: Some(vec!["all".to_string()]),
            hx_trait: None,
        }
    }
}

impl Traitable for DiscRaw {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> DiscRaw {
        let tr_def = Trait::default();
        let tr_d = match &self.hx_trait {
            Some(tr_name) => traits.get(tr_name).unwrap_or(&tr_def),
            None => &tr_def,
        };
        let radius = self.radius.or(tr_d.radius);
        let inv_mass = self.inv_mass.or(tr_d.inv_mass);
        let damping = self.damping.or(tr_d.damping);
        let b_coef = self.b_coef.or(tr_d.b_coef);
        let color = self.color.as_ref().or(tr_d.color.as_ref()).cloned();
        let c_group = self.c_group.as_ref().or(tr_d.c_group.as_ref()).cloned();
        let c_mask = self.c_mask.as_ref().or(tr_d.c_mask.as_ref()).cloned();
        let hx_trait = self.hx_trait.clone();
        DiscRaw {
            radius,
            inv_mass,
            damping,
            b_coef,
            color,
            c_group,
            c_mask,
            hx_trait,
            ..*self
        }
    }
}

impl DiscRaw {
    pub fn apply_default(&self) -> DiscRaw {
        let d_def: DiscRaw = DiscRaw::default();
        DiscRaw {
            pos: self.pos,
            speed: self.speed.or(d_def.speed),
            gravity: self.gravity.or(d_def.gravity),
            radius: self.radius.or(d_def.radius),
            inv_mass: self.inv_mass.or(d_def.inv_mass),
            damping: self.damping.or(d_def.damping),
            b_coef: self.b_coef.or(d_def.b_coef),
            color: self.color.as_ref().or(d_def.color.as_ref()).cloned(),
            c_group: self.c_group.as_ref().or(d_def.c_group.as_ref()).cloned(),
            c_mask: self.c_mask.as_ref().or(d_def.c_mask.as_ref()).cloned(),
            hx_trait: self.hx_trait.clone(),
        }
    }

    pub fn to_disc(&self, traits: &HashMap<String, Trait>) -> Disc {
        let disc_raw = self.apply_trait(traits).apply_default();
        let position = DVec2::from(disc_raw.pos);
        let speed = DVec2::from(disc_raw.speed.unwrap());
        let gravity = DVec2::from(disc_raw.gravity.unwrap());
        let radius = disc_raw.radius.unwrap();
        let inv_mass = disc_raw.inv_mass.unwrap();
        let damping = disc_raw.damping.unwrap();
        let b_coef = disc_raw.b_coef.unwrap();
        let color = parse_color(&disc_raw.color.unwrap(), true);
        let c_group = parse_collision(&disc_raw.c_group.unwrap());
        let c_mask = parse_collision(&disc_raw.c_mask.unwrap());
        Disc {
            position,
            speed,
            gravity,
            radius,
            inv_mass,
            damping,
            b_coef,
            color,
            c_group,
            c_mask,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Disc {
    pub position: DVec2,
    pub speed: DVec2,
    pub gravity: DVec2,
    pub radius: f64,
    pub inv_mass: f64,
    pub damping: f64,
    pub b_coef: f64,
    pub color: Color,
    pub c_group: CollisionFlag,
    pub c_mask: CollisionFlag,
}
