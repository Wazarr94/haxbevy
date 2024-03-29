use std::{
    collections::HashMap,
    f64::consts::PI,
    ops::{Deref, DerefMut},
};

use bevy::{math::DVec2, prelude::*};
use bevy_prototype_lyon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    hx_trait::{Trait, Traitable},
    utils::{arc, parse_collision, parse_color, BouncingCoef, Collision, CollisionFlag},
    vertex::Vertex,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SegmentRaw {
    v0: usize,
    v1: usize,
    b_coef: Option<f64>,
    curve: Option<f64>,
    curve_f: Option<f64>,
    bias: Option<f64>,
    c_group: Option<Vec<String>>,
    c_mask: Option<Vec<String>>,
    vis: Option<bool>,
    color: Option<Value>,
    #[serde(rename = "trait")]
    hx_trait: Option<String>,
}

impl Default for SegmentRaw {
    fn default() -> Self {
        SegmentRaw {
            v0: 0,
            v1: 0,
            b_coef: Some(1.0),
            curve: Some(0.0),
            curve_f: Some(0.0),
            bias: Some(0.0),
            c_group: Some(vec!["wall".to_string()]),
            c_mask: Some(vec!["all".to_string()]),
            vis: Some(true),
            color: Some(Value::String("000000".to_string())),
            hx_trait: None,
        }
    }
}

impl Traitable for SegmentRaw {
    fn apply_trait(&self, traits: &HashMap<String, Trait>) -> SegmentRaw {
        let tr_def = Trait::default();
        let tr_d = match &self.hx_trait {
            Some(tr_name) => traits.get(tr_name).unwrap_or(&tr_def),
            None => &tr_def,
        };
        let b_coef = self.b_coef.or(tr_d.b_coef);
        let curve = self.curve.or(tr_d.curve);
        let curve_f = self.curve_f.or(tr_d.curve_f);
        let bias = self.bias.or(tr_d.bias);
        let c_group = self.c_group.as_ref().or(tr_d.c_group.as_ref()).cloned();
        let c_mask = self.c_mask.as_ref().or(tr_d.c_mask.as_ref()).cloned();
        let vis = self.vis.or(tr_d.vis);
        let color = self.color.as_ref().or(tr_d.color.as_ref()).cloned();
        let hx_trait = self.hx_trait.clone();
        SegmentRaw {
            b_coef,
            curve,
            curve_f,
            bias,
            c_group,
            c_mask,
            vis,
            color,
            hx_trait,
            ..*self
        }
    }
}

impl SegmentRaw {
    pub fn apply_default(&self) -> SegmentRaw {
        let s_def = SegmentRaw::default();
        SegmentRaw {
            b_coef: self.b_coef.or(s_def.b_coef),
            curve: self.curve.or(s_def.curve),
            curve_f: self.curve_f.or(s_def.curve_f),
            bias: self.bias.or(s_def.bias),
            c_group: self.c_group.as_ref().or(s_def.c_group.as_ref()).cloned(),
            c_mask: self.c_mask.as_ref().or(s_def.c_mask.as_ref()).cloned(),
            vis: self.vis.or(s_def.vis),
            color: self.color.as_ref().or(s_def.color.as_ref()).cloned(),
            hx_trait: self.hx_trait.clone(),
            ..*self
        }
    }

    fn to_straight(&self, traits: &HashMap<String, Trait>) -> StraightSegment {
        let segment_raw = self.apply_trait(traits).apply_default();
        let vertex_indices = (segment_raw.v0, segment_raw.v1);
        let b_coef = segment_raw.b_coef.unwrap();
        let bias = segment_raw.bias.unwrap();
        let c_group = parse_collision(&segment_raw.c_group.unwrap());
        let c_mask = parse_collision(&segment_raw.c_mask.unwrap());
        let vis = segment_raw.vis.unwrap();
        let color = parse_color(&segment_raw.color.unwrap(), false);
        StraightSegment {
            vertex_indices,
            b_coef,
            bias,
            c_group,
            c_mask,
            vis,
            color,
        }
    }

    fn to_curved(&self, traits: &HashMap<String, Trait>) -> CurvedSegment {
        CurvedSegment::new(self, traits)
    }

    pub fn to_segment(&self, traits: &HashMap<String, Trait>) -> Segment {
        match self.curve_f {
            Some(curve_f) if curve_f != 0.0 => Segment::Curved(self.to_curved(traits)),
            _ => match self.curve {
                Some(curve) if curve != 0.0 => Segment::Curved(self.to_curved(traits)),
                _ => Segment::Straight(self.to_straight(traits)),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StraightSegment {
    pub vertex_indices: (usize, usize),
    pub vis: bool,
    pub color: Color,
    pub b_coef: f64,
    pub bias: f64,
    pub c_group: CollisionFlag,
    pub c_mask: CollisionFlag,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct SegmentComp {
    pub vertex_indices: (usize, usize),
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Bias(pub f64);

#[derive(Component, Debug, Clone, Copy)]
pub struct Curve(pub f64);

impl StraightSegment {
    fn spawn(&self, stadium_parent: &mut ChildBuilder, vertexes: &[Vertex], index: usize) {
        let v0 = vertexes.get(self.vertex_indices.0).unwrap();
        let v1 = vertexes.get(self.vertex_indices.1).unwrap();
        let z = 0.2 + index as f32 * 0.0001;

        stadium_parent.spawn((
            SegmentComp {
                vertex_indices: self.vertex_indices,
            },
            Bias(self.bias),
            BouncingCoef(self.b_coef),
            Collision {
                group: self.c_group,
                mask: self.c_mask,
            },
        ));

        if self.vis {
            stadium_parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(
                        Vec2::new(v0.position.x as f32, v0.position.y as f32),
                        Vec2::new(v1.position.x as f32, v1.position.y as f32),
                    )),
                    transform: Transform::from_xyz(0.0, 0.0, z),
                    ..default()
                },
                Stroke::new(self.color, 3.0),
            ));
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurvedSegment {
    pub base: StraightSegment,
    pub curve: f64,
}

impl Deref for CurvedSegment {
    type Target = StraightSegment;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CurvedSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CurvedUtils;

impl CurvedUtils {
    pub fn circle_center(vertex_0_pos: DVec2, vertex_1_pos: DVec2, curve: f64) -> DVec2 {
        let vec_center = (vertex_1_pos - vertex_0_pos) / 2.0;
        let circle_center_x = vertex_0_pos.x + vec_center.x - vec_center.y * curve;
        let circle_center_y = vertex_0_pos.y + vec_center.y + vec_center.x * curve;
        DVec2::new(circle_center_x, circle_center_y)
    }

    pub fn circle_radius(vertex_0_pos: DVec2, vertex_1_pos: DVec2, curve: f64) -> f64 {
        let center = CurvedUtils::circle_center(vertex_0_pos, vertex_1_pos, curve);
        (vertex_0_pos - center).length()
    }

    pub fn circle_tangents(vertex_0_pos: DVec2, vertex_1_pos: DVec2, curve: f64) -> (DVec2, DVec2) {
        let center = CurvedUtils::circle_center(vertex_0_pos, vertex_1_pos, curve);
        (vertex_1_pos - center, vertex_0_pos - center)
    }

    pub fn circle_angles(vertex_0_pos: DVec2, vertex_1_pos: DVec2, curve: f64) -> (f64, f64) {
        let center = CurvedUtils::circle_center(vertex_0_pos, vertex_1_pos, curve);
        let angle_0 = (vertex_0_pos.y - center.y).atan2(vertex_0_pos.x - center.x);
        let mut angle_1 = (vertex_1_pos.y - center.y).atan2(vertex_1_pos.x - center.x);
        while angle_1 < angle_0 {
            angle_1 += 2.0 * PI;
        }
        (angle_0, angle_1)
    }
}

impl CurvedSegment {
    pub fn new(raw_segment: &SegmentRaw, traits: &HashMap<String, Trait>) -> CurvedSegment {
        let base = raw_segment.to_straight(traits);
        let mut curved_segment = CurvedSegment { base, curve: 0.0 };

        let curve = raw_segment.curve.unwrap_or(0.0);
        let curve_f = raw_segment.curve_f.unwrap_or(0.0);
        let curve_final = curved_segment.get_curve(curve, curve_f);
        curved_segment.curve = curve_final;

        curved_segment
    }

    fn get_curve(&mut self, curve: f64, curve_f: f64) -> f64 {
        if curve_f != 0.0 {
            return curve_f;
        }
        let mut curve_value = curve;
        if curve_value < 0.0 {
            curve_value *= -1.0;
            self.bias *= -1.0;
            let tmp = self.vertex_indices.0;
            self.vertex_indices.0 = self.vertex_indices.1;
            self.vertex_indices.1 = tmp;
        }
        curve_value *= PI / 180.0;
        let lim_inf = 10.0 * PI / 180.0;
        let lim_sup = 340.0 * PI / 180.0;
        if curve_value > lim_inf && curve_value < lim_sup {
            curve_value = 1.0 / (curve_value / 2.0).tan();
        }
        curve_value
    }

    fn get_tolerance(&self, radius: f32) -> f32 {
        let tolerance = PI as f32 / 16.0;
        10.0 * tolerance / radius
    }

    fn spawn(&self, stadium_parent: &mut ChildBuilder, vertexes: &[Vertex], index: usize) {
        let z = 0.2 + index as f32 * 0.0001;

        let pos_0 = vertexes.get(self.vertex_indices.0).unwrap().position;
        let pos_1 = vertexes.get(self.vertex_indices.1).unwrap().position;
        let circle_angles = CurvedUtils::circle_angles(pos_0, pos_1, self.curve);
        let circle_radius = CurvedUtils::circle_radius(pos_0, pos_1, self.curve) as f32;
        let circle_center = CurvedUtils::circle_center(pos_0, pos_1, self.curve);
        let path = arc(
            Vec2::new(circle_center.x as f32, circle_center.y as f32),
            circle_radius,
            circle_angles.0 as f32,
            circle_angles.1 as f32,
            self.get_tolerance(circle_radius),
        );

        stadium_parent.spawn((
            SegmentComp {
                vertex_indices: self.vertex_indices,
            },
            Curve(self.curve),
            Bias(self.bias),
            BouncingCoef(self.b_coef),
            Collision {
                group: self.c_group,
                mask: self.c_mask,
            },
        ));

        if self.vis {
            stadium_parent.spawn((
                ShapeBundle {
                    path,
                    transform: Transform::from_xyz(0.0, 0.0, z),
                    ..default()
                },
                Stroke::new(self.color, 3.0),
            ));
        }
    }
}

#[derive(Debug, Clone)]
pub enum Segment {
    Straight(StraightSegment),
    Curved(CurvedSegment),
}

impl Segment {
    pub fn spawn(&self, stadium_parent: &mut ChildBuilder, vertexes: &[Vertex], index: usize) {
        match self {
            Segment::Straight(segment) => segment.spawn(stadium_parent, vertexes, index),
            Segment::Curved(segment) => segment.spawn(stadium_parent, vertexes, index),
        }
    }
}
