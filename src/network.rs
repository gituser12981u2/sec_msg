/*!
 * Network module for creating and managing the libp2p swarm.
 *
 * This module provides functions to create a libp2p swarm and handle
 * listening on specified addresses.
 */

use crate::protocol::Protocols;
use libp2p::{
    core::upgrade, identity, plaintext::PlainText2Config, swarm::SwarmBuilder, tcp::TcpConfig,
    yamux::YamuxConfig, Multiaddr, PeerId, Swarm, Transport,
};

/// Creates a libp2p swarm with the specified keypair, peer ID, and topic.
///
/// # Arguments
///
/// * `local_key` - The local identity keypair.
/// * `local_peer_id` - The local peer ID.
/// * `topic` - The topic to subscribe to.
///
/// # Returns
///
/// A `Result` containing the created `Swarm` or an error.
pub fn create_swarm(
    local_key: identity::Keypair,
    local_peer_id: PeerId,
    topic: &str,
) -> Result<Swarm<Protocols>, Box<dyn std::error::Error>> {
    let transport = TcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(PlainText2Config {
            local_public_key: local_key.public(),
        })
        .multiplex(YamuxConfig::default())
        .boxed();

    let mut behaviour = Protocols::new(local_peer_id.clone(), local_key);

    behaviour.subscribe(topic)?;

    let swarm = SwarmBuilder::new(transport, behaviour, local_peer_id.clone())
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    Ok(swarm)
}

/// Starts listening on the specified swarm.
///
/// # Arguments
///
/// * `swarm` -  The libp2p swarm.
///
/// # Returns
///
/// A `Result` indicating success or failure.
pub fn listen_on(swarm: &mut Swarm<Protocols>) -> Result<(), Box<dyn std::error::Error>> {
    let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(addr)?;
    Ok(())
}
