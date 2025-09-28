use anyhow::{Result, anyhow};
use log::{debug, error, info};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::crypto::CryptoEngine;
use crate::identity::Identity;
use crate::message::{Message, MessageType};
use crate::peer::Peer;
// Removed x25519_dalek imports - using simplified crypto

pub struct PeerConnection {
    pub peer: Peer,
    pub stream: Arc<RwLock<TcpStream>>,
    pub shared_secret: Option<[u8; 32]>,
}

impl PeerConnection {
    pub fn new(peer: Peer, stream: TcpStream) -> Self {
        PeerConnection {
            peer,
            stream: Arc::new(RwLock::new(stream)),
            shared_secret: None,
        }
    }

    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        if let Some(secret) = &self.shared_secret {
            let encrypted = CryptoEngine::encrypt_message(message, secret)?;
            let mut stream = self.stream.write().await;

            let data = format!("{}\n", encrypted);
            stream.write_all(data.as_bytes()).await?;
            stream.flush().await?;

            debug!("Sent encrypted message to peer {}", self.peer.id);
            Ok(())
        } else {
            Err(anyhow!("No shared secret established"))
        }
    }

    pub async fn receive_message(&mut self) -> Result<String> {
        if let Some(secret) = &self.shared_secret {
            let mut stream = self.stream.write().await;
            let mut buffer = vec![0; 4096];

            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                return Err(anyhow!("Connection closed"));
            }

            let encrypted_data = String::from_utf8_lossy(&buffer[..n]);
            let encrypted_data = encrypted_data.trim();

            let decrypted = CryptoEngine::decrypt_message(encrypted_data, secret)?;
            debug!("Received and decrypted message from peer {}", self.peer.id);

            Ok(decrypted)
        } else {
            Err(anyhow!("No shared secret established"))
        }
    }

    pub fn establish_shared_secret(&mut self, our_private: &[u8; 32], their_public: &[u8; 32]) {
        self.shared_secret = Some(CryptoEngine::generate_shared_secret(
            our_private,
            their_public,
        ));
        info!("Shared secret established with peer {}", self.peer.id);
    }
}

pub struct NetworkManager {
    identity: Identity,
    connections: Arc<RwLock<HashMap<Uuid, PeerConnection>>>,
    message_sender: mpsc::UnboundedSender<Message>,
    message_receiver: Arc<RwLock<mpsc::UnboundedReceiver<Message>>>,
}

impl NetworkManager {
    pub async fn new(identity: Identity) -> Result<Self> {
        let (message_sender, message_receiver) = mpsc::unbounded_channel();

        Ok(NetworkManager {
            identity,
            connections: Arc::new(RwLock::new(HashMap::new())),
            message_sender,
            message_receiver: Arc::new(RwLock::new(message_receiver)),
        })
    }

    pub async fn start_listening(&self, port: u16) -> Result<()> {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        info!("Rustalk listening on {}", addr);

        let connections = self.connections.clone();
        let identity = self.identity.clone();
        let message_sender = self.message_sender.clone();

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        info!("New connection from {}", addr);

                        let connections = connections.clone();
                        let identity = identity.clone();
                        let message_sender = message_sender.clone();

                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_incoming_connection(
                                stream,
                                addr,
                                connections,
                                identity,
                                message_sender,
                            )
                            .await
                            {
                                error!("Error handling connection from {}: {}", addr, e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    async fn handle_incoming_connection(
        mut stream: TcpStream,
        addr: SocketAddr,
        connections: Arc<RwLock<HashMap<Uuid, PeerConnection>>>,
        identity: Identity,
        message_sender: mpsc::UnboundedSender<Message>,
    ) -> Result<()> {
        // Perform handshake
        let handshake_msg = Message::handshake_message(
            identity.user_id,
            identity.keypair.public_key.clone(),
            identity.get_display_name(),
        );

        let handshake_data = serde_json::to_string(&handshake_msg)?;
        stream
            .write_all(format!("{}\n", handshake_data).as_bytes())
            .await?;
        stream.flush().await?;

        // Read peer's handshake
        let mut buffer = vec![0; 4096];
        let n = stream.read(&mut buffer).await?;

        if n == 0 {
            return Err(anyhow!("Connection closed during handshake"));
        }

        let peer_handshake: Message = serde_json::from_slice(&buffer[..n])?;

        if !matches!(peer_handshake.message_type, MessageType::Handshake) {
            return Err(anyhow!("Expected handshake message"));
        }

        // Create peer
        // Save values before moving
        let sender_name = peer_handshake.sender_name.clone();
        let sender_id = peer_handshake.sender_id;

        // Establish shared secret first
        let our_private = identity.get_private_key_bytes()?;
        let their_public_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            &peer_handshake.content,
        )?;

        let peer = Peer::new(
            sender_id,
            "unknown@peer.local".to_string(), // We'll need to exchange this info
            peer_handshake.sender_name,
            addr,
            peer_handshake.content, // This contains the public key
        );

        let mut connection = PeerConnection::new(peer, stream);

        if their_public_bytes.len() != 32 {
            return Err(anyhow!("Invalid public key length"));
        }

        let mut their_public = [0u8; 32];
        their_public.copy_from_slice(&their_public_bytes);

        connection.establish_shared_secret(&our_private, &their_public);
        connection.peer.set_authenticated();

        let peer_id = connection.peer.id;

        // Store connection
        {
            let mut conns = connections.write().await;
            conns.insert(peer_id, connection);
        }

        // Send connection established message
        let _ = message_sender.send(Message::system_message(format!(
            "Connected to {}",
            sender_name
        )));

        info!(
            "Successfully connected to peer {} ({})",
            peer_id, sender_name
        );

        Ok(())
    }

    pub async fn connect_to_peer(&self, address: &str) -> Result<Peer> {
        let stream = TcpStream::connect(address).await?;
        let addr: SocketAddr = address.parse()?;

        info!("Connected to peer at {}", address);

        // This is similar to handle_incoming_connection but for outgoing connections
        // For brevity, I'll implement a simplified version
        let peer = Peer::new(
            Uuid::new_v4(), // Temporary ID until handshake
            "unknown@peer.local".to_string(),
            "Unknown".to_string(),
            addr,
            "".to_string(),
        );

        let connection = PeerConnection::new(peer.clone(), stream);

        // Store connection (simplified - in real implementation, complete handshake first)
        {
            let mut conns = self.connections.write().await;
            conns.insert(connection.peer.id, connection);
        }

        Ok(peer)
    }

    pub async fn send_message(&self, peer_id: &str, content: &str) -> Result<String> {
        let peer_uuid = Uuid::parse_str(peer_id)?;

        let mut connections = self.connections.write().await;
        if let Some(connection) = connections.get_mut(&peer_uuid) {
            let message = Message::text_message(
                self.identity.user_id,
                peer_uuid,
                content.to_string(),
                self.identity.get_display_name(),
            );

            let message_id = message.id.to_string();
            let message_json = serde_json::to_string(&message)?;
            connection.send_message(&message_json).await?;

            // Send to local message handler
            let _ = self.message_sender.send(message);

            Ok(message_id)
        } else {
            Err(anyhow!("Peer not found or not connected"))
        }
    }

    pub async fn get_connected_peers(&self) -> Vec<Peer> {
        let connections = self.connections.read().await;
        connections.values().map(|conn| conn.peer.clone()).collect()
    }

    pub async fn disconnect_peer(&self, peer_id: Uuid) -> Result<()> {
        let mut connections = self.connections.write().await;
        if let Some(mut connection) = connections.remove(&peer_id) {
            connection.peer.set_disconnected();
            info!("Disconnected from peer {}", peer_id);
            Ok(())
        } else {
            Err(anyhow!("Peer not found"))
        }
    }

    pub async fn receive_messages(&self) -> Option<Message> {
        let mut receiver = self.message_receiver.write().await;
        receiver.recv().await
    }

    pub async fn ping_peer(&self, peer_id: &str) -> crate::peer::PeerPingStatus {
        let peer_uuid = match Uuid::parse_str(peer_id) {
            Ok(uuid) => uuid,
            Err(_) => return crate::peer::PeerPingStatus::offline(peer_id.to_string()),
        };

        let connections = self.connections.read().await;
        if let Some(_connection) = connections.get(&peer_uuid) {
            // Try to send a ping message
            let _ping_message = "PING".to_string();
            let start_time = std::time::Instant::now();

            // For now, just return online if connection exists
            // In a real implementation, you'd send an actual ping and wait for response
            crate::peer::PeerPingStatus {
                user_id: peer_id.to_string(),
                is_online: true,
                last_seen: chrono::Utc::now(),
                response_time: Some(start_time.elapsed().as_millis() as u64),
            }
        } else {
            crate::peer::PeerPingStatus::offline(peer_id.to_string())
        }
    }

    pub async fn set_nickname(&mut self, nickname: String) -> Result<()> {
        self.identity.display_name = Some(nickname);
        Ok(())
    }

    pub async fn stop_listening(&self) -> Result<()> {
        info!("Stopping listening for new connections...");
        // Note: In a real implementation, you'd want to store the listener handle
        // and be able to stop it. For now, we'll just shutdown existing connections.
        self.shutdown_connections().await;
        Ok(())
    }

    pub async fn shutdown_connections(&self) {
        info!("Shutting down all connections...");

        let mut connections = self.connections.write().await;
        for (peer_id, mut connection) in connections.drain() {
            connection.peer.set_disconnected();
            info!("Disconnected from peer {}", peer_id);
        }

        info!("All connections shut down");
    }

    pub async fn shutdown(&mut self) {
        info!("Shutting down network manager...");
        self.shutdown_connections().await;
        info!("Network manager shutdown complete");
    }
}
