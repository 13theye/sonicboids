//! Rules for agent behavior

use crate::sim::{Agent, SimParams};

use nannou::prelude::*;

pub mod alignment;
pub mod cohesion;
pub mod separation;

/// A SteeringRule defines a behavior for an `Agent`
pub trait SteeringRule {
    /// Applies the rule to the `agent`. Returns a force vector.
    fn apply(&self, agent: &Agent, neighbors: &[&Agent], params: &SimParams) -> Vec2;
}
