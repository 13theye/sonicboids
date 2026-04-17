//! Rules for agent behavior

use crate::sim::{Agent, AgentId, SimParams};

use nannou::prelude::*;

pub mod alignment;
pub use alignment::Alignment;
pub mod cohesion;
pub use cohesion::Cohesion;
pub mod separation;
pub use separation::Separation;

/// A SteeringRule defines a behavior for an `Agent`
pub trait SteeringRule: Send + Sync {
    /// Applies the rule to the `agent`. Returns a force vector.
    /// `neighbor_ids` are indices into `agents`.
    fn apply(&self, agent: &Agent, neighbor_ids: &[AgentId], agents: &[Agent], params: &SimParams) -> Vec2;
}
