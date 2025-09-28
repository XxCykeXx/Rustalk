use anyhow::{Result, anyhow};
use reach::get_config_file;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub public_key: String,
    pub created_at: String,
    pub last_active: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistry {
    pub users: HashMap<String, UserInfo>,
    pub current_user: Option<String>,
}

impl Default for UserRegistry {
    fn default() -> Self {
        UserRegistry {
            users: HashMap::new(),
            current_user: None,
        }
    }
}

impl UserRegistry {
    pub fn load() -> Result<Self> {
        let config_file = get_config_file()?;
        let users_file = config_file
            .parent()
            .ok_or_else(|| anyhow!("Invalid config directory"))?
            .join("users.json");

        if users_file.exists() {
            let content = fs::read_to_string(&users_file)
                .map_err(|e| anyhow!("Failed to read users file: {}", e))?;

            serde_json::from_str(&content).map_err(|e| anyhow!("Failed to parse users file: {}", e))
        } else {
            Ok(UserRegistry::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_file = get_config_file()?;
        let users_file = config_file
            .parent()
            .ok_or_else(|| anyhow!("Invalid config directory"))?
            .join("users.json");

        // Create directory if it doesn't exist
        if let Some(parent) = users_file.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| anyhow!("Failed to create config directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize users: {}", e))?;

        fs::write(&users_file, json).map_err(|e| anyhow!("Failed to write users file: {}", e))?;

        Ok(())
    }

    pub fn get_current_user(&self) -> Option<&UserInfo> {
        self.current_user.as_ref().and_then(|id| self.users.get(id))
    }

    pub fn set_current_user(&mut self, user_id: String) -> Result<()> {
        if self.users.contains_key(&user_id) {
            self.current_user = Some(user_id);
            self.save()
        } else {
            Err(anyhow!("User not found: {}", user_id))
        }
    }

    pub fn list_users(&self) -> Vec<(&String, &UserInfo)> {
        self.users.iter().collect()
    }

    pub fn remove_user(&mut self, user_id: &str) -> Result<()> {
        if self.users.remove(user_id).is_some() {
            // If we removed the current user, clear the current user
            if self.current_user.as_ref().map(|s| s.as_str()) == Some(user_id) {
                self.current_user = None;
            }
            self.save()
        } else {
            Err(anyhow!("User not found: {}", user_id))
        }
    }

    pub fn get_user(&self, user_id: &str) -> Option<&UserInfo> {
        self.users.get(user_id)
    }
}

// Helper function to format timestamps for display
pub fn format_timestamp(timestamp: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        dt.format("%Y-%m-%d %H:%M UTC").to_string()
    } else {
        timestamp.to_string()
    }
}
