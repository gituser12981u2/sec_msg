/*!
 * Event handling module for the messaging application.
 *
 * This module defines functions to handle various swarm events and protocol
 * events for Floodsub and Gossipsub.
 */

use crate::protocol::{ProtocolEvent, Protocols};
use libp2p::swarm::{Swarm, SwarmEvent};
use log::{error, info};

/// Handles swarm events and dispatches them to the appropriate handlers.
///
/// # Arguments
///
/// * `event` - The swarm event.
/// * `swarm` - The libp2p swarm.
pub async fn handle_event(event: SwarmEvent<ProtocolEvent>, swarm: &mut Swarm<Protocols>) {
    match event {
        SwarmEvent::Behaviour(event) => match event {
            ProtocolEvent::Floodsub(floodsub_event) => handle_floodsub_event(floodsub_event).await,
            ProtocolEvent::Gossipsub(gossipsub_event) => {
                handle_gossipsub_event(*gossipsub_event).await
            }
        },
        SwarmEvent::NewListenAddr {
            listener_id,
            address,
        } => {
            info!("Listening {:?} on address {:?}", listener_id, address);
        }
        SwarmEvent::ConnectionEstablished {
            peer_id,
            connection_id,
            endpoint,
            num_established,
            concurrent_dial_errors,
            established_in,
        } => {
            info!(
                "Connected to {:?}, connection_id={:?} endpoint={:?}, num_established={}, concurrent_dial_errors={:?}, established_in={:?}",
                peer_id, connection_id, endpoint, num_established, concurrent_dial_errors, established_in
            );
            swarm
                .behaviour_mut()
                .floodsub
                .add_node_to_partial_view(peer_id);
        }
        SwarmEvent::ConnectionClosed {
            peer_id,
            endpoint,
            num_established,
            cause: _,
            connection_id,
        } => {
            info!(
                "Connection closed for {:?}, endpoint={:?}, num_established={}, connection_id={:?}",
                peer_id, endpoint, num_established, connection_id
            );
        }
        SwarmEvent::IncomingConnection {
            local_addr,
            send_back_addr,
            connection_id,
        } => {
            info!(
                "Incoming connection from {:?}, send_back_addr={:?}, connection_id={:?}",
                local_addr, send_back_addr, connection_id
            );
        }
        SwarmEvent::IncomingConnectionError {
            local_addr,
            send_back_addr,
            error,
            connection_id,
        } => {
            error!(
                "Incoming connection error: {:?} from {:?}, send_back_addr={:?}, connection_id={:?}",
                error, local_addr, send_back_addr, connection_id
            )
        }
        SwarmEvent::Dialing {
            peer_id,
            connection_id,
        } => {
            info!("Dialing {:?}, connection_id={:?}", peer_id, connection_id);
        }
        _ => {
            error!("Unhandled event. Please post github issue.");
        }
    }
}

/// Handles Floodsub events.
///
/// # Arguments
///
/// * `event` - The Floodsub event.
async fn handle_floodsub_event(event: libp2p::floodsub::FloodsubEvent) {
    if let libp2p::floodsub::FloodsubEvent::Message(message) = event {
        let msg = String::from_utf8_lossy(&message.data);
        info!(
            "Floodsub message received: '{:?}' from {:?}",
            msg, message.source
        );
    }
}

/// Handles Gossipsub events.
///
/// # Arguments
///
/// * `event` -  The Gossipsub event.
async fn handle_gossipsub_event(event: libp2p::gossipsub::Event) {
    if let libp2p::gossipsub::Event::Message {
        propagation_source,
        message_id,
        message,
    } = event
    {
        let msg = String::from_utf8_lossy(&message.data);
        info!(
            "Gossipsub message received: '{:?}' from {:?} wit id {:?}, propagation source: {:?}",
            msg, message.source, message_id, propagation_source
        );
    }
}
