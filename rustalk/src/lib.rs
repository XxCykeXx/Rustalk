use napi_derive::napi;
use tokio::sync::RwLock;
use std::sync::Arc;

mod app;
mod setup;
mod ui;
mod path_manager;
mod user_manager;

pub use app::*;
pub use setup::*;
pub use ui::*;
pub use path_manager::*;
pub use user_manager::*;

// Native Node.js bindings
#[napi]
pub struct RustalkApp {
    app: Arc<RwLock<Option<ChatApp>>>,
}

#[napi]
impl RustalkApp {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            app: Arc::new(RwLock::new(None)),
        }
    }

    #[napi]
    pub async fn initialize(&self, email: String, password: String, port: Option<u16>) -> napi::Result<String> {
        let port = port.unwrap_or(8080);
        
        match ChatApp::new(email, password, port).await {
            Ok(chat_app) => {
                let mut app = self.app.write().await;
                *app = Some(chat_app);
                Ok(format!("Rustalk initialized on port {}", port))
            }
            Err(e) => Err(napi::Error::from_reason(format!("Failed to initialize: {}", e)))
        }
    }

    #[napi]
    pub async fn connect_to_peer(&self, address: String) -> napi::Result<bool> {
        let app = self.app.read().await;
        if let Some(chat_app) = app.as_ref() {
            match chat_app.connect_to_peer(&address).await {
                Ok(_) => Ok(true),
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            Err(napi::Error::from_reason("App not initialized"))
        }
    }

    #[napi]
    pub async fn send_message(&self, peer_id: String, message: String) -> napi::Result<bool> {
        let app = self.app.read().await;
        if let Some(chat_app) = app.as_ref() {
            match chat_app.send_message(&peer_id, &message).await {
                Ok(_) => Ok(true),
                Err(e) => {
                    eprintln!("Send message failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            Err(napi::Error::from_reason("App not initialized"))
        }
    }

    #[napi]
    pub async fn check_peer_status(&self, peer_id: String) -> napi::Result<String> {
        let app = self.app.read().await;
        if let Some(chat_app) = app.as_ref() {
            let status = chat_app.check_peer_status(&peer_id).await;
            Ok(serde_json::to_string(&status).unwrap_or_else(|_| "offline".to_string()))
        } else {
            Err(napi::Error::from_reason("App not initialized"))
        }
    }

    #[napi]
    pub async fn get_peer_list(&self) -> napi::Result<String> {
        let app = self.app.read().await;
        if let Some(chat_app) = app.as_ref() {
            let peers = chat_app.get_connected_peers().await;
            Ok(serde_json::to_string(&peers).unwrap_or_else(|_| "[]".to_string()))
        } else {
            Err(napi::Error::from_reason("App not initialized"))
        }
    }

    #[napi]
    pub async fn shutdown(&self) -> napi::Result<()> {
        let mut app = self.app.write().await;
        if let Some(chat_app) = app.take() {
            chat_app.shutdown().await;
        }
        Ok(())
    }
}