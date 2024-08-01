/*!
 * Protocol module defining the behavior for Floodsub and Gossipsub.
 *
 * This module implements the `Protocols` struct, which combines Floodsub
 * and Gossipsub, and provides functions to subscribe the publish messages.
 */

use crate::error::ProtocolError;
use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent},
    gossipsub::{self, MessageAuthenticity},
    identity,
    swarm::NetworkBehaviour,
    PeerId,
};
use log::info;

/// Network behavior combining Floodsub and Gossipsub.
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
    pub fn new(local_peer_id: PeerId, local_key: identity::Keypair) -> Result<Self, ProtocolError> {
        let gossipsub = gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub::Config::default(),
        )
        .map_err(|e| ProtocolError::GossipsubCreation(e.to_string()))?;

        Ok(Protocols {
            floodsub: Floodsub::new(local_peer_id),
            gossipsub,
        })
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
    pub fn subscribe(&mut self, topic: &str) -> Result<(), ProtocolError> {
        let floodsub_topic = floodsub::Topic::new(topic);
        if !self.floodsub.subscribe(floodsub_topic.clone()) {
            return Err(ProtocolError::Subscription(format!(
                "Failed to subscribe to Floodsub topic: {topic}"
            )));
        }

        let gossipsub_topic = gossipsub::IdentTopic::new(topic);
        self.gossipsub.subscribe(&gossipsub_topic).map_err(|_| {
            ProtocolError::Subscription(format!("Failed to subscribe to Gossipsub topic: {topic}"))
        })?;

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
    pub fn publish(&mut self, topic: &str, data: &[u8]) -> Result<(), ProtocolError> {
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
    use libp2p::{gossipsub, identity, PeerId};

    use crate::{error::ProtocolError, protocol::Protocols};

    fn create_test_protocols() -> Protocols {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        Protocols::new(peer_id, keypair).expect("Failed to create Protocols")
    }

    #[test]
    fn test_procotols_new() {
        let protocols = create_test_protocols();
        assert!(protocols.gossipsub.topics().next().is_none());
    }

    #[test]
    fn test_subscribe() {
        let mut protocols = create_test_protocols();
        let topic = "test-topic";

        assert!(protocols.subscribe(topic).is_ok());

        let gossipsub_topic = gossipsub::IdentTopic::new(topic);
        assert!(protocols
            .gossipsub
            .topics()
            .any(|t| t == &gossipsub_topic.hash()));
    }

    #[test]
    fn test_publish() {
        let mut protocols = create_test_protocols();
        let topic = "test-topic";
        let data = b"test-message";

        protocols.subscribe(topic).expect("Failed to subscribe");

        match protocols.publish(topic, data) {
            Ok(_) => println!("Message published successfully"),
            Err(e) => match e {
                ProtocolError::Publish(publish_error) => {
                    assert!(matches!(
                        publish_error,
                        gossipsub::PublishError::InsufficientPeers
                    ));
                }
                _ => panic!("Unexpected error: {:?}", e),
            },
        }
    }
}
