//! Spatial index computation

pub mod brute_force;
pub use brute_force::BruteForceIndex;
pub mod grid;
pub use grid::GridIndex;
pub mod grid_par;
pub use grid_par::GridIndexPar;

use crate::sim::{Agent, AgentId};

pub trait SpatialIndex: Send + Sync {
    /// Rebuild internal model of the spatial index
    fn rebuild(&mut self, agents: &[Agent]);
    /// Populate `out` with the IDs of agents within `radius` of `agent`.
    /// `out` is cleared before writing.
    fn neighbors_of(&self, agent: &Agent, radius: f32, out: &mut Vec<AgentId>);
}
