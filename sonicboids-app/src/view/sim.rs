//! Drawing functions for the Simulation

use sonicboids_core::sim::Flock;

use nannou::geom::Tri;
use nannou::prelude::*;
use rayon::prelude::*;

/// Draw all `Agents` in the `Flock`.
///
/// Each agent's color is derived from its heading (hue) and speed + force (lightness).
pub fn draw_flock(draw: &Draw, flock: &Flock, max_speed: f32, max_accel: f32) {
    let tris: Vec<Tri<(Vec3, Rgba)>> = flock
        .agents
        .par_iter()
        .flat_map_iter(|agent| {
            let heading = if agent.velocity.length_squared() > 0.0 {
                agent.velocity.normalize()
            } else {
                Vec2::Y
            };
            let color = heading_speed_color(
                agent.heading(),
                agent.speed(),
                max_speed,
                agent.last_force,
                max_accel,
            );
            simple_arrow_tris(agent.position, heading, color)
        })
        .collect();

    draw.mesh().tris_colored(tris);
}

/// Map heading, speed, and applied force to an RGBA color.
///
/// Hue is derived from the heading angle in [-π, π].
/// Lightness combines normalized speed and normalized force, so agents
/// that are fast or under heavy steering pressure appear brighter.
fn heading_speed_color(
    heading: f32,
    speed: f32,
    max_speed: f32,
    last_force: f32,
    max_accel: f32,
) -> Rgba {
    let hue = (heading + std::f32::consts::PI) / std::f32::consts::TAU;
    let speed_t = (speed / max_speed).clamp(0.0, 1.0);
    let force_t = (last_force / max_accel).clamp(0.0, 1.0);
    let lightness = (0.15 + speed_t * 0.95 + force_t * 0.05).clamp(0.0, 1.0);
    hsl_to_rgba(hue, 1.0, lightness)
}

/// Convert HSL (all components in [0, 1]) to RGBA.
fn hsl_to_rgba(h: f32, s: f32, l: f32) -> Rgba {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    Rgba::new(r + m, g + m, b + m, 1.0)
}

/// Tessellate a single-triangle arrow: tip, left base, right base.
fn simple_arrow_tris(position: Vec2, heading: Vec2, color: Rgba) -> [Tri<(Vec3, Rgba)>; 1] {
    let perp = Vec2::new(-heading.y, heading.x);
    let tip = (position + heading * 8.0).extend(0.0);
    let base_l = (position - heading * 6.0 - perp * 5.0).extend(0.0);
    let base_r = (position - heading * 6.0 + perp * 5.0).extend(0.0);
    [Tri([(tip, color), (base_l, color), (base_r, color)])]
}

/// Tessellate an arrow shape into 4 triangles (fan from tip).
/// The 6-vertex polygon [tip, wing_l, tail_l, pos, tail_r, wing_r] maps to
/// 4 triangles with tip as the shared vertex.
#[allow(dead_code)]
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

/*
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
*/

/*
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
*/
