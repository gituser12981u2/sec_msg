use libp2p::gossipsub;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    #[error("Event handling error: {0}")]
    Event(#[from] EventError),

    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Failed to subscribe to topic: {0}")]
    Subscription(String),

    #[error("Failed to publish message: {0}")]
    Publish(#[from] gossipsub::PublishError),

    #[error("Failed to create Gossipsub behavior: {0}")]
    GossipsubCreation(String),
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to handle Floodsub event: {0}")]
    FloodsubEvent(String),

    #[error("Failed to handle Gossipsub event: {0}")]
    GossipsubEvent(String),

    #[error("Unhandled swarm event: {0}")]
    UnhandledSwarm(String),
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Failed to establish connection: {0}")]
    Connection(String),

    #[error("Failed to close connection: {0}")]
    ConnectionClose(String),

    #[error("Incoming connection error: {0}")]
    IncomingConnection(String),

    #[error("Failed to create protocol: {0}")]
    ProtocolCreation(String),

    #[error("Failed to subscribe to topic: {0}")]
    TopicSubscription(String),

    #[error("Failed to build swarm: {0}")]
    SwarmBuilder(String),

    #[error("Failed to parse address: {0}")]
    AddressParse(String),

    #[error("Failed to listen on address: {0}")]
    Listen(String),
}
