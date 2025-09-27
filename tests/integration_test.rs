use tokio;
use uuid::Uuid;
use reach::{ReachEngine, UserCredentials, Config};

#[tokio::test]
async fn test_engine_initialization() {
    let credentials = UserCredentials {
        email: "test@example.com".to_string(),
        password: "test123".to_string(),
    };

    let result = ReachEngine::new(credentials).await;
    assert!(result.is_ok(), "Engine should initialize successfully");
}

#[tokio::test]
async fn test_config_creation() {
    let credentials = UserCredentials {
        email: "test@example.com".to_string(),
        password: "test123".to_string(),
    };

    let engine = ReachEngine::new(credentials).await.unwrap();
    let config = engine.get_config().await.unwrap();
    
    assert_eq!(config.identity.email, "test@example.com");
    assert!(!config.identity.user_id.is_empty());
    assert!(!config.identity.public_key.is_empty());
}

#[tokio::test]
async fn test_server_start_stop() {
    let credentials = UserCredentials {
        email: "test@example.com".to_string(),
        password: "test123".to_string(),
    };

    let mut engine = ReachEngine::new(credentials).await.unwrap();
    
    // Start server on a random port
    let result = engine.start_server(0).await;
    assert!(result.is_ok(), "Server should start successfully");
    
    // Cleanup
    engine.shutdown().await;
}

#[tokio::test]
async fn test_peer_status_check() {
    let credentials = UserCredentials {
        email: "test@example.com".to_string(),
        password: "test123".to_string(),
    };

    let engine = ReachEngine::new(credentials).await.unwrap();
    
    // Check status of non-existent peer
    let fake_peer_id = "fake_peer_123";
    let status = engine.ping_peer(fake_peer_id).await;
    
    assert!(!status.is_online, "Non-existent peer should be offline");
}

#[tokio::test]
async fn test_message_creation() {
    use reach::Message;
    use chrono::Utc;

    let message = Message {
        id: Uuid::new_v4().to_string(),
        from_user_id: "user1".to_string(),
        to_user_id: "user2".to_string(),
        content: "Hello, World!".to_string(),
        timestamp: Utc::now(),
        encrypted: true,
        message_type: "text".to_string(),
    };

    assert_eq!(message.content, "Hello, World!");
    assert_eq!(message.from_user_id, "user1");
    assert_eq!(message.to_user_id, "user2");
    assert!(message.encrypted);
}