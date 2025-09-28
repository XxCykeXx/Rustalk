use crate::identity::Identity;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub identity: Identity,
    pub default_port: u16,
    pub auto_accept_connections: bool,
    pub max_peers: usize,
    pub log_level: String,
}

impl Config {
    pub fn new(identity: Identity) -> Self {
        Config {
            identity,
            default_port: 5000,
            auto_accept_connections: false,
            max_peers: 10,
            log_level: "info".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            identity: Identity::new(crate::identity::UserCredentials {
                email: "anonymous@rustalk.local".to_string(),
                name: "Anonymous".to_string(),
                password: "default".to_string(),
            })
            .expect("Failed to create default identity"),
            default_port: 5000,
            auto_accept_connections: false,
            max_peers: 10,
            log_level: "info".to_string(),
        }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    // Try to get platform-specific config directory first
    let config_dir = if let Some(config_home) = dirs::config_dir() {
        config_home.join("rustalk")
    } else if let Some(home) = dirs::home_dir() {
        // Fallback to home directory with dot prefix
        #[cfg(windows)]
        let dir = home.join("AppData").join("Local").join("rustalk");
        #[cfg(not(windows))]
        let dir = home.join(".rustalk");
        dir
    } else {
        return Err(anyhow!(
            "Could not determine config directory - no home or config directory found"
        ));
    };

    // Ensure directory exists with proper permissions
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).map_err(|e| {
            anyhow!(
                "Failed to create config directory {}: {}",
                config_dir.display(),
                e
            )
        })?;

        // Set appropriate permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(&config_dir)
                .map_err(|e| anyhow!("Failed to get config directory metadata: {}", e))?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o700); // Read/write/execute for owner only
            std::fs::set_permissions(&config_dir, permissions)
                .map_err(|e| anyhow!("Failed to set config directory permissions: {}", e))?;
        }
    }

    Ok(config_dir)
}

pub fn get_config_file() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("config.json"))
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_file = get_config_file()?;
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;

    std::fs::write(config_file, json).map_err(|e| anyhow!("Failed to write config file: {}", e))
}

pub fn load_config() -> Result<Config> {
    let config_file = get_config_file()?;

    if !config_file.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }

    let contents = std::fs::read_to_string(config_file)
        .map_err(|e| anyhow!("Failed to read config file: {}", e))?;

    let config: Config = serde_json::from_str(&contents)
        .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;

    Ok(config)
}

pub fn config_exists() -> bool {
    get_config_file().map(|path| path.exists()).unwrap_or(false)
}
