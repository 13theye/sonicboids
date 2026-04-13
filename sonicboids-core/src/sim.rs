//! The Boids simulation

mod agent;
mod flock;
mod params;

pub use agent::{Agent, AgentId};
pub use flock::Flock;
pub use params::{BoundsBehavior, SimParams};

use crate::{
    spatial::{BruteForceIndex, SpatialIndex},
    steering::{Alignment, Cohesion, Separation, SteeringRule},
};

use nannou::prelude::*;
use rayon::prelude::*;
use std::time::Duration;

pub struct Simulation {
    pub flock: Flock,
    pub params: SimParams,
    pub rules: Vec<Box<dyn SteeringRule>>,
    pub spatial: Box<dyn SpatialIndex>,
}

impl Simulation {
    pub fn new(params: SimParams) -> Self {
        let rules = init_rules();
        let spatial = Box::new(BruteForceIndex::new());

        Self {
            flock: Flock::new(params.agent_count, params.bounds),
            params,
            rules,
            spatial,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.spatial.rebuild(&self.flock.agents);

        // Compute forces for each agent
        let forces = self.par_generate_forces();

        // Apply forces
        self.par_apply_forces(forces, dt);
    }

    fn generate_forces(&self) -> Vec<Vec2> {
        self.flock
            .agents
            .iter()
            .map(|agent| {
                let neighbor_ids = self
                    .spatial
                    .neighbors_of(agent, self.params.perception_radius);

                let neighbors: Vec<&Agent> = neighbor_ids
                    .iter()
                    .filter_map(|id| self.flock.agents.get(*id))
                    .collect();

                self.rules
                    .iter()
                    .map(|rule| rule.apply(agent, &neighbors, &self.params))
                    .fold(Vec2::ZERO, |acc, f| acc + f)
            })
            .collect()
    }

    /// Applies a Vec of forces to each agent with the corresponding index
    fn apply_forces(&mut self, forces: Vec<Vec2>, dt: Duration) {
        let dt = dt.as_secs_f32();
        self.flock
            .agents
            .iter_mut()
            .zip(forces)
            .for_each(|(agent, force)| {
                agent.apply_force(force, &self.params);
                agent.integrate(dt, &self.params);
            });
    }

    fn par_generate_forces(&self) -> Vec<Vec2> {
        self.flock
            .agents
            .par_iter()
            .map(|agent| {
                let neighbor_ids = self
                    .spatial
                    .neighbors_of(agent, self.params.perception_radius);

                let neighbors: Vec<&Agent> = neighbor_ids
                    .iter()
                    .filter_map(|id| self.flock.agents.get(*id))
                    .collect();

                self.rules
                    .iter()
                    .map(|rule| rule.apply(agent, &neighbors, &self.params))
                    .fold(Vec2::ZERO, |acc, f| acc + f)
            })
            .collect()
    }

    fn par_apply_forces(&mut self, forces: Vec<Vec2>, dt: Duration) {
        let dt = dt.as_secs_f32();
        self.flock
            .agents
            .par_iter_mut()
            .zip(forces)
            .for_each(|(agent, force)| {
                agent.apply_force(force, &self.params);
                agent.integrate(dt, &self.params);
            });
    }

    pub fn flock(&self) -> &Flock {
        &self.flock
    }
}

fn init_rules() -> Vec<Box<dyn SteeringRule>> {
    vec![
        Box::new(Alignment {}),
        Box::new(Cohesion {}),
        Box::new(Separation {}),
    ]
}
