//! The Nannou app Model

use nannou_egui::Egui;
use prat::ClockService;
use prat::clockservice::BeatEvent;
use tokio::sync::broadcast;

use sonicboids_core::sim::Simulation;

pub struct Model {
    pub sim: Simulation,
}
