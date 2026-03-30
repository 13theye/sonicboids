//! Spatial index computation

mod brute_force;
mod grid;

use crate::sim::{Agent, AgentId};

pub trait SpatialIndex {
    /// Rebuild internal model of the spatial index
    fn rebuild(&mut self, agents: &[Agent]);
    /// Return the neighbors of the agent within a given radius
    fn neighbors_of(&self, agent: &Agent, radius: f32, all: &[Agent]) -> Vec<AgentId>;
}
