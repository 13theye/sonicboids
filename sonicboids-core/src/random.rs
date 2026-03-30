//! Random-related functionality

mod types;
pub use types::RandomRangeSize;

use rand::prelude::*;
use rand_distr::{Beta, Distribution};
