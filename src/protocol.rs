/*!
 * Protocol module defining the behavior for Floodsub and Gossipsub.
 *
 * This module implements the `Protocols` struct, which combines Floodsub
 * and Gossipsub, and provides functions to subscribe the publish messages.
 */

use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    gossipsub::{self, Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity},
    identity, NetworkBehaviour, PeerId,
};
use log::{error, info};
use std::error::Error;

/// Network behavior combining Floodsub and Gossipsub protocols.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "ProtocolEvent")]
pub struct Protocols {
    pub floodsub: Floodsub,
    pub gossipsub: Gossipsub,
}

impl Protocols {
    /// Creates a new `Protocols` instance.
    ///
    /// # Arguments
    ///
    /// * `local_peer_id` - The local peer ID.
    /// * `local_key` - The local identity keypair.
    ///
    /// # Returns
    ///
    /// A new `Protocols` instance.
    pub fn new(local_peer_id: PeerId, local_key: identity::Keypair) -> Self {
        let gossipsub_config = GossipsubConfig::default();
        let gossipsub =
            Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config).unwrap();

        Protocols {
            floodsub: Floodsub::new(local_peer_id.clone()),
            gossipsub,
        }
    }

    /// Subscribes to the specified topic.
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic to subscribe to.
    ///
    /// # Returns
    ///
    /// A `result` indicating success or failure.
    pub fn subscribe(&mut self, topic: &str) -> Result<(), Box<dyn Error>> {
        let floodsub_topic = floodsub::Topic::new(topic);
        if !self.floodsub.subscribe(floodsub_topic.clone()) {
            error!("Failed to subscribe to floodsub topic: {:?}", topic);
            return Err("Failed to subscribe to floodsub topic".into());
        }

        let gossipsub_topic = gossipsub::IdentTopic::new(topic);
        if self.gossipsub.subscribe(&gossipsub_topic).is_err() {
            error!("Failed to subscribe to gossipsub topic: {:?}", topic);
            return Err("Failed to subscribe to gossipsub topic".into());
        }
        info!("Subscribed to topic: {:?}", topic);
        Ok(())
    }

    /// Publishes a message to the specified topic.
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic to publish to.
    /// * `data` - The message data.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn publish(&mut self, topic: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let floodsub_topic = floodsub::Topic::new(topic);
        self.floodsub.publish(floodsub_topic, data);

        let gossipsub_topic = gossipsub::IdentTopic::new(topic);
        if self.gossipsub.publish(gossipsub_topic, data).is_err() {
            error!("Failed to publish to gossipsub topic: {:?}", topic);
            return Err("Failed to publish to gossipsub topic".into());
        }

        let message = String::from_utf8_lossy(data);
        info!("Published message {:?} to topic: {:?}", message, topic);
        Ok(())
    }
}

/// Enumeration of protocol events.
#[derive(Debug)]
pub enum ProtocolEvent {
    Floodsub(FloodsubEvent),
    Gossipsub(GossipsubEvent),
}

impl From<FloodsubEvent> for ProtocolEvent {
    fn from(event: FloodsubEvent) -> Self {
        ProtocolEvent::Floodsub(event)
    }
}

impl From<GossipsubEvent> for ProtocolEvent {
    fn from(event: GossipsubEvent) -> Self {
        ProtocolEvent::Gossipsub(event)
    }
}
