//! Appstate update

use crate::model::Model;

use nannou::prelude::*;

pub fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last;
    model.fps.update();
    model.sim.update(dt);
}
