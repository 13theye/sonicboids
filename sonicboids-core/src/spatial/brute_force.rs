use super::SpatialIndex;
use crate::sim::{Agent, AgentId};

#[derive(Default)]
pub struct BruteForceIndex {
    pub agents: Vec<Agent>,
}

impl SpatialIndex for BruteForceIndex {
    /// Nothing to rebuild
    fn rebuild(&mut self, agents: &[Agent]) {
        self.agents = agents.to_vec();
    }

    /// Brute force search of neighbors for a given agent
    fn neighbors_of(&self, agent: &Agent, radius: f32, out: &mut Vec<AgentId>) {
        out.clear();
        self.agents
            .iter()
            .filter(|a| a.id != agent.id && a.position.distance(agent.position) < radius)
            .for_each(|a| out.push(a.id));
    }
}

impl BruteForceIndex {
    pub fn new() -> Self {
        Self { agents: vec![] }
    }
}
