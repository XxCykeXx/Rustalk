use anyhow::Result;
use clap::{Parser, Subcommand};
use reach::{CliOperations, SessionManager};

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
        Some(Commands::Info) => {
            match CliOperations::get_user_info().await {
                Ok(info) => println!("{}", info),
                Err(e) => {
                    eprintln!("❌ Failed to get user info: {}", e);
                    std::process::exit(1);
                }
            }
        }
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
        Some(Commands::Id) => {
            match CliOperations::get_user_info().await {
                Ok(info) => println!("{}", info),
                Err(e) => {
                    eprintln!("❌ Failed to get user info: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Reset) => {
            match CliOperations::reset_config().await {
                Ok(message) => println!("✅ {}", message),
                Err(e) => {
                    eprintln!("❌ Failed to reset config: {}", e);
                    std::process::exit(1);
                }
            }
        }
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
        println!("📡 Session: {} | Port: {} | Peers: {}", session_id, port, peer_count);
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
            let limit = parts.get(1)
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
