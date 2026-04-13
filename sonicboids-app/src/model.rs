//! The Nannou app Model

use fps::FpsManager;
use nannou_egui::Egui;
use prat::ClockService;
use prat::clockservice::BeatEvent;
use tokio::sync::broadcast;

use sonicboids_core::sim::Simulation;

pub struct Model {
    pub sim: Simulation,
    pub fps: FpsManager,
}
