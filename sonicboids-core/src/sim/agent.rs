//! The Simulation agent for 2D space

use super::{BoundsBehavior, SimParams};

use nannou::prelude::*;

/// Identifier for the `Agent`
pub type AgentId = usize;

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    /// Magnitude of the net acceleration applied last frame (before reset).
    pub last_force: f32,
    //pub history: History,
}

impl Agent {
    /// Applies a force to the `Agent`, consuming the force.
    /// Acceleration is scaled by 1/mass (F = ma → a = F/m), so heavier
    /// agents are slower to change direction.
    pub fn apply_force(&mut self, force: Vec2, params: &SimParams) {
        self.acceleration += force.clamp_length_max(params.max_force) / params.agent_mass;
    }

    /// Apply the acceleration and velocity to the position
    pub fn integrate(&mut self, dt: f32, params: &SimParams) {
        let dv = self.acceleration * dt;
        self.velocity = (self.velocity + dv).clamp_length_max(params.max_speed);
        self.position += self.velocity * dt;

        // Check bounds behavior & apply special behavior if applicable
        if !params.bounds.contains(self.position) {
            match params.bounds_behavior {
                BoundsBehavior::Bounce => {
                    self.velocity = bounce(self.position, self.velocity, &params.bounds);
                }
                BoundsBehavior::Wraparound => {
                    self.position = wrap_position(self.position, &params.bounds);
                }
                BoundsBehavior::Through => {}
            }
        }

        // Capture force magnitude before reset
        self.last_force = self.acceleration.length();

        // Reset acceleration
        self.acceleration = Vec2::ZERO;
    }

    /// Angle of the velocity vector in radians
    pub fn heading(&self) -> f32 {
        self.velocity.y.atan2(self.velocity.x)
    }

    /// Magnitude of the velocity vector
    pub fn speed(&self) -> f32 {
        self.velocity.length()
    }
}

/// Helper function to wrap position if exceeding bounds.
/// Returns the wrapped position
fn wrap_position(position: Vec2, bounds: &Rect) -> Vec2 {
    let mut position = position;
    if position.x < bounds.left() {
        let diff = position.x - bounds.left();
        position.x = bounds.right() + diff;
    } else if position.x > bounds.right() {
        let diff = position.x - bounds.right();
        position.x = bounds.left() + diff;
    }
    if position.y < bounds.bottom() {
        let diff = position.y - bounds.bottom();
        position.y = bounds.top() + diff;
    } else if position.y > bounds.top() {
        let diff = position.y - bounds.top();
        position.y = bounds.bottom() + diff;
    }
    position
}

/// Helper function to bounce off the bounds
fn bounce(position: Vec2, incoming_velocity: Vec2, bounds: &Rect) -> Vec2 {
    let mut velocity = incoming_velocity;
    if position.x < bounds.left() || position.x > bounds.right() {
        velocity.x *= -1.0;
    }

    if position.y < bounds.bottom() || position.y > bounds.top() {
        velocity.y *= -1.0;
    }

    velocity
}
