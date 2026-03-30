//! The SteeringRule governing Separation

use super::SteeringRule;
use crate::sim::{Agent, SimParams};

use nannou::prelude::*;

/// The SteeringRule governing Separation
/// - Finds the position of all neighbors weighted by distance from the agent,
/// - then applies a repulsive force away from that position
pub struct Separation;

impl SteeringRule for Separation {
    fn apply(&self, agent: &Agent, neighbors: &[&Agent], params: &SimParams) -> Vec2 {
        if neighbors.is_empty() {
            return Vec2::ZERO;
        }

        let repulsion = neighbors.iter().fold(Vec2::ZERO, |accum, n| {
            let dp = agent.position - n.position;
            let distance = dp.length().max(f32::EPSILON); // avoid div by zero
            accum + dp / distance
        });

        let desired = repulsion.normalize_or_zero() * params.max_speed;
        desired - agent.velocity
    }
}
