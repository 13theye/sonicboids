//! The SteeringRule governing Separation

use super::SteeringRule;
use crate::sim::{Agent, AgentId, SimParams};

use nannou::prelude::*;

/// The SteeringRule governing Separation
/// - Finds the position of all neighbors weighted by distance from the agent,
/// - then applies a repulsive force away from that position
pub struct Separation;

impl SteeringRule for Separation {
    fn apply(
        &self,
        agent: &Agent,
        neighbor_ids: &[AgentId],
        agents: &[Agent],
        params: &SimParams,
    ) -> Vec2 {
        if neighbor_ids.is_empty() {
            return Vec2::ZERO;
        }

        let sep_sq = params.separation_radius.powi(2);

        let repulsion = neighbor_ids.iter().fold(Vec2::ZERO, |accumulator, &id| {
            let dp = agent.position - agents[id].position;
            let dist_sq = dp.length_squared().max(f32::EPSILON);
            if dist_sq > sep_sq {
                return accumulator;
            }
            accumulator + dp / dist_sq
        });

        let desired = repulsion.normalize_or_zero() * params.max_speed;
        desired * params.separation_weight - agent.velocity
    }
}
