use bevy::math::DVec2;
use bevy::prelude::*;

use crate::parser::disc::{Damping, DiscComp, Gravity, InverseMass, Radius, Velocity};
use crate::parser::plane::PlaneComp;
use crate::parser::segment::{Bias, Curve, SegmentComp};
use crate::parser::utils::{BouncingCoef, Collision, CollisionFlag, Position};
use crate::parser::vertex::VertexComp;
use crate::AppState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_discs,
                disc_disc_collision,
                disc_plane_collision,
                disc_straight_segment_collision,
                disc_vertex_collision,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn disc_disc_collision(
    mut discs: Query<(
        &DiscComp,
        &mut Position,
        &mut Velocity,
        &Radius,
        &InverseMass,
        &BouncingCoef,
        &Collision,
    )>,
) {
    let mut combinations = discs.iter_combinations_mut();
    while let Some([disc_a, disc_b]) = combinations.fetch_next() {
        let (
            disc_comp_a,
            mut position_a,
            mut velocity_a,
            radius_a,
            inv_mass_a,
            b_coef_a,
            collision_a,
        ) = disc_a;
        let (
            disc_comp_b,
            mut position_b,
            mut velocity_b,
            radius_b,
            inv_mass_b,
            b_coef_b,
            collision_b,
        ) = disc_b;

        if disc_comp_a.index == disc_comp_b.index {
            continue;
        }

        if collision_a.group & collision_b.mask == CollisionFlag::empty()
            || collision_b.group & collision_a.mask == CollisionFlag::empty()
        {
            continue;
        }

        let dist = position_a.0.distance(position_b.0);
        let sum_radius = radius_a.0 + radius_b.0;

        if dist > sum_radius {
            continue;
        }

        let normal = (position_a.0 - position_b.0).normalize();
        let mass_factor = inv_mass_a.0 / (inv_mass_a.0 + inv_mass_b.0);
        position_a.0 += normal * (sum_radius - dist) * mass_factor;
        position_b.0 -= normal * (sum_radius - dist) * (1.0 - mass_factor);

        let relative_velocity = velocity_a.0 - velocity_b.0;
        let normal_velocity = relative_velocity.dot(normal);

        if normal_velocity < 0.0 {
            let impulse = -(1.0 + b_coef_a.0 * b_coef_b.0) * normal_velocity;
            velocity_a.0 += normal * impulse * mass_factor;
            velocity_b.0 -= normal * impulse * (1.0 - mass_factor);
        }
    }
}

fn disc_plane_collision(
    mut discs: Query<(
        &mut Position,
        &mut Velocity,
        &Radius,
        &InverseMass,
        &BouncingCoef,
        &Collision,
    )>,
    planes: Query<(&PlaneComp, &BouncingCoef, &Collision)>,
) {
    for (mut position, mut velocity, radius, inv_mass, b_coef_disc, collision_disc) in
        discs.iter_mut()
    {
        for (plane_comp, b_coef_plane, collision_plane) in planes.iter() {
            if inv_mass.0 == 0.0 {
                continue;
            }

            if (collision_disc.group & collision_plane.mask) == CollisionFlag::empty()
                || (collision_plane.group & collision_disc.mask) == CollisionFlag::empty()
            {
                continue;
            }

            let norm_plane = plane_comp.normal.normalize();
            let dist = plane_comp.dist - position.0.dot(norm_plane) + radius.0;

            if dist <= 0.0 {
                continue;
            }

            position.0 += norm_plane * dist;
            let normal_velocity = velocity.0.dot(norm_plane);
            if normal_velocity < 0.0 {
                let impulse = -(1.0 + b_coef_disc.0 * b_coef_plane.0) * normal_velocity;
                velocity.0 += plane_comp.normal * impulse;
            }
        }
    }
}

fn segment_apply_bias(bias: &Bias, dist: f64, norm_segment: DVec2) -> (f64, DVec2) {
    let (mut b, mut d, mut n) = (bias.0, dist, norm_segment);
    if bias.0 == 0.0 && dist < 0.0 {
        (d, n) = (-dist, -norm_segment);
    } else if bias.0 < 0.0 {
        (b, d, n) = (-bias.0, -dist, -norm_segment);
    }

    if d < -b {
        (f64::INFINITY, n)
    } else {
        (d, n)
    }
}

fn disc_straight_segment_collision(
    mut discs: Query<
        (
            &mut Position,
            &mut Velocity,
            &Radius,
            &InverseMass,
            &BouncingCoef,
            &Collision,
        ),
        Without<VertexComp>,
    >,
    segments: Query<(&SegmentComp, &BouncingCoef, &Bias, &Collision), Without<Curve>>,
    vertexes: Query<(&VertexComp, &Position, &Collision)>,
) {
    let vertexes_vec = vertexes.iter().collect::<Vec<_>>();

    for (mut position, mut velocity, radius, inv_mass, b_coef_disc, collision_disc) in
        discs.iter_mut()
    {
        for (segment_comp, b_coef_segment, bias, collision_segment) in segments.iter() {
            if inv_mass.0 == 0.0 {
                continue;
            }

            if collision_disc.group & collision_segment.mask == CollisionFlag::empty()
                || collision_segment.group & collision_disc.mask == CollisionFlag::empty()
            {
                continue;
            }

            let vertex_a_pos = vertexes_vec.get(segment_comp.vertex_indices.0).unwrap().1;
            let vertex_b_pos = vertexes_vec.get(segment_comp.vertex_indices.1).unwrap().1;

            let segment_vec = vertex_b_pos.0 - vertex_a_pos.0;
            let disc_vertex_a_vec = position.0 - vertex_a_pos.0;
            let disc_vertex_b_vec = position.0 - vertex_b_pos.0;

            if segment_vec.dot(disc_vertex_a_vec) <= 0.0
                || segment_vec.dot(disc_vertex_b_vec) >= 0.0
            {
                continue;
            }

            let norm_segment = DVec2::new(segment_vec.y, -segment_vec.x).normalize();
            let dist = norm_segment.dot(disc_vertex_b_vec);
            let (dist_f, norm_segment_f) = segment_apply_bias(bias, dist, norm_segment);

            if dist_f > radius.0 {
                continue;
            }

            position.0 += norm_segment_f * (radius.0 - dist_f);
            let normal_velocity = velocity.0.dot(norm_segment_f);
            if normal_velocity < 0.0 {
                let impulse = -(1.0 + b_coef_disc.0 * b_coef_segment.0) * normal_velocity;
                velocity.0 += norm_segment_f * impulse;
            }
        }
    }
}

fn disc_vertex_collision(
    mut discs: Query<
        (
            &mut Position,
            &mut Velocity,
            &Radius,
            &InverseMass,
            &BouncingCoef,
            &Collision,
        ),
        Without<VertexComp>,
    >,
    vertexes: Query<(&Position, &Collision), With<VertexComp>>,
) {
    for (mut position, mut velocity, radius, inv_mass, b_coef_disc, collision_disc) in
        discs.iter_mut()
    {
        for (vertex_pos, collision_vertex) in vertexes.iter() {
            if inv_mass.0 == 0.0 {
                return;
            }

            if collision_disc.group & collision_vertex.mask == CollisionFlag::empty()
                || collision_vertex.group & collision_disc.mask == CollisionFlag::empty()
            {
                return;
            }

            let dist = position.0.distance(vertex_pos.0);
            if dist > radius.0 {
                return;
            }

            let norm_vertex = (position.0 - vertex_pos.0).normalize();
            position.0 += norm_vertex * (radius.0 - dist);
            let normal_velocity = velocity.0.dot(norm_vertex);
            if normal_velocity < 0.0 {
                let impulse = -(1.0 + b_coef_disc.0) * normal_velocity;
                velocity.0 += norm_vertex * impulse;
            }
        }
    }
}

fn update_discs(mut discs: Query<(&mut Position, &mut Velocity, &Gravity, &Damping)>) {
    for (mut position, mut velocity, gravity, damping) in discs.iter_mut() {
        position.0 += velocity.0;
        velocity.0 = (velocity.0 + gravity.0) * damping.0;
    }
}
