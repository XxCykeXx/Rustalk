use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use dirs;

use crate::{Identity, UserCredentials, Config, SessionManager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub email: String,
    pub name: String,
    pub created_at: String,
    pub last_used: String,
    pub config_path: PathBuf,
}

pub struct UserManager {
    users_file: PathBuf,
    current_user_file: PathBuf,
}

impl UserManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("rustalk");
        
        fs::create_dir_all(&config_dir)?;
        
        Ok(UserManager {
            users_file: config_dir.join("users.json"),
            current_user_file: config_dir.join("current_user"),
        })
    }

    pub fn register_user(&self, credentials: &UserCredentials, config_path: PathBuf) -> Result<()> {
        let mut users = self.load_users()?;
        
        let profile = UserProfile {
            email: credentials.email.clone(),
            name: credentials.name.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            last_used: chrono::Utc::now().to_rfc3339(),
            config_path,
        };
        
        users.insert(credentials.email.clone(), profile);
        self.save_users(&users)?;
        
        // Set as current user
        fs::write(&self.current_user_file, &credentials.email)?;
        
        Ok(())
    }

    pub fn list_users(&self) -> Result<Vec<UserProfile>> {
        let users = self.load_users()?;
        Ok(users.values().cloned().collect())
    }

    pub fn switch_user(&self, email: &str) -> Result<()> {
        let users = self.load_users()?;
        
        if users.contains_key(email) {
            fs::write(&self.current_user_file, email)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("User '{}' not found", email))
        }
    }

    pub fn remove_user(&self, email: &str) -> Result<()> {
        let mut users = self.load_users()?;
        
        if let Some(user) = users.remove(email) {
            // Remove config file if it exists
            if user.config_path.exists() {
                fs::remove_file(&user.config_path)?;
            }
            
            self.save_users(&users)?;
            
            // If this was the current user, clear it
            if let Ok(current) = self.get_current_user() {
                if current == email {
                    let _ = fs::remove_file(&self.current_user_file);
                }
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("User '{}' not found", email))
        }
    }

    pub fn get_current_user(&self) -> Result<String> {
        fs::read_to_string(&self.current_user_file)
            .map(|s| s.trim().to_string())
            .map_err(|_| anyhow::anyhow!("No current user set"))
    }

    pub fn get_user_profile(&self, email: &str) -> Result<UserProfile> {
        let users = self.load_users()?;
        users.get(email)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("User '{}' not found", email))
    }

    fn load_users(&self) -> Result<HashMap<String, UserProfile>> {
        if !self.users_file.exists() {
            return Ok(HashMap::new());
        }
        
        let contents = fs::read_to_string(&self.users_file)?;
        let users: HashMap<String, UserProfile> = serde_json::from_str(&contents)
            .unwrap_or_else(|_| HashMap::new());
        
        Ok(users)
    }

    fn save_users(&self, users: &HashMap<String, UserProfile>) -> Result<()> {
        let contents = serde_json::to_string_pretty(users)?;
        fs::write(&self.users_file, contents)?;
        Ok(())
    }
}

pub struct PathManager;

impl PathManager {
    pub fn add_to_path() -> Result<String> {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent()
            .ok_or_else(|| anyhow::anyhow!("Could not get executable directory"))?;
        
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let path_str = exe_dir.to_string_lossy();
            
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!(
                        "[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ';{}', 'User')",
                        path_str
                    )
                ])
                .output()?;
            
            if output.status.success() {
                Ok(format!("Added {} to PATH", path_str))
            } else {
                Err(anyhow::anyhow!("Failed to add to PATH: {}", String::from_utf8_lossy(&output.stderr)))
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            // For Unix-like systems, we'll add to ~/.bashrc and ~/.zshrc
            let home = dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
            
            let path_str = exe_dir.to_string_lossy();
            let export_line = format!("export PATH=\"$PATH:{}\"", path_str);
            
            for shell_config in &[".bashrc", ".zshrc"] {
                let config_file = home.join(shell_config);
                if config_file.exists() {
                    let contents = fs::read_to_string(&config_file)?;
                    if !contents.contains(&path_str) {
                        fs::write(&config_file, format!("{}\n{}", contents, export_line))?;
                    }
                }
            }
            
            Ok(format!("Added {} to shell configurations", path_str))
        }
    }

    pub fn remove_from_path() -> Result<String> {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent()
            .ok_or_else(|| anyhow::anyhow!("Could not get executable directory"))?;
        
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let path_str = exe_dir.to_string_lossy();
            
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!(
                        "[Environment]::SetEnvironmentVariable('PATH', ([Environment]::GetEnvironmentVariable('PATH', 'User') -replace ';?{}', ''), 'User')",
                        regex::escape(&path_str)
                    )
                ])
                .output()?;
            
            if output.status.success() {
                Ok(format!("Removed {} from PATH", path_str))
            } else {
                Err(anyhow::anyhow!("Failed to remove from PATH: {}", String::from_utf8_lossy(&output.stderr)))
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            let home = dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
            
            let path_str = exe_dir.to_string_lossy();
            
            for shell_config in &[".bashrc", ".zshrc"] {
                let config_file = home.join(shell_config);
                if config_file.exists() {
                    let contents = fs::read_to_string(&config_file)?;
                    let new_contents = contents.lines()
                        .filter(|line| !line.contains(&path_str))
                        .collect::<Vec<_>>()
                        .join("\n");
                    fs::write(&config_file, new_contents)?;
                }
            }
            
            Ok(format!("Removed {} from shell configurations", path_str))
        }
    }

    pub fn check_in_path() -> Result<String> {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent()
            .ok_or_else(|| anyhow::anyhow!("Could not get executable directory"))?;
        
        let path_env = std::env::var("PATH")?;
        let path_str = exe_dir.to_string_lossy();
        
        if path_env.contains(path_str.as_ref()) {
            Ok(format!("✓ {} is in PATH", path_str))
        } else {
            Ok(format!("✗ {} is NOT in PATH", path_str))
        }
    }
}

pub struct CliOperations;

impl CliOperations {
    pub async fn setup_user(email: Option<String>, name: Option<String>, password: Option<String>) -> Result<UserCredentials> {
        use std::io::{self, Write};
        
        let email = match email {
            Some(e) => e,
            None => {
                print!("Enter your email: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            }
        };

        let name = match name {
            Some(n) => n,
            None => {
                print!("Enter your display name: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            }
        };

        let password = match password {
            Some(p) => p,
            None => {
                print!("Enter your password: ");
                io::stdout().flush()?;
                // For now, just read plain text. In production, use rpassword crate
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            }
        };

        let credentials = UserCredentials { email, name, password };
        
        // Create identity and save config
        let identity = Identity::new(credentials.clone())?;
        let config = Config::new(identity);
        
        let config_file = crate::config::get_config_file()?;
        crate::config::save_config(&config)?;
        
        // Register user
        let user_manager = UserManager::new()?;
        user_manager.register_user(&credentials, config_file)?;
        
        Ok(credentials)
    }

    pub async fn start_chat_session(port: u16) -> Result<SessionManager> {
        // Load current user
        let user_manager = UserManager::new()?;
        let current_email = user_manager.get_current_user()?;
        let _profile = user_manager.get_user_profile(&current_email)?;
        
        // Load config
        let config = crate::config::load_config()?;
        
        let session_manager = SessionManager::new(config.identity).await?;
        session_manager.start_session(port).await?;
        
        Ok(session_manager)
    }

    pub async fn get_user_info() -> Result<String> {
        let user_manager = UserManager::new()?;
        let current_email = user_manager.get_current_user()?;
        let profile = user_manager.get_user_profile(&current_email)?;
        
        Ok(format!(
            "📧 Email: {}\n👤 Name: {}\n📅 Created: {}\n🕒 Last used: {}",
            profile.email, profile.name, profile.created_at, profile.last_used
        ))
    }

    pub async fn reset_config() -> Result<String> {
        let config_file = crate::config::get_config_file()?;
        if config_file.exists() {
            fs::remove_file(&config_file)?;
            Ok("Configuration reset successfully".to_string())
        } else {
            Ok("No configuration to reset".to_string())
        }
    }
}