//! Supporting types for random-related functionality

/// Shorthand for random range sizes. Can be converted to parameters for:
/// - Uniform Distribution, via [`RandomRangeSize::to_range_inclusive`]
/// - Beta Distribution, via [`RandomRangeSize::to_beta_distribution_params`]
#[derive(Debug, Default, Clone, Copy)]
pub enum RandomRangeSize {
    XS = 0,
    S = 1,
    #[default]
    M = 2,
    L = 3,
    XL = 4,
    R = 5,
}

impl RandomRangeSize {
    pub fn to_range_inclusive(&self) -> std::ops::RangeInclusive<f32> {
        match self {
            RandomRangeSize::XS => 0.0..=0.25,
            RandomRangeSize::S => 0.2..=0.45,
            RandomRangeSize::M => 0.4..=0.66,
            RandomRangeSize::L => 0.5..=0.88,
            RandomRangeSize::XL => 0.5..=1.0,
            RandomRangeSize::R => 0.0..=1.0,
        }
    }

    /// converts the named range to parameters for Beta Distribution.
    /// returns (alpha, beta)
    /// All ranges produce values in [0.0, 1.0] with different skew:
    /// - XS, S: skewed toward 0.0
    /// - M: normal distribution (no skew)
    /// - L, XL: skewed toward 1.0
    /// - R: uniform random
    pub fn to_beta_distribution_params(&self) -> (f32, f32) {
        match self {
            RandomRangeSize::XS => (1.0, 3.0),
            RandomRangeSize::S => (2.0, 3.5),
            RandomRangeSize::M => (3.0, 3.0),
            RandomRangeSize::L => (3.5, 2.0),
            RandomRangeSize::XL => (3.0, 1.0),
            RandomRangeSize::R => (1.0, 1.0),
        }
    }
}
