use std::cell::RefCell;

use sonicboids_app::{model::Model, update::update, view::main_window::main_window};
use sonicboids_core::sim::{SimParams, Simulation};
use sonicboids_render::renderer::FeedbackRenderer;

use fps::FpsManager;
use nannou::prelude::*;

fn main() {
    nannou::app(init_model).update(update).run();
}

fn init_model(app: &App) -> Model {
    let window_w = 1920.0;
    let window_h = 1080.0;
    let params = SimParams::default();

    let Ok(window_id) = app
        .new_window()
        .title("Sonic Boids")
        .size(window_w as u32, window_h as u32)
        .msaa_samples(1)
        .view(main_window)
        .build()
    else {
        panic!("Failed to create WindowId");
    };

    let Some(_window) = app.window(window_id) else {
        panic!("Failed to create Window");
    };

    let window = app.main_window();
    let device = window.device();
    let feedback = RefCell::new(FeedbackRenderer::new(
        device,
        [window_w as u32, window_h as u32],
    ));

    let sim = Simulation::new(params);
    let fps = FpsManager::new_with(true, true);

    Model { sim, fps, feedback }
}
