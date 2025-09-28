use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};

use crate::{Identity, Message, MessageType, NetworkManager, Peer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub active_peers: HashMap<String, Peer>,
    pub message_history: Vec<Message>,
    pub current_port: u16,
}

impl ChatSession {
    pub fn new(id: String, port: u16) -> Self {
        Self {
            id,
            active_peers: HashMap::new(),
            message_history: Vec::new(),
            current_port: port,
        }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.active_peers.insert(peer.id.to_string(), peer);
    }

    pub fn remove_peer(&mut self, peer_id: &str) -> Option<Peer> {
        self.active_peers.remove(peer_id)
    }

    pub fn add_message(&mut self, message: Message) {
        self.message_history.push(message);
    }

    pub fn get_peers(&self) -> Vec<&Peer> {
        self.active_peers.values().collect()
    }

    pub fn get_peer(&self, peer_id: &str) -> Option<&Peer> {
        self.active_peers.get(peer_id)
    }

    pub fn get_recent_messages(&self, limit: usize) -> Vec<&Message> {
        let start = if self.message_history.len() > limit {
            self.message_history.len() - limit
        } else {
            0
        };
        self.message_history[start..].iter().collect()
    }
}

pub struct SessionManager {
    pub identity: Identity,
    pub network: Arc<RwLock<NetworkManager>>,
    pub current_session: Arc<RwLock<Option<ChatSession>>>,
    pub message_sender: Option<mpsc::Sender<Message>>,
    pub message_receiver: Arc<RwLock<Option<mpsc::Receiver<Message>>>>,
}

impl SessionManager {
    pub async fn new(identity: Identity) -> Result<Self> {
        let network = NetworkManager::new(identity.clone()).await?;
        let (tx, rx) = mpsc::channel(100);

        Ok(SessionManager {
            identity,
            network: Arc::new(RwLock::new(network)),
            current_session: Arc::new(RwLock::new(None)),
            message_sender: Some(tx),
            message_receiver: Arc::new(RwLock::new(Some(rx))),
        })
    }

    pub async fn start_session(&self, port: u16) -> Result<String> {
        let session_id = format!("session_{}", chrono::Utc::now().timestamp());
        let session = ChatSession::new(session_id.clone(), port);

        {
            let mut current_session = self.current_session.write().await;
            *current_session = Some(session);
        }

        {
            let network = self.network.write().await;
            network.start_listening(port).await?;
        }

        Ok(session_id)
    }

    pub async fn connect_to_peer(&self, address: &str) -> Result<()> {
        let network = self.network.read().await;
        network.connect_to_peer(address).await?;

        // Add peer to current session
        if let Some(session) = self.current_session.write().await.as_mut() {
            let peer_addr: SocketAddr = address.parse()?;
            let peer = Peer::new(
                uuid::Uuid::new_v4(),
                format!("unknown@{}", address),
                "Unknown".to_string(),
                peer_addr,
                "unknown_key".to_string(),
            );
            session.add_peer(peer);
        }

        Ok(())
    }

    pub async fn send_message(&self, content: String, target_peer: Option<String>) -> Result<()> {
        let recipient_id = if let Some(_peer_name) = target_peer {
            // In a real implementation, you'd look up the peer ID by name
            // For now, just use None for broadcast
            None
        } else {
            None
        };

        let message = Message::new(
            self.identity.user_id,
            recipient_id,
            MessageType::Text,
            content,
            self.identity.get_display_name(),
        );

        // Add to session history
        if let Some(session) = self.current_session.write().await.as_mut() {
            session.add_message(message.clone());
        }

        // Send through message channel
        if let Some(sender) = &self.message_sender {
            sender
                .send(message)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to send message: {}", e))?;
        }

        Ok(())
    }

    pub async fn get_active_peers(&self) -> Vec<Peer> {
        if let Some(session) = self.current_session.read().await.as_ref() {
            session.active_peers.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub async fn get_session_info(&self) -> Option<(String, u16, usize)> {
        if let Some(session) = self.current_session.read().await.as_ref() {
            Some((
                session.id.clone(),
                session.current_port,
                session.active_peers.len(),
            ))
        } else {
            None
        }
    }

    pub async fn end_session(&self) -> Result<()> {
        {
            let mut current_session = self.current_session.write().await;
            *current_session = None;
        }

        let network = self.network.read().await;
        network.stop_listening().await?;

        Ok(())
    }

    pub async fn list_recent_messages(&self, limit: usize) -> Vec<Message> {
        if let Some(session) = self.current_session.read().await.as_ref() {
            let recent: Vec<Message> = session
                .get_recent_messages(limit)
                .into_iter()
                .cloned()
                .collect();
            recent
        } else {
            Vec::new()
        }
    }

    pub async fn get_peer_count(&self) -> usize {
        if let Some(session) = self.current_session.read().await.as_ref() {
            session.active_peers.len()
        } else {
            0
        }
    }
}
