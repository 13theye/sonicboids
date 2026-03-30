//! Settings types
//!

use serde::Deserialize;

/// Configuration for the OSC module
#[derive(Deserialize)]
pub struct OscSendConfig {
    pub target_addr: String,
    pub target_port: u16,
}

/// Initial tempo settings for the Sequencer & Clock
#[derive(Deserialize)]
pub struct TempoConfig {
    pub bpm: f32,
}
