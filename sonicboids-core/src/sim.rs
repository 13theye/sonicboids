//! The Boids simulation

mod agent;
mod flock;
mod params;

pub use agent::{Agent, AgentId};
pub use flock::Flock;
pub use params::SimParams;

use crate::{spatial::SpatialIndex, steering::SteeringRule};

use nannou::prelude::*;

pub struct Simulation {
    pub flock: Flock,
    pub params: SimParams,
    pub rules: Vec<Box<dyn SteeringRule>>,
    pub spatial: Box<dyn SpatialIndex>,
}

impl Simulation {
    pub fn update(&mut self, dt: f32) {
        self.spatial.rebuild(&self.flock.agents);

        // Compute forces for each agent
        let forces: Vec<Vec2> = self
            .flock
            .agents
            .iter()
            .map(|agent| {
                let neighbor_ids = self.spatial.neighbors_of(
                    agent,
                    self.params.perception_radius,
                    &self.flock.agents,
                );

                let neighbors: Vec<&Agent> = neighbor_ids
                    .iter()
                    .filter_map(|id| self.flock.agents.get(*id))
                    .collect();

                self.rules
                    .iter()
                    .map(|rule| rule.apply(agent, &neighbors, &self.params))
                    .fold(Vec2::ZERO, |acc, f| acc + f)
            })
            .collect();

        // Apply forces
        self.flock
            .agents
            .iter_mut()
            .zip(forces)
            .for_each(|(agent, force)| {
                agent.apply_force(force, &self.params);
                agent.integrate(dt, &self.params);
            });
    }
}
