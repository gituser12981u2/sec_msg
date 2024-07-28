/*!
 * Network module for creating and managing the libp2p swarm.
 *
 * This module provides functions to create a libp2p swarm and handle
 * listening on specified addresses.
 */

use std::{error::Error, time::Duration};

use libp2p::{identity, tcp, tls, yamux, Multiaddr, PeerId, Swarm, SwarmBuilder};

use crate::protocol::Protocols;

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
pub async fn create_swarm(
    local_key: identity::Keypair,
    local_peer_id: PeerId,
    topic: &str,
) -> Result<Swarm<Protocols>, Box<dyn Error>> {
    let mut behaviour = Protocols::new(local_peer_id, local_key);

    behaviour.subscribe(topic)?;

    let swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            tls::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| behaviour)?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(30))) // Allows us to observe pings for 30 seconds.
        .build();

    Ok(swarm)
}

/// Starts listening on the specified swarm.
///
/// # Arguments
///
/// * `swarm` - The libp2p swarm.
///
/// # Returns
///
/// A `Result` indicating success or failure.
pub fn listen_on(swarm: &mut Swarm<Protocols>) -> Result<(), Box<dyn Error>> {
    let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(addr)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use libp2p::{identity, PeerId};

    use super::{create_swarm, listen_on};

    #[tokio::test]
    async fn test_create_swarm() {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        let topic = "test-topic";
        let swarm = create_swarm(keypair, peer_id, topic).await;
        assert!(swarm.is_ok());
    }

    #[tokio::test]
    async fn test_listen_on() {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        let topic = "test-topic";
        let mut swarm = create_swarm(keypair, peer_id, topic).await.unwrap();
        let result = listen_on(&mut swarm);
        assert!(result.is_ok());
    }
}
