use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeerStatus {
    Connected,
    Connecting,
    Disconnected,
    HandshakeInProgress,
    Authenticated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub address: SocketAddr,
    pub public_key: String,
    pub status: PeerStatus,
    pub connected_at: Option<DateTime<Utc>>,
    pub last_seen: DateTime<Utc>,
}

impl Peer {
    pub fn new(
        id: Uuid,
        email: String,
        display_name: String,
        address: SocketAddr,
        public_key: String,
    ) -> Self {
        Peer {
            id,
            email,
            display_name,
            address,
            public_key,
            status: PeerStatus::Connecting,
            connected_at: None,
            last_seen: Utc::now(),
        }
    }

    pub fn set_connected(&mut self) {
        self.status = PeerStatus::Connected;
        self.connected_at = Some(Utc::now());
        self.last_seen = Utc::now();
    }

    pub fn set_authenticated(&mut self) {
        self.status = PeerStatus::Authenticated;
        self.last_seen = Utc::now();
    }

    pub fn set_disconnected(&mut self) {
        self.status = PeerStatus::Disconnected;
        self.last_seen = Utc::now();
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            self.status,
            PeerStatus::Connected | PeerStatus::Authenticated
        )
    }

    pub fn connection_duration(&self) -> Option<chrono::Duration> {
        if let Some(connected_at) = self.connected_at {
            Some(Utc::now() - connected_at)
        } else {
            None
        }
    }
}

/// Status information for ping operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerPingStatus {
    pub user_id: String,
    pub is_online: bool,
    pub last_seen: DateTime<Utc>,
    pub response_time: Option<u64>, // in milliseconds
}

impl PeerPingStatus {
    pub fn online(user_id: String, response_time: u64) -> Self {
        Self {
            user_id,
            is_online: true,
            last_seen: Utc::now(),
            response_time: Some(response_time),
        }
    }

    pub fn offline(user_id: String) -> Self {
        Self {
            user_id,
            is_online: false,
            last_seen: Utc::now(),
            response_time: None,
        }
    }
}
