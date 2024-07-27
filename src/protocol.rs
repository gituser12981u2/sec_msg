/*!
 * Protocol module defining the behavior for Floodsub and Gossipsub.
 *
 * This module implements the `Protocols` struct, which combines Floodsub
 * and Gossipsub, and provides functions to subscribe the publish messages.
 */

use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    gossipsub::{self, MessageAuthenticity},
    identity,
    swarm::NetworkBehaviour,
    PeerId,
};
use log::{error, info};
use std::error::Error;

/// Network behavior combining Floodsub, Gossipsub, and mDNS protocols.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "ProtocolEvent")]
pub struct Protocols {
    pub floodsub: Floodsub,
    pub gossipsub: gossipsub::Behaviour,
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
        // let gossipsub_config = gossipsub::ConfigBuilder::default()
        //     .heartbeat_interval(Duration::from_secs(10))
        //     .validation_mode(gossipsub::ValidationMode::Strict)
        //     .build()
        //     .expect("Valid gossipsub config");

        Protocols {
            floodsub: Floodsub::new(local_peer_id),
            gossipsub: gossipsub::Behaviour::new(
                MessageAuthenticity::Signed(local_key.clone()),
                gossipsub::Config::default(),
            )
            .expect("Valid gossipsub instance"),
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
        self.floodsub.publish(floodsub_topic, data.to_vec());

        let gossipsub_topic = gossipsub::IdentTopic::new(topic);
        self.gossipsub.publish(gossipsub_topic, data.to_vec())?;

        let message = String::from_utf8_lossy(data);
        info!("Published message {:?} to topic: {:?}", message, topic);
        Ok(())
    }
}

/// Enumeration of protocol events.
#[derive(Debug)]
pub enum ProtocolEvent {
    Floodsub(FloodsubEvent),
    Gossipsub(Box<gossipsub::Event>),
}

impl From<FloodsubEvent> for ProtocolEvent {
    fn from(event: FloodsubEvent) -> Self {
        ProtocolEvent::Floodsub(event)
    }
}

impl From<gossipsub::Event> for ProtocolEvent {
    fn from(event: gossipsub::Event) -> Self {
        ProtocolEvent::Gossipsub(Box::new(event))
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use libp2p::{
        floodsub,
        gossipsub::{self, PublishError},
        identity, PeerId,
    };

    use crate::protocol::Protocols;

    #[test]
    fn test_procotols_new() {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        let protocols = Protocols::new(peer_id, keypair);
        // Floodsub does not have a direct method to get topics
        assert!(protocols.gossipsub.topics().next().is_none());
    }

    #[test]
    fn test_subscribe_publish() {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        let mut protocols = Protocols::new(peer_id, keypair);

        let topic = "test-topic";
        protocols.subscribe(topic).unwrap();

        // Check Floodsub subscription
        let _floodsub_topic = floodsub::Topic::new(topic);
        // Note: Floodsub does not have a public API to check subscribed topics,
        // therefore, it assumed that if subscribe() returns no error then it works.

        // Check Gossipsub subscription
        let gossipsub_topic = gossipsub::IdentTopic::new(topic);
        assert!(protocols
            .gossipsub
            .topics()
            .any(|t| t == &gossipsub_topic.hash()));

        thread::sleep(Duration::from_millis(100));

        let data = b"test-message";
        match protocols.publish(topic, data) {
            Ok(_) => println!("Message published successfully"),
            Err(e) => {
                if let Some(publish_error) = e.downcast_ref::<PublishError>() {
                    if matches!(publish_error, PublishError::InsufficientPeers) {
                        println!("Expected error: InsufficientPeers");
                    } else {
                        panic!("Unexpected gossipsub publish error: {:?}", publish_error);
                    }
                } else {
                    panic!("Unexpected error: {:?}", e);
                }
            }
        }
    }
}
