//! Tunable parameters for the simulation

use nannou::prelude::*;

pub struct SimParams {
    pub agent_count: usize,
    pub bounds: Rect,
    pub perception_radius: f32,
    pub separation_radius: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub bounds_behavior: BoundsBehavior,
}

impl Default for SimParams {
    fn default() -> Self {
        let rect_center = Vec2::ZERO;
        let rect_size = Vec2::new(1920.0, 1080.0);

        let bounds = Rect::from_xy_wh(rect_center, rect_size);

        Self {
            agent_count: 5000,
            bounds,
            perception_radius: 30.0,
            separation_radius: 20.0,
            max_speed: 600.0,
            max_force: 600.0,
            separation_weight: 1.0,
            alignment_weight: 0.3,
            cohesion_weight: 0.8,
            bounds_behavior: BoundsBehavior::Bounce,
        }
    }
}

pub enum BoundsBehavior {
    Bounce,
    Wrap,
    Through,
}
