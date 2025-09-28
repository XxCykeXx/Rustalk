use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use reach::{CliOperations, SessionManager};

mod path_manager;
mod user_manager;

use path_manager::*;
use user_manager::*;

#[derive(Parser)]
#[command(name = "rus")]
#[command(about = "Rustalk CLI - Easy-to-use wrapper for Rustalk P2P chat")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup new user credentials
    Setup,
    /// Start chatting
    Chat {
        /// Port to listen on
        #[arg(short, long, default_value = "5000")]
        port: u16,
    },
    /// Connect to a peer directly
    Connect {
        /// Peer address (IP:PORT)
        address: String,
        /// Port to listen on
        #[arg(short, long, default_value = "5000")]
        port: u16,
    },
    /// Show user information  
    Info,
    /// Send a quick message (requires active session)
    Send {
        /// Message to send
        message: String,
        /// Target peer ID (optional)
        #[arg(short, long)]
        to: Option<String>,
    },
    /// List connected peers
    Peers,
    /// Set display name
    Nick {
        /// New display name
        name: String,
    },
    /// Show your unique ID
    Id,
    /// Reset configuration
    Reset,
    /// System and PATH management
    Path {
        #[command(subcommand)]
        action: PathCommands,
    },
    /// User management operations
    Users {
        #[command(subcommand)]
        action: UserCommands,
    },
}

#[derive(Subcommand)]
enum PathCommands {
    /// Add rustalk to system PATH
    Add,
    /// Remove rustalk from system PATH  
    Remove,
    /// Check if rustalk is in PATH
    Check,
    /// Show current PATH status
    Status,
}

#[derive(Subcommand)]
enum UserCommands {
    /// List all registered users
    List,
    /// Switch to different user
    Switch {
        /// User ID to switch to
        user_id: String,
    },
    /// Remove a user from registry
    Remove {
        /// User ID to remove
        user_id: String,
    },
    /// Show current user
    Current,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Setup) => {
            println!("🔧 Setting up Rustalk credentials...");
            match CliOperations::setup_user(None, None, None).await {
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
            println!("🚀 Starting chat session on port {}...", port);
            match CliOperations::start_chat_session(port).await {
                Ok(session_manager) => {
                    start_interactive_chat(session_manager).await?;
                }
                Err(e) => {
                    eprintln!("❌ Failed to start chat: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Connect { address, port }) => {
            println!("🔗 Starting chat and connecting to {}...", address);
            match CliOperations::start_chat_session(port).await {
                Ok(session_manager) => {
                    if let Err(e) = session_manager.connect_to_peer(&address).await {
                        eprintln!("⚠️  Failed to connect to {}: {}", address, e);
                    } else {
                        println!("✅ Connected to {}", address);
                    }
                    start_interactive_chat(session_manager).await?;
                }
                Err(e) => {
                    eprintln!("❌ Failed to start chat: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Info) => match CliOperations::get_user_info().await {
            Ok(info) => println!("{}", info),
            Err(e) => {
                eprintln!("❌ Failed to get user info: {}", e);
                std::process::exit(1);
            }
        },
        Some(Commands::Send { message: _, to: _ }) => {
            println!("📤 Send functionality requires an active chat session");
            println!("💡 Use 'rus chat' first, then send messages interactively");
        }
        Some(Commands::Peers) => {
            println!("👥 Peer list functionality requires an active chat session");
            println!("💡 Use 'rus chat' to see connected peers");
        }
        Some(Commands::Nick { name }) => {
            println!("👤 Nickname functionality requires an active chat session");
            println!("💡 Use 'rus chat' then type '/nick {}' in the chat", name);
        }
        Some(Commands::Id) => match CliOperations::get_user_info().await {
            Ok(info) => println!("{}", info),
            Err(e) => {
                eprintln!("❌ Failed to get user info: {}", e);
                std::process::exit(1);
            }
        },
        Some(Commands::Reset) => match CliOperations::reset_config().await {
            Ok(message) => println!("✅ {}", message),
            Err(e) => {
                eprintln!("❌ Failed to reset config: {}", e);
                std::process::exit(1);
            }
        },
        Some(Commands::Path { action }) => match action {
            PathCommands::Add => match add_to_path() {
                Ok(()) => println!("✅ Successfully added rustalk to PATH"),
                Err(e) => {
                    eprintln!("❌ Failed to add to PATH: {}", e);
                    std::process::exit(1);
                }
            },
            PathCommands::Remove => match remove_from_path() {
                Ok(()) => println!("✅ Successfully removed rustalk from PATH"),
                Err(e) => {
                    eprintln!("❌ Failed to remove from PATH: {}", e);
                    std::process::exit(1);
                }
            },
            PathCommands::Check => match check_in_path() {
                Ok(true) => println!("✅ rustalk is in PATH"),
                Ok(false) => println!("❌ rustalk is not in PATH"),
                Err(e) => {
                    eprintln!("❌ Failed to check PATH: {}", e);
                    std::process::exit(1);
                }
            },
            PathCommands::Status => match get_path_status() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("❌ Failed to get PATH status: {}", e);
                    std::process::exit(1);
                }
            },
        },
        Some(Commands::Users { action }) => match action {
            UserCommands::List => match list_all_users() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("❌ Failed to list users: {}", e);
                    std::process::exit(1);
                }
            },
            UserCommands::Switch { user_id } => match switch_user(&user_id) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("❌ Failed to switch user: {}", e);
                    std::process::exit(1);
                }
            },
            UserCommands::Remove { user_id } => match remove_user(&user_id) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("❌ Failed to remove user: {}", e);
                    std::process::exit(1);
                }
            },
            UserCommands::Current => match UserRegistry::load() {
                Ok(registry) => {
                    if let Some(user) = registry.get_current_user() {
                        println!("👤 Current user: {} ({})", user.display_name, user.user_id);
                        println!("📧 Email: {}", user.email);
                        println!("🕒 Last active: {}", user.last_active);
                    } else {
                        println!("❌ No current user set");
                        println!("💡 Run 'rus setup' to create a user");
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to get current user: {}", e);
                    std::process::exit(1);
                }
            },
        },
        None => {
            show_interactive_help().await;
        }
    }

    Ok(())
}

async fn start_interactive_chat(session_manager: SessionManager) -> Result<()> {
    use std::io::{self, Write};

    println!("💬 Chat session started! Type '/help' for commands or '/quit' to exit");

    if let Some((session_id, port, peer_count)) = session_manager.get_session_info().await {
        println!(
            "📡 Session: {} | Port: {} | Peers: {}",
            session_id, port, peer_count
        );
    }

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.starts_with('/') {
            match handle_chat_command(&session_manager, input).await {
                Ok(should_quit) => {
                    if should_quit {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Command error: {}", e);
                }
            }
        } else {
            // Send message
            if let Err(e) = session_manager.send_message(input.to_string(), None).await {
                eprintln!("❌ Failed to send message: {}", e);
            } else {
                println!("📤 Message sent");
            }
        }
    }

    println!("👋 Ending chat session...");
    session_manager.end_session().await?;
    Ok(())
}

async fn handle_chat_command(session_manager: &SessionManager, command: &str) -> Result<bool> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    match parts.get(0) {
        Some(&"/help") => {
            println!("💡 Available commands:");
            println!("   /connect <ip:port>  - Connect to a peer");
            println!("   /peers              - List connected peers");
            println!("   /info               - Show session info");
            println!("   /history [limit]    - Show recent messages");
            println!("   /quit               - Exit chat");
            println!("   /help               - Show this help");
            println!();
            println!("💬 Just type normally to send messages!");
        }
        Some(&"/connect") => {
            if let Some(address) = parts.get(1) {
                match session_manager.connect_to_peer(address).await {
                    Ok(()) => println!("✅ Connected to {}", address),
                    Err(e) => eprintln!("❌ Failed to connect: {}", e),
                }
            } else {
                println!("❌ Usage: /connect <ip:port>");
            }
        }
        Some(&"/peers") => {
            let peers = session_manager.get_active_peers().await;
            if peers.is_empty() {
                println!("� No connected peers");
            } else {
                println!("👥 Connected peers ({}):", peers.len());
                for peer in peers {
                    println!("   • {} ({})", peer.display_name, peer.id);
                }
            }
        }
        Some(&"/info") => {
            if let Some((session_id, port, peer_count)) = session_manager.get_session_info().await {
                println!("📡 Session Info:");
                println!("   ID: {}", session_id);
                println!("   Port: {}", port);
                println!("   Connected peers: {}", peer_count);
            }
        }
        Some(&"/history") => {
            let limit = parts
                .get(1)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(10);

            let messages = session_manager.list_recent_messages(limit).await;
            if messages.is_empty() {
                println!("📜 No message history");
            } else {
                println!("📜 Recent messages ({}):", messages.len());
                for msg in messages {
                    println!("   [{}] {}: {}", msg.timestamp, msg.sender(), msg.content);
                }
            }
        }
        Some(&"/quit") | Some(&"/exit") => {
            return Ok(true);
        }
        _ => {
            println!("❌ Unknown command: {}", command);
            println!("💡 Type '/help' for available commands");
        }
    }

    Ok(false)
}

async fn show_interactive_help() {
    println!("🦀 Welcome to Rus - Rustalk CLI!");
    println!("   Easy-to-use P2P secure chat powered by Reach");
    println!();
    println!("🚀 Quick Start:");
    println!("   rus setup              - Configure your credentials");
    println!("   rus chat               - Start interactive chat");
    println!("   rus info               - Show your information");
    println!();
    println!("💬 Chat Commands:");
    println!("   rus connect <ip:port>  - Start chat and auto-connect");
    println!("   rus send <message>     - Send quick message (interactive mode)");
    println!("   rus peers              - List connected peers (interactive mode)");
    println!("   rus nick <name>        - Set display name (interactive mode)");
    println!();
    println!("🔧 Management:");
    println!("   rus reset              - Reset configuration");
    println!("   rus path add           - Add rustalk to system PATH");
    println!("   rus path remove        - Remove rustalk from PATH");
    println!("   rus path check         - Check PATH status");
    println!("   rus users list         - List all registered users");
    println!("   rus users switch <id>  - Switch to different user");
    println!("   rus users current      - Show current user");
    println!("   rus --help             - Show detailed help");
    println!();
    println!("💡 Example workflow:");
    println!("   1. rus setup           # Set up your credentials");
    println!("   2. rus chat            # Start interactive chat");
    println!("   3. /connect 192.168.1.100:5000  # Connect to peer");
    println!("   4. Hello there!        # Send messages");
    println!("   5. /quit               # Exit chat");
    println!();
    println!("🌟 Features:");
    println!("   • End-to-end encryption with AES-256-GCM");
    println!("   • Peer-to-peer networking with no central server");
    println!("   • Cross-platform support (Windows, macOS, Linux)");
    println!("   • User management and session persistence");
}

fn list_all_users() -> Result<()> {
    let registry = UserRegistry::load()?;
    let users = registry.list_users();

    if users.is_empty() {
        println!("👥 No users found.");
        println!("💡 Run 'rus setup' to create your first user.");
        return Ok(());
    }

    println!("👥 Registered Users ({} total):", users.len());
    println!();

    for (index, (user_id, user)) in users.iter().enumerate() {
        let is_current = registry.current_user.as_ref() == Some(user_id);
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
        println!(
            "Current active user: {} ({})",
            current_user.display_name, current_user.user_id
        );
    }

    Ok(())
}

fn switch_user(user_id: &str) -> Result<()> {
    let mut registry = UserRegistry::load()?;

    if let Some(user) = registry.get_user(user_id) {
        let user_name = user.display_name.clone();
        let user_email = user.email.clone();
        registry.set_current_user(user_id.to_string())?;
        println!("✅ Switched to user: {} ({})", user_name, user_email);
    } else {
        return Err(anyhow!("User with ID '{}' not found", user_id));
    }

    Ok(())
}

fn remove_user(user_id: &str) -> Result<()> {
    let mut registry = UserRegistry::load()?;

    if let Some(user) = registry.get_user(user_id) {
        let user_name = user.display_name.clone();
        registry.remove_user(user_id)?;
        println!("✅ Removed user: {} ({})", user_name, user_id);

        if registry.current_user.is_none() && !registry.users.is_empty() {
            let first_user_id = registry.users.keys().next().unwrap().clone();
            registry.set_current_user(first_user_id)?;
            if let Some(new_current) = registry.get_current_user() {
                println!("👤 Switched to user: {}", new_current.display_name);
            }
        }
    } else {
        return Err(anyhow!("User with ID '{}' not found", user_id));
    }

    Ok(())
}
