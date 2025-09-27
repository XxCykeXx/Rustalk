mod ui;
mod app;
mod setup;
mod path_manager;
mod user_manager;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger;

use reach::{load_config, config_exists, get_config_file};
use crate::setup::{setup_user, setup_user_with_args};
use crate::app::ChatApp;
use crate::path_manager::PathManager;
use crate::user_manager::{list_all_users, switch_user, remove_user, register_current_user};

#[derive(Parser)]
#[command(name = "rustalk")]
#[command(about = "Secure P2P terminal chat application")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup new user credentials
    Setup {
        /// Email address
        #[arg(long)]
        email: Option<String>,
        /// Display name
        #[arg(long)]
        name: Option<String>,
        /// Password
        #[arg(long)]
        password: Option<String>,
    },
    /// Start the chat application
    Chat {
        /// Port to listen on
        #[arg(short, long, default_value = "5000")]
        port: u16,
    },
    /// Show user information
    Info,
    /// Check online status of peers
    Status,
    /// Reset configuration
    Reset,
    /// Add Rustalk to system PATH
    AddPath,
    /// Remove Rustalk from system PATH
    RemovePath,
    /// Check if Rustalk is in system PATH
    CheckPath,
    /// List all registered users
    ListUsers,
    /// Switch to a different user
    SwitchUser {
        /// User ID to switch to
        user_id: String,
    },
    /// Remove a user from the registry
    RemoveUser {
        /// User ID to remove
        user_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Setup { email, name, password }) => {
            if let (Some(email), Some(name), Some(password)) = (email, name, password) {
                setup_user_with_args(email, name, password).await?;
            } else {
                setup_user().await?;
            }
        }
        Some(Commands::Chat { port }) => {
            if !config_exists() {
                println!("🔐 No configuration found. Please run 'rustalk setup' first.");
                return Ok(());
            }
            
            let config = load_config()?;
            let mut app = ChatApp::from_config(config).await?;
            app.start_server(port).await?;
        }
        Some(Commands::Info) => {
            if !config_exists() {
                println!("No configuration found. Please run 'rustalk setup' first.");
                return Ok(());
            }
            
            let config = load_config()?;
            println!("🆔 User ID: {}", config.identity.user_id);
            println!("📧 Email: {}", config.identity.email);
            println!("👤 Display Name: {}", config.identity.get_display_name());
            println!("🔑 Public Key: {}", config.identity.keypair.public_key);
            println!("🌐 Default Port: {}", config.default_port);
        }
        Some(Commands::Status) => {
            if !config_exists() {
                println!("🔐 No configuration found. Please run 'rustalk setup' first.");
                return Ok(());
            }
            
            let config = load_config()?;
            let app = ChatApp::from_config(config).await?;
            
            println!("🔄 Checking peer status...");
            let peers = app.get_connected_peers().await;
            
            if peers.is_empty() {
                println!("📡 No peers connected.");
                println!("💡 Use '/connect <address>' to connect to peers first.");
            } else {
                println!("👥 Connected Peers:");
                for peer in peers {
                    let status_icon = if peer.is_online { "🟢" } else { "🔴" };
                    let response_time = peer.response_time
                        .map(|rt| format!(" ({}ms)", rt))
                        .unwrap_or_else(|| " (unknown)".to_string());
                    
                    println!("  {} {} - {}{}", 
                        status_icon, 
                        peer.nickname.as_deref().unwrap_or("Unknown"),
                        peer.email,
                        response_time
                    );
                }
            }
        }
        Some(Commands::Reset) => {
            if let Ok(config_file) = get_config_file() {
                if config_file.exists() {
                    std::fs::remove_file(config_file)?;
                    println!("✅ Configuration reset. Please run 'rustalk setup' to create new credentials.");
                } else {
                    println!("No configuration found to reset.");
                }
            }
        }
        Some(Commands::AddPath) => {
            let path_manager = PathManager::new()?;
            
            if path_manager.is_in_path() {
                println!("✅ Rustalk is already in your PATH");
                println!("📁 Location: {}", path_manager.get_install_location().display());
            } else {
                println!("➕ Adding Rustalk to system PATH...");
                match path_manager.add_to_path() {
                    Ok(()) => {
                        // Register current user if config exists
                        if config_exists() {
                            if let Err(e) = register_current_user() {
                                eprintln!("⚠️  Warning: Failed to register user: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to add to PATH: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        Some(Commands::RemovePath) => {
            let path_manager = PathManager::new()?;
            
            if !path_manager.is_in_path() {
                println!("ℹ️  Rustalk is not in your PATH");
            } else {
                println!("➖ Removing Rustalk from system PATH...");
                path_manager.remove_from_path()?;
            }
        }
        Some(Commands::CheckPath) => {
            let path_manager = PathManager::new()?;
            
            println!("🔍 PATH Check Results:");
            println!("📁 Install Location: {}", path_manager.get_install_location().display());
            println!("🖥️  Platform: {:?}", path_manager.get_platform());
            
            if path_manager.is_in_path() {
                println!("✅ Rustalk is in your PATH");
                println!("💡 You can run 'rustalk' from anywhere in your terminal");
            } else {
                println!("❌ Rustalk is NOT in your PATH");
                println!("💡 Run 'rustalk add-path' to add it to your PATH");
            }
        }
        Some(Commands::ListUsers) => {
            list_all_users()?;
        }
        Some(Commands::SwitchUser { user_id }) => {
            switch_user(&user_id)?;
        }
        Some(Commands::RemoveUser { user_id }) => {
            remove_user(&user_id)?;
        }
        None => {
            // Default behavior - try to start chat or show help
            if config_exists() {
                let config = load_config()?;
                let mut app = ChatApp::from_config(config).await?;
                app.start_server(5000).await?;
            } else {
                println!("🔐 Welcome to Rustalk!");
                println!("Please run 'rustalk setup' to configure your credentials.");
                println!("Then use 'rustalk chat' to start chatting.");
            }
        }
    }
    
    Ok(())
}
