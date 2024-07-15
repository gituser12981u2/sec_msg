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
pub async fn handle_event<THandlerErr>(
    event: SwarmEvent<ProtocolEvent, THandlerErr>,
    swarm: &mut Swarm<Protocols>,
) {
    match event {
        SwarmEvent::Behaviour(event) => match event {
            ProtocolEvent::Floodsub(floodsub_event) => handle_floodsub_event(floodsub_event).await,
            ProtocolEvent::Gossipsub(gossipsub_event) => {
                handle_gossipsub_event(gossipsub_event).await
            }
        },
        SwarmEvent::NewListenAddr {
            listener_id,
            address,
        } => {
            info!("Listening on {:?} with address {:?}", listener_id, address);
        }
        SwarmEvent::ConnectionEstablished {
            peer_id,
            endpoint,
            num_established,
            concurrent_dial_errors,
        } => {
            info!("Connected to {:?}", peer_id);
            swarm
                .behaviour_mut()
                .floodsub
                .add_node_to_partial_view(peer_id);
        }
        SwarmEvent::ConnectionClosed {
            peer_id,
            endpoint,
            num_established,
            cause,
        } => {
            info!("Disconnected from {:?}", peer_id);
        }
        SwarmEvent::IncomingConnection {
            local_addr,
            send_back_addr,
        } => {
            info!("Incoming connection from {:?}", local_addr);
        }
        SwarmEvent::IncomingConnectionError {
            local_addr,
            send_back_addr,
            error,
        } => {
            error!(
                "Incoming connection error: {:?} from {:?}",
                error, local_addr
            )
        }
        SwarmEvent::Dialing(peer_id) => {
            info!("Dialing {:?}", peer_id);
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
        info!("Received: '{:?}' from {:?}", msg, message.source);
    }
}

/// Handles Gossipsub events.
///
/// # Arguments
///
/// * `event` -  The Gossipsub event.
async fn handle_gossipsub_event(event: libp2p::gossipsub::GossipsubEvent) {
    if let libp2p::gossipsub::GossipsubEvent::Message {
        propagation_source: _,
        message_id: _,
        message,
    } = event
    {
        let msg = String::from_utf8_lossy(&message.data);
        info!("Received: '{:?}' from {:?}", msg, message.source);
    }
}
