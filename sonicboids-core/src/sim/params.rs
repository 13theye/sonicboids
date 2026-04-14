//! Tunable parameters for the simulation

use nannou::prelude::*;

pub struct SimParams {
    pub agent_count: usize,
    pub bounds: Rect,
    pub perception_radius: f32,
    pub separation_radius: f32,
    pub agent_mass: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub bounds_behavior: BoundsBehavior,
    // pub history_length: usize,
    // Color of the overlay drawn each frame over the accumulated texture.
    // Controls trail decay speed (via alpha) and color shift (via rgb).
    // e.g. Rgba::new(0.0, 0.0, 0.0, 0.04) for a simple fade to black.
    pub trail_tint: Rgba,
}

impl Default for SimParams {
    fn default() -> Self {
        let rect_center = Vec2::ZERO;
        let sim_size = Vec2::new(1920.0, 1080.0);

        let bounds = Rect::from_xy_wh(rect_center, sim_size);

        Self {
            agent_count: 20000,
            bounds,
            perception_radius: 100.0,
            separation_radius: 40.0,
            agent_mass: 2.5,
            max_speed: 1000.0,
            max_force: 600.0,
            separation_weight: 1.0,
            alignment_weight: 0.5,
            cohesion_weight: 0.7,
            bounds_behavior: BoundsBehavior::Bounce,
            //history_length: 8,
            trail_tint: Rgba::new(0.0, 0.0, 0.0, 0.02),
        }
    }
}

pub enum BoundsBehavior {
    Bounce,
    Wraparound,
    Through,
}
