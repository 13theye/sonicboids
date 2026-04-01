//! View update loop
//!

use crate::model::Model;
use sonicboids_core::sim::Flock;

use nannou::prelude::*;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_flock(&draw, &model.sim.flock);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_flock(draw: &Draw, flock: &Flock) {
    for agent in &flock.agents {
        draw.ellipse()
            .radius(5.0)
            .x_y(agent.position.x, agent.position.y);
    }
}
