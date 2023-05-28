use bevy::math::DVec2;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::background::{Background, BackgroundRaw};
use super::ball_physics::{handle_ball, Ball};
use super::disc::{Disc, DiscRaw};
use super::goal::{Goal, GoalRaw};
use super::hx_trait::handle_traits;
use super::plane::{Plane, PlaneRaw};
use super::player_physics::{PlayerPhysics, PlayerPhysicsRaw};
use super::segment::{Segment, SegmentRaw};
use super::vertex::{Vertex, VertexRaw};

#[derive(Serialize, Deserialize, Debug)]
pub enum CameraFollow {
    Player,
    Ball,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KickoffReset {
    Partial,
    Full,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StadiumRaw {
    name: String,
    bg: BackgroundRaw,
    width: Option<f64>,
    height: Option<f64>,
    camera_width: Option<f64>,
    camera_height: Option<f64>,
    max_view_width: Option<f64>,
    camera_follow: Option<String>,
    spawn_distance: Option<f64>,
    can_be_stored: Option<bool>,
    kick_off_reset: Option<String>,
    traits: Option<Value>,
    vertexes: Option<Vec<VertexRaw>>,
    segments: Option<Vec<SegmentRaw>>,
    goals: Option<Vec<GoalRaw>>,
    discs: Option<Vec<DiscRaw>>,
    planes: Option<Vec<PlaneRaw>>,
    red_spawn_points: Option<Vec<Vec<f64>>>,
    blue_spawn_points: Option<Vec<Vec<f64>>>,
    player_physics: Option<PlayerPhysicsRaw>,
    ball_physics: Option<Value>,
}

impl Default for StadiumRaw {
    fn default() -> Self {
        StadiumRaw {
            name: "".to_string(),
            bg: BackgroundRaw::default(),
            width: Some(0.0),
            height: Some(0.0),
            camera_width: Some(0.0),
            camera_height: Some(0.0),
            max_view_width: Some(0.0),
            camera_follow: Some("ball".to_string()),
            spawn_distance: Some(200.0),
            can_be_stored: Some(true),
            kick_off_reset: Some("partial".to_string()),
            traits: Some(Value::Array(vec![])),
            vertexes: Some(vec![]),
            segments: Some(vec![]),
            goals: Some(vec![]),
            discs: Some(vec![]),
            planes: Some(vec![]),
            red_spawn_points: Some(vec![]),
            blue_spawn_points: Some(vec![]),
            player_physics: Some(PlayerPhysicsRaw::default()),
            ball_physics: None,
        }
    }
}

impl StadiumRaw {
    pub fn apply_default(&self) -> StadiumRaw {
        let s_def = StadiumRaw::default();
        StadiumRaw {
            name: self.name.clone(),
            bg: self.bg.clone(),
            width: self.width.or(s_def.width),
            height: self.height.or(s_def.height),
            camera_width: self.camera_width.or(s_def.camera_width),
            camera_height: self.camera_height.or(s_def.camera_height),
            max_view_width: self.max_view_width.or(s_def.max_view_width),
            camera_follow: self.camera_follow.clone().or(s_def.camera_follow),
            spawn_distance: self.spawn_distance.or(s_def.spawn_distance),
            can_be_stored: self.can_be_stored.or(s_def.can_be_stored),
            kick_off_reset: self.kick_off_reset.clone().or(s_def.kick_off_reset),
            traits: self.traits.clone().or(s_def.traits),
            vertexes: self.vertexes.clone().or(s_def.vertexes),
            segments: self.segments.clone().or(s_def.segments),
            goals: self.goals.clone().or(s_def.goals),
            discs: self.discs.clone().or(s_def.discs),
            planes: self.planes.clone().or(s_def.planes),
            red_spawn_points: self.red_spawn_points.clone().or(s_def.red_spawn_points),
            blue_spawn_points: self.blue_spawn_points.clone().or(s_def.blue_spawn_points),
            player_physics: self.player_physics.clone().or(s_def.player_physics),
            ball_physics: self.ball_physics.clone().or(s_def.ball_physics),
        }
    }

    pub fn to_stadium(&self) -> Stadium {
        let s_default = self.apply_default();
        let traits = handle_traits(s_default.traits.unwrap());
        let bg = self.bg.to_background();
        let width = s_default.width.unwrap();
        let height = s_default.height.unwrap();
        let camera_width = s_default.camera_width.unwrap();
        let camera_height = s_default.camera_height.unwrap();
        let max_view_width = s_default.max_view_width.unwrap();
        let camera_follow = match s_default.camera_follow.unwrap().as_str() {
            "player" => CameraFollow::Player,
            "ball" => CameraFollow::Ball,
            _ => CameraFollow::Ball,
        };
        let spawn_distance = s_default.spawn_distance.unwrap();
        let can_be_stored = s_default.can_be_stored.unwrap();
        let kick_off_reset = match s_default.kick_off_reset.unwrap().as_str() {
            "partial" => KickoffReset::Partial,
            "full" => KickoffReset::Full,
            _ => KickoffReset::Partial,
        };
        let vertexes = s_default
            .vertexes
            .clone()
            .unwrap()
            .iter()
            .map(|v| v.to_vertex(&traits))
            .collect();
        let segments = s_default
            .segments
            .clone()
            .unwrap()
            .iter()
            .map(|s| s.to_segment(&traits))
            .collect();
        let mut discs: Vec<Disc> = s_default
            .discs
            .clone()
            .unwrap()
            .iter()
            .map(|d| d.to_disc(&traits))
            .collect();
        let goals = s_default
            .goals
            .clone()
            .unwrap()
            .iter()
            .map(|g| g.to_goal())
            .collect();
        let planes = s_default
            .planes
            .clone()
            .unwrap()
            .iter()
            .map(|p| p.to_plane(&traits))
            .collect();
        let red_spawn_points = s_default
            .red_spawn_points
            .clone()
            .unwrap()
            .iter()
            .map(|p| DVec2::new(p[0], p[1]))
            .collect();
        let blue_spawn_points = s_default
            .blue_spawn_points
            .clone()
            .unwrap()
            .iter()
            .map(|p| DVec2::new(p[0], p[1]))
            .collect();
        let player_physics = s_default.player_physics.unwrap().to_player_physics();
        let ball_physics = handle_ball(&s_default.ball_physics, &mut discs, &traits);
        Stadium {
            name: self.name.clone(),
            bg,
            width,
            height,
            camera_width,
            camera_height,
            max_view_width,
            camera_follow,
            spawn_distance,
            can_be_stored,
            kick_off_reset,
            vertexes,
            segments,
            goals,
            discs,
            planes,
            red_spawn_points,
            blue_spawn_points,
            player_physics,
            ball_physics,
        }
    }
}

pub struct Stadium {
    pub name: String,
    pub bg: Background,
    pub width: f64,
    pub height: f64,
    pub camera_width: f64,
    pub camera_height: f64,
    pub max_view_width: f64,
    pub camera_follow: CameraFollow,
    pub spawn_distance: f64,
    pub can_be_stored: bool,
    pub kick_off_reset: KickoffReset,
    pub vertexes: Vec<Vertex>,
    pub segments: Vec<Segment>,
    pub goals: Vec<Goal>,
    pub discs: Vec<Disc>,
    pub planes: Vec<Plane>,
    pub red_spawn_points: Vec<DVec2>,
    pub blue_spawn_points: Vec<DVec2>,
    pub player_physics: PlayerPhysics,
    pub ball_physics: Ball,
}
