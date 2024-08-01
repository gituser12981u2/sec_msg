/*!
 * Main entry point for the messaging application.
 *
 * This module sets up the configuration, initializes the logger,
 * and starts the main event loop to handle user input and network events.
 */

mod config;
mod error;
mod event;
mod network;
mod protocol;
mod security;
mod ui;
mod utils;

use config::Config;
use futures::StreamExt;
use log::error;
use network::{create_swarm, listen_on};
use tokio::io::AsyncBufReadExt;
use ui::handle_user_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&config.log_level))
        .init();

    let (local_key, local_peer_id) = utils::generate_keypair();

    let topic = "chat";

    let mut swarm = create_swarm(local_key.clone(), local_peer_id, topic).await?;

    listen_on(&mut swarm)?;

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        handle_user_input(line, &mut swarm, topic).await;
                    }
                    Ok(None) => {
                        error!("stdin closed");
                        break;
                    }
                    Err(e) => {
                        error!("Error reading stdin: {:?}", e);
                        break;
                    }
                }
            }
            event = swarm.next() => match event {
                Some(event) => event::handle_event(event, &mut swarm).await?,
                None => error!("Swarm stream closed"),
            }
        }
    }

    Ok(())
}
