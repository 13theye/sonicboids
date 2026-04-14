//! The Nannou app Model

use std::cell::RefCell;

use fps::FpsManager;
use sonicboids_core::sim::Simulation;
use sonicboids_render::renderer::FeedbackRenderer;

pub struct Model {
    pub sim: Simulation,
    pub fps: FpsManager,
    pub feedback: RefCell<FeedbackRenderer>,
}
