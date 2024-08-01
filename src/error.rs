/*! This module defines the error types used throughout the application.
 *
 * It provides a hierarchical structure of errors, allowing for more precise error handling and reporting.
*/

use libp2p::gossipsub;
use thiserror::Error;

/// Represents the top-level error type for the application.
#[derive(Error, Debug)]
pub enum AppError {
    /// Errors related to protocol operations.
    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    /// Errors related to network operations.
    #[error("Event handling error: {0}")]
    Event(#[from] EventError),

    /// Errors related to network operations.
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    /// Standard I/O errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Represents errors that can occur during protocol operations.
#[derive(Error, Debug)]
pub enum ProtocolError {
    /// Error occurring when failing to subscribe to a topic.
    #[error("Failed to subscribe to topic: {0}")]
    Subscription(String),

    /// Error occurring when failing to publish a message.
    #[error("Failed to publish message: {0}")]
    Publish(#[from] gossipsub::PublishError),

    /// Error occurring when failing to create a Gossipsub behaviour.
    #[error("Failed to create Gossipsub behavior: {0}")]
    GossipsubCreation(String),
}

/// Represents errors that can occur during event handling
#[derive(Error, Debug)]
pub enum EventError {
    /// Error occurring when handling a Floodsub handling.
    #[error("Failed to handle Floodsub event: {0}")]
    FloodsubEvent(String),

    /// Error occurring when handling a Gossipsub event.
    #[error("Failed to handle Gossipsub event: {0}")]
    GossipsubEvent(String),

    /// Error occurring when encountering an unhandled swarm event.
    #[error("Unhandled swarm event: {0}")]
    UnhandledSwarm(String),
}

/// Represents errors that can occur during network operations
#[derive(Error, Debug)]
pub enum NetworkError {
    /// Error occurring when failing to establish a connection.
    #[error("Failed to establish connection: {0}")]
    Connection(String),

    /// Error occurring when failing to close a connection.
    #[error("Failed to close connection: {0}")]
    ConnectionClose(String),

    /// Error occurring with an incoming connection.
    #[error("Incoming connection error: {0}")]
    IncomingConnection(String),

    /// Error occurring when creating a protocol.
    #[error("Failed to create protocol: {0}")]
    ProtocolCreation(String),

    /// Error occurring when subscribing to a topic.
    #[error("Failed to subscribe to topic: {0}")]
    TopicSubscription(String),

    /// Error occurring when building a swarm.
    #[error("Failed to build swarm: {0}")]
    SwarmBuilder(String),

    /// Error occurring when parsing an address.
    #[error("Failed to parse address: {0}")]
    AddressParse(String),

    /// Error occurring when listening on an address.
    #[error("Failed to listen on address: {0}")]
    Listen(String),
}
