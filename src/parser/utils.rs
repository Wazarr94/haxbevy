use bevy::{math::DVec2, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bitflags::bitflags;
use serde_json::Value;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct CollisionFlag: u16 {
        // the keys are uppercase because the parser is case sensitive
        const BALL = 1;
        const RED = 2;
        const BLUE = 4;
        const REDKO = 8;
        const BLUEKO = 16;
        const WALL = 32;
        const ALL = 63;
        const KICK = 64;
        const SCORE = 128;
        const C0 = 256;
        const C1 = 512;
        const C2 = 1024;
        const C3 = 2048;
  }
}

impl std::str::FromStr for CollisionFlag {
    type Err = bitflags::parser::ParseError;

    fn from_str(flags: &str) -> Result<Self, Self::Err> {
        // deal with none
        if flags == "none" {
            return Ok(Self::empty());
        }

        let upper_flags = flags.to_uppercase();
        Ok(Self(upper_flags.parse()?))
    }
}

pub fn parse_color(color_val: &Value, transparent_supported: bool) -> Color {
    // the value is either "transparent", a hex string, or an array of 3 ints
    // from the documentation, there are cases where transparent is not supported
    match color_val {
        Value::String(s) => {
            if s == "transparent" && !transparent_supported {
                panic!("Transparent color not supported")
            } else if s == "transparent" {
                Color::rgba_u8(0, 0, 0, 0)
            } else {
                let hex = u32::from_str_radix(s, 16).unwrap();
                let r: u8 = ((hex >> 16) & 0xFF) as u8;
                let g: u8 = ((hex >> 8) & 0xFF) as u8;
                let b: u8 = (hex & 0xFF) as u8;
                Color::rgb_u8(r, g, b)
            }
        }
        Value::Array(arr) => Color::rgb_u8(
            arr[0].as_u64().unwrap() as u8,
            arr[1].as_u64().unwrap() as u8,
            arr[2].as_u64().unwrap() as u8,
        ),
        _ => panic!("Invalid color value"),
    }
}

pub fn parse_collision(vec: &Vec<String>) -> CollisionFlag {
    let mut flag = CollisionFlag::empty();
    for s in vec {
        flag |= s.parse().unwrap();
    }
    flag
}

pub fn arc(center: Vec2, radius: f32, start_angle: f32, end_angle: f32, tolerance: f32) -> Path {
    let mut path_builder = PathBuilder::new();
    let mut angle = start_angle;
    path_builder.move_to(Vec2::new(
        center.x + radius * start_angle.cos(),
        center.y + radius * start_angle.sin(),
    ));
    while angle < end_angle {
        angle += tolerance;
        path_builder.line_to(Vec2::new(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        ));
    }
    path_builder.build()
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Team {
    Spectator = 1,
    Red = 2,
    Blue = 3,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub DVec2);

#[derive(Component, Debug, Clone, Copy)]
pub struct BouncingCoef(pub f64);

#[derive(Component, Debug, Clone, Copy)]
pub struct Collision {
    pub group: CollisionFlag,
    pub mask: CollisionFlag,
}
