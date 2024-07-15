/*!
 * Utility functions for the messaging application.
 *
 * This module provides utility functions for generating keypairs
 * and peer IDs.
 */

use libp2p::{identity, PeerId};
use log::info;

/// Generates a new Ed25519 keypair and corresponding peer ID.
///
/// # Returns
///
/// A tuple containing the generated keypair and peer ID.
pub fn generate_keypair() -> (identity::Keypair, PeerId) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("Generated local key pair with peer id: {:?}", local_peer_id);
    (local_key, local_peer_id)
}
