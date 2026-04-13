//! Main window view loop

use super::sim;
use crate::model::Model;

use nannou::prelude::*;

pub fn main_window(app: &App, model: &Model, frame: Frame) {
    let rect = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);
    //sim::draw_flock(&draw, &model.sim.flock);
    //sim::draw_histories(&draw, &model.sim.flock);

    sim::draw_all(&draw, &model.sim.flock);

    let fps_position = rect.top_right() - vec2(100.0, 50.0);
    draw.text(&format!("FPS: {:.1}", model.fps.fps()))
        .xy(fps_position)
        .color(WHITE)
        .font_size(32);

    draw.to_frame(app, &frame).unwrap();
}
