use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use reach::{Config, get_config_file};

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
        let registry_file = get_user_registry_file()?;
        
        if !registry_file.exists() {
            return Ok(UserRegistry::default());
        }

        let contents = fs::read_to_string(&registry_file)
            .map_err(|e| anyhow!("Failed to read user registry: {}", e))?;

        let registry: UserRegistry = serde_json::from_str(&contents)
            .map_err(|e| anyhow!("Failed to parse user registry: {}", e))?;

        Ok(registry)
    }

    pub fn save(&self) -> Result<()> {
        let registry_file = get_user_registry_file()?;
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize user registry: {}", e))?;

        fs::write(registry_file, json)
            .map_err(|e| anyhow!("Failed to write user registry: {}", e))
    }

    pub fn add_user(&mut self, config: &Config) -> Result<()> {
        let user_info = UserInfo {
            user_id: config.identity.user_id.to_string(),
            email: config.identity.email.clone(),
            display_name: config.identity.get_display_name(),
            public_key: config.identity.keypair.public_key.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            last_active: chrono::Utc::now().to_rfc3339(),
        };

        let user_id_str = config.identity.user_id.to_string();
        self.users.insert(user_id_str.clone(), user_info);
        self.current_user = Some(user_id_str);
        self.save()
    }

    pub fn update_last_active(&mut self, user_id: &str) -> Result<()> {
        if let Some(user) = self.users.get_mut(user_id) {
            user.last_active = chrono::Utc::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
    }

    pub fn set_current_user(&mut self, user_id: &str) -> Result<()> {
        if self.users.contains_key(user_id) {
            self.current_user = Some(user_id.to_string());
            self.update_last_active(user_id)?;
            self.save()
        } else {
            Err(anyhow!("User {} not found in registry", user_id))
        }
    }

    pub fn remove_user(&mut self, user_id: &str) -> Result<()> {
        if self.users.remove(user_id).is_some() {
            if self.current_user.as_ref() == Some(&user_id.to_string()) {
                self.current_user = None;
            }
            self.save()
        } else {
            Err(anyhow!("User {} not found", user_id))
        }
    }

    pub fn list_users(&self) -> Vec<&UserInfo> {
        self.users.values().collect()
    }

    pub fn get_current_user(&self) -> Option<&UserInfo> {
        self.current_user.as_ref()
            .and_then(|user_id| self.users.get(user_id))
    }

    pub fn get_user(&self, user_id: &str) -> Option<&UserInfo> {
        self.users.get(user_id)
    }
}

pub fn get_user_registry_file() -> Result<PathBuf> {
    let config_file = get_config_file()?;
    let config_dir = config_file.parent()
        .ok_or_else(|| anyhow!("Could not get config directory"))?;
    Ok(config_dir.join("users.json"))
}

pub fn list_all_users() -> Result<()> {
    let registry = UserRegistry::load()?;
    let users = registry.list_users();

    if users.is_empty() {
        println!("👥 No users found.");
        println!("💡 Run 'rustalk setup' to create your first user.");
        return Ok(());
    }

    println!("👥 Registered Users ({} total):", users.len());
    println!();

    for (index, user) in users.iter().enumerate() {
        let is_current = registry.current_user.as_ref() == Some(&user.user_id);
        let status_icon = if is_current { "👤" } else { "  " };
        
        println!("{}{}. {}", status_icon, index + 1, user.display_name);
        println!("   📧 Email: {}", user.email);
        println!("   🆔 ID: {}", user.user_id);
        println!("   🔑 Public Key: {}...", &user.public_key[..20]);
        println!("   📅 Created: {}", format_timestamp(&user.created_at));
        println!("   🕒 Last Active: {}", format_timestamp(&user.last_active));
        
        if is_current {
            println!("   ⭐ Current User");
        }
        
        println!();
    }

    if let Some(current_user) = registry.get_current_user() {
        println!("Current active user: {} ({})", current_user.display_name, current_user.user_id);
    }

    Ok(())
}

pub fn switch_user(user_id: &str) -> Result<()> {
    let mut registry = UserRegistry::load()?;
    
    if registry.get_user(user_id).is_some() {
        let user_name = registry.get_user(user_id).unwrap().display_name.clone();
        let user_email = registry.get_user(user_id).unwrap().email.clone();
        registry.set_current_user(user_id)?;
        println!("✅ Switched to user: {} ({})", user_name, user_email);
    } else {
        return Err(anyhow!("User with ID '{}' not found", user_id));
    }

    Ok(())
}

pub fn remove_user(user_id: &str) -> Result<()> {
    let mut registry = UserRegistry::load()?;
    
    if let Some(user) = registry.get_user(user_id) {
        let user_name = user.display_name.clone();
        registry.remove_user(user_id)?;
        println!("✅ Removed user: {} ({})", user_name, user_id);
        
        if registry.current_user.is_none() && !registry.users.is_empty() {
            let first_user_id = registry.users.keys().next().unwrap().clone();
            registry.set_current_user(&first_user_id)?;
            println!("👤 Switched to user: {}", registry.get_current_user().unwrap().display_name);
        }
    } else {
        return Err(anyhow!("User with ID '{}' not found", user_id));
    }

    Ok(())
}

fn format_timestamp(timestamp: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        dt.format("%Y-%m-%d %H:%M UTC").to_string()
    } else {
        timestamp.to_string()
    }
}

pub fn register_current_user() -> Result<()> {
    let config = reach::load_config()?;
    let mut registry = UserRegistry::load()?;
    registry.add_user(&config)?;
    Ok(())
}