//! Drawing functions for the Simulation

use sonicboids_core::sim::Flock;

use nannou::geom::Tri;
use nannou::prelude::*;
use rayon::prelude::*;

pub fn draw_all(draw: &Draw, flock: &Flock) {
    let now_color = Rgba::new(1.0, 1.0, 1.0, 1.0);
    let history_color = Rgba::new(1.0, 0.0, 0.0, 0.5);

    let tris: Vec<Tri<(Vec3, Rgba)>> = flock
        .agents
        .par_iter()
        .flat_map_iter(|agent| {
            let heading = if agent.velocity.length_squared() > 0.0 {
                agent.velocity.normalize()
            } else {
                Vec2::Y
            };
            let current = arrow_tris(agent.position, heading, now_color);
            let history = agent.history.inner.iter().flat_map(|(pos, vel)| {
                let h = if vel.length_squared() > 0.0 {
                    vel.normalize()
                } else {
                    Vec2::Y
                };
                arrow_tris(*pos, h, history_color)
            });
            history.into_iter().chain(current)
        })
        .collect();

    draw.mesh().tris_colored(tris);
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

/// Tessellate an arrow shape into 4 triangles (fan from tip).
/// The 6-vertex polygon [tip, wing_l, tail_l, pos, tail_r, wing_r] maps to
/// 4 triangles with tip as the shared vertex.
fn arrow_tris(position: Vec2, heading: Vec2, color: Rgba) -> [Tri<(Vec3, Rgba)>; 4] {
    let perp = Vec2::new(-heading.y, heading.x);
    let tip = (position + heading * 8.0).extend(0.0);
    let wing_l = (position - perp * 4.0).extend(0.0);
    let wing_r = (position + perp * 4.0).extend(0.0);
    let tail_ctr = position - heading * 6.0;
    let tail_l = (tail_ctr - perp * 5.0 + heading * 2.0).extend(0.0);
    let tail_r = (tail_ctr + perp * 5.0 + heading * 2.0).extend(0.0);
    let pos = position.extend(0.0);
    let c = color;
    [
        Tri([(tip, c), (wing_l, c), (tail_l, c)]),
        Tri([(tip, c), (tail_l, c), (pos, c)]),
        Tri([(tip, c), (pos, c), (tail_r, c)]),
        Tri([(tip, c), (tail_r, c), (wing_r, c)]),
    ]
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
