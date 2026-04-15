use super::SpatialIndex;
use crate::sim::{Agent, AgentId};

use nannou::prelude::*;

pub struct GridIndex {
    cell_size: f32,
    /// Flat row-major array of cells: index = row * grid_width + col
    cells: Vec<Vec<AgentId>>,
    grid_width: usize,
    grid_height: usize,
    /// Bottom-left corner of the world (bounds.left(), bounds.bottom())
    origin: Vec2,
}

impl SpatialIndex for GridIndex {
    fn rebuild(&mut self, agents: &[Agent]) {
        self.cells.iter_mut().for_each(|c| c.clear());

        agents.iter().for_each(|a| {
            let idx = self.flat_idx(a.position);
            self.cells[idx].push(a.id);
        });
    }

    fn neighbors_of(&self, agent: &Agent, radius: f32) -> Vec<AgentId> {
        let mut neighbors = Vec::new();

        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let (col, row) = self.cell_coords(agent.position);

        for dr in -cell_radius..=cell_radius {
            let r = row + dr;
            if r < 0 || r >= self.grid_height as i32 {
                continue;
            }
            for dc in -cell_radius..=cell_radius {
                let c = col + dc;
                if c < 0 || c >= self.grid_width as i32 {
                    continue;
                }
                let idx = r as usize * self.grid_width + c as usize;
                neighbors.extend_from_slice(&self.cells[idx]);
            }
        }

        neighbors
    }
}

impl GridIndex {
    pub fn new(cell_size: f32, bounds: Rect) -> Self {
        let origin = Vec2::new(bounds.left(), bounds.bottom());
        let grid_width = (bounds.w() / cell_size).ceil() as usize;
        let grid_height = (bounds.h() / cell_size).ceil() as usize;
        let cells = vec![Vec::new(); grid_width * grid_height];

        Self {
            cell_size,
            cells,
            grid_width,
            grid_height,
            origin,
        }
    }

    /// Returns the (col, row) cell coordinates for a position.
    fn cell_coords(&self, pos: Vec2) -> (i32, i32) {
        let col = ((pos.x - self.origin.x) / self.cell_size).floor() as i32;
        let row = ((pos.y - self.origin.y) / self.cell_size).floor() as i32;
        (col, row)
    }

    /// Returns the flat cell index for a position, clamped to grid bounds.
    /// Clamping handles agents that briefly exceed world bounds (BoundsBehavior::Through).
    fn flat_idx(&self, pos: Vec2) -> usize {
        let col = ((pos.x - self.origin.x) / self.cell_size)
            .floor()
            .clamp(0.0, (self.grid_width - 1) as f32) as usize;
        let row = ((pos.y - self.origin.y) / self.cell_size)
            .floor()
            .clamp(0.0, (self.grid_height - 1) as f32) as usize;
        row * self.grid_width + col
    }
}
