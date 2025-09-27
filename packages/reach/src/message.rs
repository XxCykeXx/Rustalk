use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Connect,
    Disconnect,
    Handshake,
    KeyExchange,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Option<Uuid>,
    pub message_type: MessageType,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub sender_name: String,
}

impl Message {
    pub fn new(
        sender_id: Uuid,
        recipient_id: Option<Uuid>,
        message_type: MessageType,
        content: String,
        sender_name: String,
    ) -> Self {
        Message {
            id: Uuid::new_v4(),
            sender_id,
            recipient_id,
            message_type,
            content,
            timestamp: Utc::now(),
            sender_name,
        }
    }

    pub fn text_message(
        sender_id: Uuid,
        recipient_id: Uuid,
        content: String,
        sender_name: String,
    ) -> Self {
        Self::new(
            sender_id,
            Some(recipient_id),
            MessageType::Text,
            content,
            sender_name,
        )
    }

    pub fn system_message(content: String) -> Self {
        Self::new(
            Uuid::nil(),
            None,
            MessageType::System,
            content,
            "System".to_string(),
        )
    }

    pub fn handshake_message(sender_id: Uuid, public_key: String, sender_name: String) -> Self {
        Self::new(
            sender_id,
            None,
            MessageType::Handshake,
            public_key,
            sender_name,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub encrypted_content: String,
    pub timestamp: DateTime<Utc>,
    pub sender_public_key: String,
}

impl EncryptedMessage {
    pub fn from_message(message: &Message, encrypted_content: String, sender_public_key: String) -> Self {
        EncryptedMessage {
            sender_id: message.sender_id,
            recipient_id: message.recipient_id.unwrap_or(Uuid::nil()),
            encrypted_content,
            timestamp: message.timestamp,
            sender_public_key,
        }
    }
}