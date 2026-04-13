//! Drawing functions for the Simulation

use sonicboids_core::sim::Flock;

use nannou::prelude::*;

pub fn draw_all(draw: &Draw, flock: &Flock) {
    //let now_color = Rgba::new(1.0, 1.0, 1.0, 1.0);
    let history_color = Rgba::new(1.0, 0.0, 0.0, 0.5);

    flock.agents.iter().for_each(|agent| {
        /*
        let heading = if agent.velocity.length_squared() > 0.0 {
            agent.velocity.normal
        */

        agent.history.inner.iter().for_each(|(pos, vel)| {
            let heading = if vel.length_squared() > 0.0 {
                vel.normalize()
            } else {
                Vec2::Y
            };
            draw_agent(draw, *pos, heading, history_color);
        });

        //draw_agent(draw, agent.position, heading, now_color);
    });
}

/// Draw all `Agents` in the `Flock`
pub fn draw_flock(draw: &Draw, flock: &Flock) {
    let color = Rgba::new(1.0, 1.0, 1.0, 1.0);
    for agent in &flock.agents {
        let heading = if agent.velocity.length_squared() > 0.0 {
            agent.velocity.normalize()
        } else {
            Vec2::Y
        };

        draw_agent(draw, agent.position, heading, color);
    }
}

/// Draw all `Histories` in the `Flock`
pub fn draw_histories(draw: &Draw, flock: &Flock) {
    let color = Rgba::new(1.0, 0.0, 0.0, 0.5);
    flock.agents.iter().for_each(|agent| {
        agent.history.inner.iter().for_each(|(pos, vel)| {
            let heading = if vel.length_squared() > 0.0 {
                vel.normalize()
            } else {
                Vec2::Y
            };
            draw_agent(draw, *pos, heading, color);
        });
    });
}

fn draw_agent(draw: &Draw, position: Vec2, heading: Vec2, color: Rgba) {
    let perpendicular = Vec2::new(-heading.y, heading.x);

    // Arrowhead with split tail
    let tip = position + heading * 8.0;
    let tail_center = position - heading * 6.0;
    let tail_left = tail_center - perpendicular * 5.0 + heading * 2.0;
    let tail_right = tail_center + perpendicular * 5.0 + heading * 2.0;
    let wing_left = position - perpendicular * 4.0;
    let wing_right = position + perpendicular * 4.0;

    draw.polygon()
        .points([tip, wing_left, tail_left, position, tail_right, wing_right])
        .color(color);
}
