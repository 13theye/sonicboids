//! Position history of an Agent

use std::collections::VecDeque;

use nannou::prelude::*;

#[derive(Debug, Clone)]
pub struct History {
    capacity: usize,
    // (position, velocity)
    pub inner: VecDeque<(Vec2, Vec2)>,
}

impl History {
    /// Creates a new history of size `capacity`
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            inner: vec![(Vec2::ZERO, Vec2::ZERO); capacity].into(),
        }
    }

    /// Pushes a new position to the history
    pub fn push(&mut self, position: Vec2, velocity: Vec2) {
        self.inner.push_back((position, velocity));
        if self.inner.len() > self.capacity {
            self.inner.pop_front();
        }
    }
}
