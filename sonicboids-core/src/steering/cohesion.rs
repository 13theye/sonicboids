//! The SteeringRule governing Cohesion: the attractive force between Agents

use super::SteeringRule;
use crate::sim::{Agent, SimParams};

use nannou::prelude::*;

/// The SteeringRule governing Cohesion
/// - Finds the average position of all neighbors,
/// - then applies an attractive force towards that position
pub struct Cohesion;

impl SteeringRule for Cohesion {
    fn apply(&self, agent: &Agent, neighbor_ids: &[&Agent], params: &SimParams) -> Vec2 {
        if neighbor_ids.is_empty() {
            return Vec2::ZERO;
        }

        let center = neighbor_ids
            .iter()
            .fold(Vec2::ZERO, |accum, n| accum + n.position)
            / neighbor_ids.len() as f32;
        let desired = (center - agent.position).normalize_or_zero() * params.max_speed;
        desired - agent.velocity
    }
}
