//! Main window view loop

use super::sim;
use crate::model::Model;

use nannou::prelude::*;

pub fn main_window(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    sim::draw_flock(&draw, &model.sim.flock);
    draw.to_frame(app, &frame).unwrap();
}
