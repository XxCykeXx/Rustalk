use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use log::{info, warn};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use reach::{Config, ReachEngine, UserCredentials, PeerPingStatus};

#[derive(Debug, Clone)]
#[allow(dead_code)] // Used by future TUI implementation
pub enum UIEvent {
    Quit,
    SendMessage(String),
    SwitchTab(usize),
    StatusCheck(String),
    Input(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub encrypted: bool,
    pub message_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedPeer {
    pub id: String,
    pub email: String,
    pub nickname: Option<String>,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
    pub response_time: Option<u64>,
}

pub struct ChatApp {
    _config: Config,
    engine: Arc<RwLock<ReachEngine>>,
    _message_history: Arc<RwLock<Vec<ChatMessage>>>,
    peers: Arc<RwLock<HashMap<String, ConnectedPeer>>>,
    _shutdown_tx: Option<mpsc::Sender<()>>,
}

#[allow(dead_code)] // Methods used by NAPI bindings and CLI
impl ChatApp {
    pub async fn from_config(config: Config) -> Result<Self> {
        info!("🚀 Initializing Rustalk application from config...");
        
        // Create credentials from config identity
        let credentials = UserCredentials { 
            email: config.identity.email.clone(), 
            password: "".to_string() // Config-based apps don't need password verification
        };
        let engine = ReachEngine::new(credentials).await?;
        
        info!("✅ Application initialized with user ID: {}", config.identity.user_id);
        
        Ok(ChatApp {
            _config: config,
            engine: Arc::new(RwLock::new(engine)),
            _message_history: Arc::new(RwLock::new(Vec::new())),
            peers: Arc::new(RwLock::new(HashMap::new())),
            _shutdown_tx: None,
        })
    }

    pub async fn new(email: String, password: String, _port: u16) -> Result<Self> {
        info!("🚀 Initializing Rustalk application...");
        
        let credentials = UserCredentials { email, password };
        let engine = ReachEngine::new(credentials).await?;
        let config = engine.get_config().await?;
        
        info!("✅ Application initialized with user ID: {}", config.identity.user_id);
        
        Ok(ChatApp {
            _config: config,
            engine: Arc::new(RwLock::new(engine)),
            _message_history: Arc::new(RwLock::new(Vec::new())),
            peers: Arc::new(RwLock::new(HashMap::new())),
            _shutdown_tx: None,
        })
    }

    pub async fn start_server(&mut self, port: u16) -> Result<()> {
        info!("🌐 Starting server on port {}", port);
        
        let mut engine = self.engine.write().await;
        engine.start_server(port).await?;
        
        info!("✅ Server started successfully on port {}", port);
        Ok(())
    }

    pub async fn connect_to_peer(&self, address: &str) -> Result<()> {
        info!("🔗 Connecting to peer: {}", address);
        
        let engine = self.engine.read().await;
        let peer_info = engine.connect_to_peer(address).await?;
        
        // Add to connected peers
        let mut peers = self.peers.write().await;
        peers.insert(peer_info.id.to_string(), ConnectedPeer {
            id: peer_info.id.to_string(),
            email: peer_info.email.clone(),
            nickname: Some(peer_info.display_name.clone()),
            is_online: true,
            last_seen: Utc::now(),
            response_time: None,
        });

        info!("✅ Successfully connected to peer: {} ({})", peer_info.email, peer_info.id);
        Ok(())
    }

    pub async fn send_message(&self, peer_id: &str, content: &str) -> Result<()> {
        info!("📤 Sending message to peer: {}", peer_id);
        
        // Check if peer is online first
        let peer_status = self.check_peer_status(peer_id).await;
        if !peer_status.is_online {
            warn!("❌ Peer {} is offline. Message not sent.", peer_id);
            return Err(anyhow::anyhow!("Peer is offline"));
        }

        let engine = self.engine.read().await;
        let message_id = engine.send_message(peer_id, content).await?;
        
        // Store message in history
        let chat_message = ChatMessage {
            id: message_id,
            from_user_id: self._config.identity.user_id.to_string(),
            to_user_id: peer_id.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            encrypted: true,
            message_type: "text".to_string(),
        };

        let mut history = self._message_history.write().await;
        history.push(chat_message);

        info!("✅ Message sent successfully to {}", peer_id);
        Ok(())
    }

    pub async fn check_peer_status(&self, peer_id: &str) -> PeerPingStatus {
        info!("🔍 Checking status of peer: {}", peer_id);
        
        let engine = self.engine.read().await;
        let status = engine.ping_peer(peer_id).await;
        
        // Update peer status in our list
        if let Ok(mut peers) = self.peers.try_write() {
            if let Some(peer) = peers.get_mut(peer_id) {
                peer.is_online = status.is_online;
                peer.last_seen = status.last_seen;
                peer.response_time = status.response_time;
            }
        }

        info!("📊 Peer {} status: {}", peer_id, if status.is_online { "online" } else { "offline" });
        status
    }

    pub async fn get_connected_peers(&self) -> Vec<ConnectedPeer> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    pub async fn get_message_history(&self) -> Vec<ChatMessage> {
        let history = self._message_history.read().await;
        history.clone()
    }

    pub async fn set_nickname(&self, nickname: String) -> Result<()> {
        info!("👤 Setting nickname to: {}", nickname);
        
        let engine = self.engine.read().await;
        engine.set_nickname(nickname).await?;
        
        info!("✅ Nickname updated successfully");
        Ok(())
    }

    pub async fn get_user_info(&self) -> serde_json::Value {
        serde_json::json!({
            "user_id": self._config.identity.user_id,
            "email": self._config.identity.email,
            "nickname": self._config.identity.display_name,
            "public_key": self._config.identity.keypair.public_key
        })
    }

    pub async fn list_commands(&self) -> Vec<&'static str> {
        vec![
            "/connect <IP:PORT> - Connect to a peer",
            "/send <peer_id> <message> - Send message to peer",
            "/nick <name> - Set display name",
            "/peers - List connected peers",
            "/status <peer_id> - Check peer online status",
            "/history - Show message history",
            "/id - Show user ID and info",
            "/help - Show this help",
            "/quit - Exit application",
        ]
    }

    pub async fn handle_command(&self, input: &str) -> Result<String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.is_empty() || !parts[0].starts_with('/') {
            return Ok("Invalid command. Type /help for available commands.".to_string());
        }

        match parts[0] {
            "/help" => {
                let commands = self.list_commands().await;
                Ok(format!("Available commands:\n{}", commands.join("\n")))
            }
            "/id" => {
                let info = self.get_user_info().await;
                Ok(format!("User Info:\n{}", serde_json::to_string_pretty(&info)?))
            }
            "/peers" => {
                let peers = self.get_connected_peers().await;
                if peers.is_empty() {
                    Ok("No connected peers.".to_string())
                } else {
                    let peer_list: Vec<String> = peers.iter().map(|p| {
                        format!("• {} ({}) - {}", 
                               p.email, 
                               p.id, 
                               if p.is_online { "online" } else { "offline" })
                    }).collect();
                    Ok(format!("Connected Peers:\n{}", peer_list.join("\n")))
                }
            }
            "/status" => {
                if parts.len() < 2 {
                    return Ok("Usage: /status <peer_id>".to_string());
                }
                let peer_id = parts[1];
                let status = self.check_peer_status(peer_id).await;
                Ok(format!("Peer {} status: {} (last seen: {})", 
                          peer_id, 
                          if status.is_online { "online" } else { "offline" },
                          status.last_seen.format("%Y-%m-%d %H:%M:%S UTC")))
            }
            "/connect" => {
                if parts.len() < 2 {
                    return Ok("Usage: /connect <IP:PORT>".to_string());
                }
                let address = parts[1];
                match self.connect_to_peer(address).await {
                    Ok(_) => Ok(format!("Successfully connected to {}", address)),
                    Err(e) => Ok(format!("Failed to connect to {}: {}", address, e))
                }
            }
            "/send" => {
                if parts.len() < 3 {
                    return Ok("Usage: /send <peer_id> <message>".to_string());
                }
                let peer_id = parts[1];
                let message = parts[2..].join(" ");
                match self.send_message(peer_id, &message).await {
                    Ok(_) => Ok(format!("Message sent to {}", peer_id)),
                    Err(e) => Ok(format!("Failed to send message: {}", e))
                }
            }
            "/nick" => {
                if parts.len() < 2 {
                    return Ok("Usage: /nick <nickname>".to_string());
                }
                let nickname = parts[1..].join(" ");
                match self.set_nickname(nickname.clone()).await {
                    Ok(_) => Ok(format!("Nickname set to: {}", nickname)),
                    Err(e) => Ok(format!("Failed to set nickname: {}", e))
                }
            }
            "/history" => {
                let history = self.get_message_history().await;
                if history.is_empty() {
                    Ok("No message history.".to_string())
                } else {
                    let messages: Vec<String> = history.iter().take(20).map(|m| {
                        format!("[{}] {}: {}", 
                               m.timestamp.format("%H:%M:%S"), 
                               m.from_user_id, 
                               m.content)
                    }).collect();
                    Ok(format!("Recent Messages:\n{}", messages.join("\n")))
                }
            }
            "/quit" => {
                Ok("quit".to_string()) // Special return value to indicate quit
            }
            _ => {
                Ok(format!("Unknown command: {}. Type /help for available commands.", parts[0]))
            }
        }
    }

    pub async fn shutdown(&self) {
        info!("🔌 Shutting down Rustalk application...");
        
        if let Ok(engine) = self.engine.try_read() {
            engine.shutdown().await;
        }
        
        info!("✅ Application shutdown complete");
    }
}