//! The SteeringRule governing Alignment

use super::SteeringRule;
use crate::sim::{Agent, SimParams};

use nannou::prelude::*;

/// The SteeringRule governing Alignment
/// - Finds the average velocity of all neighbors,
/// - then applies an attractive force towards that velocity
pub struct Alignment;

impl SteeringRule for Alignment {
    fn apply(&self, agent: &Agent, neighbor_ids: &[&Agent], params: &SimParams) -> Vec2 {
        if neighbor_ids.is_empty() {
            return Vec2::ZERO;
        }

        let avg_velocity = neighbor_ids.iter().fold(Vec2::ZERO, |accum, n| {
            let v = n.velocity;
            accum + v
        }) / neighbor_ids.len() as f32;

        let desired = avg_velocity.normalize_or_zero() * params.max_speed;
        desired * params.alignment_weight - agent.velocity
    }
}
