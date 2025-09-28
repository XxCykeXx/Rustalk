pub mod cli;
pub mod config;
pub mod crypto;
pub mod identity;
pub mod message;
pub mod network;
pub mod peer;
pub mod session;

pub use cli::{CliOperations, PathManager, UserManager};
pub use config::{Config, config_exists, get_config_file, load_config, save_config};
pub use crypto::{CryptoEngine, KeyPair};
pub use identity::{Identity, UserCredentials};
pub use message::{Message, MessageType};
pub use network::{NetworkManager, PeerConnection};
pub use peer::{Peer, PeerPingStatus, PeerStatus};
pub use session::{ChatSession, SessionManager};

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main Reach P2P Engine
pub struct ReachEngine {
    pub identity: Identity,
    pub network: Arc<RwLock<NetworkManager>>,
    pub peers: Arc<RwLock<HashMap<String, Peer>>>,
    pub config: Config,
}

impl ReachEngine {
    pub async fn new(credentials: UserCredentials) -> Result<Self> {
        let identity = Identity::new(credentials)?;
        let config = Config::new(identity.clone());
        let network = NetworkManager::new(identity.clone()).await?;

        Ok(ReachEngine {
            identity,
            network: Arc::new(RwLock::new(network)),
            peers: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    pub async fn get_config(&self) -> Result<Config> {
        Ok(self.config.clone())
    }

    pub async fn start_server(&mut self, port: u16) -> Result<()> {
        let network = self.network.write().await;
        network.start_listening(port).await
    }

    pub async fn connect_to_peer(&self, address: &str) -> Result<Peer> {
        let network = self.network.write().await;
        let peer = network.connect_to_peer(address).await?;

        // Add to our peer list
        let mut peers = self.peers.write().await;
        peers.insert(peer.id.to_string(), peer.clone());

        Ok(peer)
    }

    pub async fn send_message(&self, peer_id: &str, content: &str) -> Result<String> {
        let network = self.network.read().await;
        network.send_message(peer_id, content).await
    }

    pub async fn ping_peer(&self, peer_id: &str) -> PeerPingStatus {
        let network = self.network.read().await;
        network.ping_peer(peer_id).await
    }

    pub async fn set_nickname(&self, nickname: String) -> Result<()> {
        let mut network = self.network.write().await;
        network.set_nickname(nickname).await
    }

    pub async fn get_connected_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    pub async fn shutdown(&self) {
        let mut network = self.network.write().await;
        network.shutdown().await;
    }
}
