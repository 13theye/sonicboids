//! Main window view loop

use super::sim;
use crate::model::Model;

use nannou::prelude::*;

pub fn main_window(app: &App, model: &Model, frame: Frame) {
    execute_renderer_pipeline(app, model, frame);
}

/// Execute `sonicboids-renderer` ping-pong pipeline
pub fn execute_renderer_pipeline(app: &App, model: &Model, frame: Frame) {
    let rect = app.window_rect();
    let draw = app.draw();

    // 1. Draw the accumulated history texture (previous frame's output)
    // - Draw1 is for the off-screen texture
    draw.texture(model.feedback.borrow().read_texture());

    // 2. Draw a tinted overlay to fade/color-shift the history each frame.
    //    trail_tint alpha controls decay speed; rgb controls color shift direction.
    let t = model.sim.params.trail_tint;
    draw.rect()
        .wh(rect.wh())
        .rgba(t.red, t.green, t.blue, t.alpha);

    // 3. Draw current agents only (no history iteration)
    let max_accel = model.sim.params.max_force / model.sim.params.agent_mass;
    sim::draw_flock(
        &draw,
        &model.sim.flock,
        model.sim.params.max_speed,
        max_accel,
    );

    // 4. Render the draw context into the write texture, flip read <-> write
    let window = app.main_window();
    let device = window.device();
    model
        .feedback
        .borrow()
        .submit(device, &mut frame.command_encoder(), &draw);

    // 5. Display the result to screen and FPS counter
    // - Draw2 is for the frame
    let draw2 = app.draw();
    draw2.texture(model.feedback.borrow().read_texture());

    let fps_position = rect.top_right() - vec2(100.0, 50.0);
    draw2
        .text(&format!("FPS: {:.1}", model.fps.fps()))
        .xy(fps_position)
        .color(RED)
        .font_size(32);

    draw2.to_frame(app, &frame).unwrap();
}

/// Simply draw all agents
pub fn execute_simple_pipeline(app: &App, model: &Model, frame: Frame) {
    let rect = app.window_rect();
    let draw = app.draw();

    draw.background().color(BLACK);

    sim::draw_flock(
        &draw,
        &model.sim.flock,
        model.sim.params.max_speed,
        model.sim.params.max_force,
    );

    // Draw FPS
    let fps_position = rect.top_right() - vec2(100.0, 50.0);
    draw.rect()
        .xy(fps_position)
        .wh(vec2(150.0, 50.0))
        .rgba(0.0, 0.0, 0.0, 1.0);

    draw.text(&format!("FPS: {:.1}", model.fps.fps()))
        .xy(fps_position)
        .color(RED)
        .font_size(32);

    draw.to_frame(app, &frame).unwrap();
}
