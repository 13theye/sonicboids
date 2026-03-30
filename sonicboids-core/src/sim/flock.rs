//! Flock: the collection of all agents

use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use rand::Rng;

use crate::sim::agent::Agent;

pub struct Flock {
    pub agents: Vec<Agent>,
}

impl Flock {
    pub fn new(n: usize, bounds: Rect) -> Self {
        let perlin = Perlin::default();
        let mut rng = rand::rng();
        let mut agents = Vec::with_capacity(n);
        let mut id = 0;

        while agents.len() < n {
            let x = rng.random_range(bounds.left()..=bounds.right());
            let y = rng.random_range(bounds.bottom()..=bounds.top());

            // Perlin returns [-1, 1]; remap to [0, 1] for use as probability
            let noise_val = (perlin.get([x as f64 * 0.01, y as f64 * 0.01]) + 1.0) / 2.0;

            if rng.random::<f64>() < noise_val {
                let angle = rng.random_range(0.0..TAU);
                let velocity = Vec2::new(angle.cos(), angle.sin());
                agents.push(Agent {
                    id,
                    position: Vec2::new(x, y),
                    velocity,
                    acceleration: Vec2::ZERO,
                });
                id += 1;
            }
        }

        Self { agents }
    }
}
