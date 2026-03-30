use super::SpatialIndex;
use crate::sim::{Agent, AgentId};

use nannou::prelude::*;
use std::collections::HashMap;

pub struct GridIndex {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<AgentId>>,
    agent_positions: Vec<(AgentId, Vec2)>,
}

impl SpatialIndex for GridIndex {
    fn rebuild(&mut self, agents: &[Agent]) {
        self.cells.clear();
        self.agent_positions.clear();
        agents.iter().for_each(|a| {
            let cell = (
                (a.position.x / self.cell_size) as i32,
                (a.position.y / self.cell_size) as i32,
            );
        });
    }

    fn neighbors_of(&self, agent: &Agent, radius: f32, all: &[Agent]) -> Vec<AgentId> {
        todo!()
    }
}

impl GridIndex {}
