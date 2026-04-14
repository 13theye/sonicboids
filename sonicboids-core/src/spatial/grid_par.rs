//! Parallized version of GridIndex

use super::SpatialIndex;
use crate::sim::{Agent, AgentId};

use nannou::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct GridIndexPar {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<AgentId>>,
}

impl SpatialIndex for GridIndexPar {
    fn rebuild(&mut self, agents: &[Agent]) {
        self.cells.clear();

        self.rebuild_fully_par(agents);
    }

    /// Same as GridIndex: parallelism won't give gains here
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

impl GridIndexPar {
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

    /// Rebuild function with parallel cell assignment, sequential insertion
    #[allow(dead_code)]
    fn rebuild_par_assignment(&mut self, agents: &[Agent]) {
        // Compute (cell_key, agent_id, position) for every agent

        let assignments: Vec<((i32, i32), AgentId)> = agents
            .par_iter()
            .map(|a| {
                let idx = self.get_idx(a.position);
                (idx, a.id)
            })
            .collect();

        // Insert into cell map
        assignments.iter().for_each(|(cell_idx, agent_id)| {
            self.cells.entry(*cell_idx).or_default().push(*agent_id);
        });
    }

    /// Rebuild function with parallel cell assignment and parallel insertion
    fn rebuild_fully_par(&mut self, agents: &[Agent]) {
        // Alternative using Rayon's fold/reduce
        self.cells = agents
            .par_iter()
            .fold(
                HashMap::new,
                |mut map: HashMap<(i32, i32), Vec<usize>>, a| {
                    let idx = self.get_idx(a.position);
                    map.entry(idx).or_default().push(a.id);
                    map
                },
            )
            .reduce(HashMap::new, |mut a, b| {
                for (key, ids) in b {
                    a.entry(key).or_default().extend(ids);
                }
                a
            });
    }
}
