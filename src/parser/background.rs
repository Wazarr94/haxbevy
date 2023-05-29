use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::utils::parse_color;

const GRASS_BORDER_COLOR: Color = Color::rgb(0.78, 0.9, 0.74);
const GRASS_FILL_COLOR: Color = Color::rgb(0.44, 0.55, 0.35);

const HOCKEY_BORDER_COLOR: Color = Color::rgb(0.91, 0.8, 0.43);
const HOCKEY_FILL_COLOR: Color = Color::rgb(0.33, 0.33, 0.33);

#[derive(Serialize, Deserialize, Debug)]
pub enum BackgroundType {
    None,
    Grass,
    Hockey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundRaw {
    #[serde(rename = "type")]
    bg_type: Option<String>,
    width: Option<f64>,
    height: Option<f64>,
    kick_off_radius: Option<f64>,
    corner_radius: Option<f64>,
    goal_line: Option<f64>,
    color: Option<Value>,
}

impl Default for BackgroundRaw {
    fn default() -> Self {
        BackgroundRaw {
            bg_type: Some("none".to_string()),
            width: Some(0.0),
            height: Some(0.0),
            kick_off_radius: Some(0.0),
            corner_radius: Some(0.0),
            goal_line: Some(0.0),
            color: Some(Value::String("718C5A".to_string())),
        }
    }
}

impl BackgroundRaw {
    pub fn to_background(&self) -> Background {
        let background_raw = BackgroundRaw::default();
        let bg_type = match &self.bg_type {
            Some(t) => match t.as_str() {
                "grass" => BackgroundType::Grass,
                "hockey" => BackgroundType::Hockey,
                _ => BackgroundType::None,
            },
            None => match background_raw.bg_type.unwrap().as_str() {
                "none" => BackgroundType::None,
                _ => panic!("Invalid default background type"),
            },
        };
        let width = match self.width {
            Some(w) => w,
            None => background_raw.width.unwrap(),
        };
        let height = match self.height {
            Some(h) => h,
            None => background_raw.height.unwrap(),
        };
        let kick_off_radius = match self.kick_off_radius {
            Some(k) => k,
            None => background_raw.kick_off_radius.unwrap(),
        };
        let corner_radius = match self.corner_radius {
            Some(c) => c,
            None => background_raw.corner_radius.unwrap(),
        };
        let goal_line = match self.goal_line {
            Some(g) => g,
            None => background_raw.goal_line.unwrap(),
        };
        let color = match &self.color {
            Some(c) => parse_color(c, false),
            None => parse_color(&background_raw.color.unwrap(), false),
        };
        Background {
            bg_type,
            width,
            height,
            kick_off_radius,
            corner_radius,
            goal_line,
            color,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    #[serde(rename = "type")]
    pub bg_type: BackgroundType,
    pub width: f64,
    pub height: f64,
    pub kick_off_radius: f64,
    pub corner_radius: f64,
    pub goal_line: f64,
    pub color: Color,
}

impl Background {
    fn draw_limit(&self, commands: &mut Commands) {
        match self.bg_type {
            BackgroundType::Grass => {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Rectangle {
                            extents: Vec2::new(2.0 * self.width as f32, 2.0 * self.height as f32),
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, 0.01),
                        ..default()
                    },
                    Stroke::new(GRASS_BORDER_COLOR, 3.0),
                ));
            }
            BackgroundType::Hockey => {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Rectangle {
                            extents: Vec2::new(2.0 * self.width as f32, 2.0 * self.height as f32),
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, 0.01),
                        ..default()
                    },
                    Stroke::new(HOCKEY_BORDER_COLOR, 3.0),
                ));
            }
            _ => {}
        }
    }

    fn draw_kickoff_circle(&self, commands: &mut Commands) {
        match self.bg_type {
            BackgroundType::Grass => {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Circle {
                            radius: self.kick_off_radius as f32,
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, 0.11),
                        ..default()
                    },
                    Stroke::new(GRASS_BORDER_COLOR, 3.0),
                ));
            }
            BackgroundType::Hockey => {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Circle {
                            radius: self.kick_off_radius as f32,
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.0, 0.11),
                        ..default()
                    },
                    Stroke::new(HOCKEY_BORDER_COLOR, 3.0),
                ));
            }
            _ => {}
        }
    }

    fn draw_kickoff_line(&self, commands: &mut Commands) {
        if self.height == 0.0 {
            return;
        }

        match self.bg_type {
            BackgroundType::Grass => {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Line(
                            Vec2::new(0.0, -self.height as f32),
                            Vec2::new(0.0, self.height as f32),
                        )),
                        transform: Transform::from_xyz(0.0, 0.0, 0.12),
                        ..default()
                    },
                    Stroke::new(GRASS_BORDER_COLOR, 3.0),
                ));
            }
            BackgroundType::Hockey => {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Line(
                            Vec2::new(0.0, -self.height as f32),
                            Vec2::new(0.0, self.height as f32),
                        )),
                        transform: Transform::from_xyz(0.0, 0.0, 0.12),
                        ..default()
                    },
                    Stroke::new(HOCKEY_BORDER_COLOR, 3.0),
                ));
            }
            _ => {}
        }
    }

    fn fill_canvas(&self, commands: &mut Commands) {
        match self.bg_type {
            BackgroundType::Grass => {
                commands.insert_resource(ClearColor(GRASS_FILL_COLOR));
            }
            BackgroundType::Hockey => {
                commands.insert_resource(ClearColor(HOCKEY_FILL_COLOR));
            }
            _ => {
                commands.insert_resource(ClearColor(self.color));
            }
        }
    }

    pub fn draw(&self, commands: &mut Commands) {
        self.fill_canvas(commands);
        self.draw_limit(commands);
        self.draw_kickoff_circle(commands);
        self.draw_kickoff_line(commands);
    }
}
