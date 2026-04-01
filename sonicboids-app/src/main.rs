use sonicboids_app::{model::Model, update::update, view::view};
use sonicboids_core::sim::{SimParams, Simulation};

use nannou::prelude::*;

fn main() {
    nannou::app(init_model).update(update).run();
}

fn init_model(app: &App) -> Model {
    let params = SimParams::default();

    let Ok(window_id) = app
        .new_window()
        .title("Sonic Boids")
        .size(1920, 1080)
        .msaa_samples(1)
        .view(view)
        .build()
    else {
        panic!("Failed to create WindowId");
    };

    let Some(_window) = app.window(window_id) else {
        panic!("Failed to create Window");
    };

    let sim = Simulation::new(params);

    Model { sim }
}
