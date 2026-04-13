use super::SpatialIndex;
use crate::sim::{Agent, AgentId};

use nannou::prelude::*;
use std::collections::HashMap;

pub struct GridIndex {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<AgentId>>,
}

impl SpatialIndex for GridIndex {
    fn rebuild(&mut self, agents: &[Agent]) {
        self.cells.clear();
        agents.iter().for_each(|a| {
            let (x, y) = self.get_idx(a.position);
            self.cells.entry((x, y)).or_default().push(a.id);
        })
    }

    fn neighbors_of(&self, agent: &Agent, radius: f32) -> Vec<AgentId> {
        let mut neighbors = Vec::new();

        let (x, y) = self.get_idx(agent.position);
        // The radius converted to number of cells
        let cell_radius = (radius / self.cell_size).ceil() as i32;

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = (x + dx, y + dy);
                if let Some(ids) = self.cells.get(&cell) {
                    neighbors.extend(ids);
                }
            }
        }

        neighbors
    }
}

impl GridIndex {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    /// Helper function to compute the cell index of a position
    fn get_idx(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }
}
