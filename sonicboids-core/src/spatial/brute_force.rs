use super::SpatialIndex;
use crate::sim::{Agent, AgentId};

pub struct BruteForceIndex {}

impl SpatialIndex for BruteForceIndex {
    /// Nothing to rebuild
    fn rebuild(&mut self, _agents: &[Agent]) {}

    /// Brute force search of neighbors for a given agent
    fn neighbors_of(&self, agent: &Agent, radius: f32, all: &[Agent]) -> Vec<AgentId> {
        all.iter()
            .filter(|a| a.id != agent.id && a.position.distance(agent.position) < radius)
            .map(|a| a.id)
            .collect()
    }
}
