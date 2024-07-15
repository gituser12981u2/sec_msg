/*!
 * User interface module for handling user input.
 *
 * This module provides functions to process and handle user input commands.
 */

use crate::protocol::Protocols;
use libp2p::Swarm;
use log::{error, info};

/// Handles user input commands and executes the corresponding actions.
///
/// # Arguments
///
/// * `line` - The user input line.
/// * `swarm` - The libp2p swarm.
/// * `topic` - The topic to publish messages to.
pub async fn handle_user_input(line: String, swarm: &mut Swarm<Protocols>, topic: &str) {
    if line.starts_with("/connect") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            match parts[1].parse::<libp2p::Multiaddr>() {
                Ok(addr) => {
                    info!("Dialing {:?}", addr);
                    swarm.dial(addr).unwrap_or_else(|e| {
                        error!("Failed to dial address: {:?} on topic {:?}", e, topic)
                    });
                }
                Err(_) => error!("Invalid multiaddress"),
            }
        } else {
            error!("Usage: /connect <multiaddress>");
        }
    } else {
        info!("Publishing message: {:?}", line);
        if let Err(e) = swarm.behaviour_mut().publish(topic, line.as_bytes()) {
            error!("Failed to publish message: {:?} on {:?}", e, topic);
        }
    }
}
