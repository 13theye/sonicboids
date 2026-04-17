//! The Boids simulation

mod agent;
mod flock;
mod history;
mod params;

pub use agent::{Agent, AgentId};
pub use flock::Flock;
pub use history::History;
pub use params::{BoundsBehavior, SimParams};

// Allow unsed Spatial modules
#[allow(unused_imports)]
use crate::{
    spatial::{BruteForceIndex, GridIndex, GridIndexPar, SpatialIndex},
    steering::{Alignment, Cohesion, Separation, SteeringRule},
};

use nannou::prelude::*;
use rayon::prelude::*;
use std::cell::RefCell;
use std::time::Duration;

// Thread-local storage for neighbor IDs -- saves an allocation on the hot loop.
thread_local! {
    static NEIGHBOR_IDS: RefCell<Vec<AgentId>> = RefCell::new(Vec::with_capacity(256));
}

/// Helper function to initialize rules for testing
pub fn init_rules() -> Vec<Box<dyn SteeringRule>> {
    vec![
        Box::new(Alignment {}),
        Box::new(Cohesion {}),
        Box::new(Separation {}),
    ]
}

pub fn init_spatial(params: &SimParams) -> Box<dyn SpatialIndex> {
    Box::new(GridIndexPar::new(params.perception_radius, params.bounds))
}

pub struct Simulation {
    // A wrapper for Vec<Agents>
    flock: Flock,

    // The state of the physics simulation
    physics: Physics,

    // Simulation parameters
    params: SimParams,

    // Rules governing agent behavior
    rules: Vec<Box<dyn SteeringRule>>,

    // The graph of agent positions
    spatial: Box<dyn SpatialIndex>,
}

impl Simulation {
    pub fn new(
        params: SimParams,
        rules: Vec<Box<dyn SteeringRule>>,
        spatial: Box<dyn SpatialIndex>,
    ) -> Self {
        let physics = Physics::new(params.agent_count);

        Self {
            flock: Flock::new(params.agent_count, params.bounds),
            physics,
            params,
            rules,
            spatial,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.spatial.rebuild(&self.flock.agents);

        // Compute forces for each agent
        self.physics.forces = self.par_generate_forces();

        // Apply forces
        self.par_apply_forces(dt);

        // Update histories
        // This is disabled while history is created on the render side
        //self.flock.update_histories();
    }

    fn par_generate_forces(&self) -> Vec<Vec2> {
        self.flock
            .agents
            .par_iter()
            .map(|agent| {
                NEIGHBOR_IDS.with(|cell| {
                    let mut ids = cell.borrow_mut();
                    self.spatial
                        .neighbors_of(agent, self.params.perception_radius, &mut ids);

                    self.rules
                        .iter()
                        .map(|rule| rule.apply(agent, &ids, &self.flock.agents, &self.params))
                        .fold(Vec2::ZERO, |acc, f| acc + f)
                })
            })
            .collect()
    }

    /// Apply forces to each agent with the corresponding index, saving computed parameters to `Physics`
    fn par_apply_forces(&mut self, dt: Duration) {
        let dt = dt.as_secs_f32();
        let params = &self.params;
        let forces = &self.physics.forces;

        self.physics
            .accelerations
            .par_iter_mut()
            .zip(self.physics.delta_v.par_iter_mut())
            .zip(forces.par_iter())
            .zip(self.flock.agents.par_iter_mut())
            .for_each(|(((accel, dv), &force), agent)| {
                *accel = force.clamp_length_max(params.max_force) / params.agent_mass;
                *dv = *accel * dt;
                agent.apply_force(force, params);
                agent.integrate(dt, params);
            });
    }

    /*********** Single-threaded implementations ****************** */

    #[allow(dead_code)]
    fn generate_forces(&self) -> Vec<Vec2> {
        let mut ids = Vec::new();
        self.flock
            .agents
            .iter()
            .map(|agent| {
                self.spatial
                    .neighbors_of(agent, self.params.perception_radius, &mut ids);

                self.rules
                    .iter()
                    .map(|rule| rule.apply(agent, &ids, &self.flock.agents, &self.params))
                    .fold(Vec2::ZERO, |acc, f| acc + f)
            })
            .collect()
    }

    /// Applies a Vec of forces to each agent with the corresponding index, saving computed parameters to `Physics`
    #[allow(dead_code)]
    fn apply_forces(&mut self, dt: Duration) {
        let dt = dt.as_secs_f32();
        let params = &self.params;
        let forces = &self.physics.forces;

        self.physics
            .accelerations
            .iter_mut()
            .zip(self.physics.delta_v.iter_mut())
            .zip(forces.iter())
            .zip(self.flock.agents.iter_mut())
            .for_each(|(((accel, dv), &force), agent)| {
                *accel = force.clamp_length_max(params.max_force) / params.agent_mass;
                *dv = *accel * dt;
                agent.apply_force(force, params);
                agent.integrate(dt, params);
            });
    }

    /************* Getters ***************** */

    pub fn agents(&self) -> &[Agent] {
        &self.flock.agents
    }

    pub fn physics(&self) -> &Physics {
        &self.physics
    }

    pub fn params(&self) -> &SimParams {
        &self.params
    }
}

/// The state of the physics simulation
#[derive(Default, Clone)]
pub struct Physics {
    pub forces: Vec<Vec2>,
    pub accelerations: Vec<Vec2>,
    pub delta_v: Vec<Vec2>,
}

impl Physics {
    pub fn new(num_agents: usize) -> Self {
        Self {
            forces: vec![Vec2::ZERO; num_agents],
            accelerations: vec![Vec2::ZERO; num_agents],
            delta_v: vec![Vec2::ZERO; num_agents],
        }
    }
}
