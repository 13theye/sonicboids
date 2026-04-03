//! Drawing functions for the Simulation

use sonicboids_core::sim::Flock;

use nannou::prelude::*;

pub fn draw_flock(draw: &Draw, flock: &Flock) {
    for agent in &flock.agents {
        let heading = if agent.velocity.length_squared() > 0.0 {
            agent.velocity.normalize()
        } else {
            Vec2::Y
        };
        let perpendicular = Vec2::new(-heading.y, heading.x);

        // Arrowhead with split tail
        let tip = agent.position + heading * 8.0;
        let tail_center = agent.position - heading * 6.0;
        let tail_left = tail_center - perpendicular * 5.0 + heading * 2.0;
        let tail_right = tail_center + perpendicular * 5.0 + heading * 2.0;
        let wing_left = agent.position - perpendicular * 4.0;
        let wing_right = agent.position + perpendicular * 4.0;

        draw.polygon().points([
            tip,
            wing_left,
            tail_left,
            agent.position,
            tail_right,
            wing_right,
        ]);
    }
}
