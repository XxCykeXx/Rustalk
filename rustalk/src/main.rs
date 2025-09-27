mod ui;
mod app;
mod setup;
mod path_manager;
mod user_manager;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger;

use reach::{load_config, config_exists, get_config_file, CliOperations, UserManager, PathManager, SessionManager};
use crate::setup::{setup_user, setup_user_with_args};
use crate::app::ChatApp;
// Remove local implementations that are now in reach
// use crate::path_manager::PathManager;
// use crate::user_manager::{list_all_users, switch_user, remove_user, register_current_user};

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
            match CliOperations::setup_user(email, name, password).await {
                Ok(credentials) => {
                    println!("✅ Setup complete for {}", credentials.name);
                    println!("📧 Email: {}", credentials.email);
                }
                Err(e) => {
                    eprintln!("❌ Setup failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Chat { port }) => {
            println!("🚀 Starting advanced chat session on port {}...", port);
            match CliOperations::start_chat_session(port).await {
                Ok(session_manager) => {
                    // For rustalk, we'll use the advanced ChatApp UI
                    if config_exists() {
                        let config = load_config()?;
                        let mut app = ChatApp::from_config(config).await?;
                        app.start_server(port).await?;
                    } else {
                        println!("🔐 No configuration found. Please run 'rustalk setup' first.");
                        return Ok(());
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to start chat: {}", e);
                    println!("💡 Try running 'rustalk setup' first");
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Info) => {
            match CliOperations::get_user_info().await {
                Ok(info) => println!("{}", info),
                Err(e) => {
                    eprintln!("❌ Failed to get user info: {}", e);
                    println!("� Run 'rustalk setup' first to configure credentials");
                    std::process::exit(1);
                }
            }
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
            match CliOperations::reset_config().await {
                Ok(message) => {
                    println!("✅ {}", message);
                    println!("💡 Run 'rustalk setup' to create new credentials");
                }
                Err(e) => {
                    eprintln!("❌ Failed to reset config: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::AddPath) => {
            match PathManager::add_to_path() {
                Ok(message) => {
                    println!("✅ {}", message);
                    println!("💡 You can now run 'rustalk' from anywhere in your terminal");
                }
                Err(e) => {
                    eprintln!("❌ Failed to add to PATH: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::RemovePath) => {
            match PathManager::remove_from_path() {
                Ok(message) => {
                    println!("✅ {}", message);
                }
                Err(e) => {
                    eprintln!("❌ Failed to remove from PATH: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::CheckPath) => {
            match PathManager::check_in_path() {
                Ok(message) => {
                    println!("{}", message);
                    if message.contains("NOT in PATH") {
                        println!("� Run 'rustalk add-path' to add it to your PATH");
                    } else {
                        println!("💡 You can run 'rustalk' from anywhere in your terminal");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to check PATH: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::ListUsers) => {
            let user_manager = UserManager::new()?;
            match user_manager.list_users() {
                Ok(users) => {
                    if users.is_empty() {
                        println!("👤 No registered users found");
                        println!("💡 Run 'rustalk setup' to register a new user");
                    } else {
                        println!("👥 Registered users ({}):", users.len());
                        for user in users {
                            let current = if let Ok(current_email) = user_manager.get_current_user() {
                                if current_email == user.email { " (current)" } else { "" }
                            } else { "" };
                            println!("  📧 {} - {}{}", user.email, user.name, current);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to list users: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::SwitchUser { user_id }) => {
            let user_manager = UserManager::new()?;
            match user_manager.switch_user(&user_id) {
                Ok(()) => {
                    println!("✅ Switched to user: {}", user_id);
                }
                Err(e) => {
                    eprintln!("❌ Failed to switch user: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::RemoveUser { user_id }) => {
            let user_manager = UserManager::new()?;
            match user_manager.remove_user(&user_id) {
                Ok(()) => {
                    println!("✅ Removed user: {}", user_id);
                }
                Err(e) => {
                    eprintln!("❌ Failed to remove user: {}", e);
                    std::process::exit(1);
                }
            }
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
