// src/osc/osc_control.rs
//
// OSC commands, sender, and receiver for System3

use nannou_osc as osc;
use std::error::Error;

use crate::settings::OscSendConfig;

/// Helper function to round a float to 3 decimal places
fn round3(f: f32) -> f32 {
    (f * 1000.0).round() / 1000.0
}

/// Wrapper for nannou_osc::Sender
pub struct OscSender {
    sender: osc::Sender,
    target_addr: String,
    target_port: u16,
}

impl OscSender {
    pub fn new(config: &OscSendConfig) -> Result<Self, Box<dyn Error>> {
        let target_addr = config.target_addr.to_owned();
        let target_port = config.target_port;
        let sender = osc::sender()?;
        println!("OSC Sender sending to {}:{}", target_addr, target_port);

        Ok(Self {
            sender,
            target_addr,
            target_port,
        })
    }

    /// Returns the current target address and port
    pub fn get_config(&self) -> OscSendConfig {
        OscSendConfig {
            target_addr: self.target_addr.clone(),
            target_port: self.target_port,
        }
    }

    /// Sends OSC message to `/markov/ch[channel]/note` where
    /// - channel: Int [0-255]
    /// - midi_note: Int [0-127]
    /// - velocity: Int [0-127]
    /// - duration: Float (representing duration in beats)
    pub fn send_note(&self, channel: i32, midi_note: i32, velocity: i32, duration: f32) {
        let addr = format!("/markov/ch{}/note", channel);
        let args = vec![
            osc::Type::Int(midi_note),
            osc::Type::Int(velocity),
            osc::Type::Float(duration),
        ];
        self.sender
            .send((addr, args), (self.target_addr.as_str(), self.target_port))
            .ok();
    }

    /***************** Send functions for testing *********************************** */
}

pub struct OscController {
    receiver: osc::Receiver,
}

impl OscController {
    pub fn new(port: u16) -> Result<Self, Box<dyn Error>> {
        let receiver = osc::receiver(port)?;

        Ok(Self { receiver })
    }

    pub fn process_messages(&mut self) -> Vec<String> {
        let mut commands = Vec::new();
        for (packet, _addr) in self.receiver.try_iter() {
            for message in packet.into_msgs() {
                match message.addr.as_str() {
                    "/markov/example1" => commands.push("example1".to_string()),
                    "/markov/example2" => commands.push("example2".to_string()),

                    _ => {}
                }
            }
        }
        commands
    }
}
