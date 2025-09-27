use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;

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
            run_rustalk_command(&["setup"]).await?;
        }
        Some(Commands::Chat { port }) => {
            run_rustalk_command(&["chat", "--port", &port.to_string()]).await?;
        }
        Some(Commands::Connect { address, port }) => {
            println!("🔗 Starting Rustalk and connecting to {}...", address);
            // This would need to be implemented to auto-connect after starting
            run_rustalk_command(&["chat", "--port", &port.to_string()]).await?;
        }
        Some(Commands::Info) => {
            run_rustalk_command(&["info"]).await?;
        }
        Some(Commands::Send { message: _, to: _ }) => {
            println!("📤 Send command not yet implemented in this version");
            println!("💡 Please use the interactive chat mode: rus chat");
        }
        Some(Commands::Peers) => {
            println!("👥 Peers command not yet implemented in this version");
            println!("💡 Please use the interactive chat mode: rus chat");
        }
        Some(Commands::Nick { name }) => {
            println!("👤 Nick command not yet implemented in this version");
            println!("💡 Please use the interactive chat mode and type: /nick {}", name);
        }
        Some(Commands::Id) => {
            run_rustalk_command(&["info"]).await?;
        }
        Some(Commands::Reset) => {
            run_rustalk_command(&["reset"]).await?;
        }
        None => {
            // Show help or start default mode
            show_interactive_help().await;
        }
    }
    
    Ok(())
}

async fn run_rustalk_command(args: &[&str]) -> Result<()> {
    let output = Command::new("rustalk")
        .args(args)
        .output();
        
    match output {
        Ok(output) => {
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
            Ok(())
        }
        Err(_) => {
            // If rustalk binary is not found, try to run from source
            println!("💡 Rustalk binary not found. Make sure it's installed:");
            println!("   cargo install --path packages/rustalk");
            println!("   OR run: cargo run -p rustalk -- {}", args.join(" "));
            Ok(())
        }
    }
}

async fn show_interactive_help() {
    println!("🦀 Welcome to Rus - Rustalk CLI!");
    println!("   Easy-to-use P2P secure chat");
    println!();
    println!("🚀 Quick Start:");
    println!("   rus setup      - Configure your credentials");
    println!("   rus chat       - Start chatting");
    println!("   rus info       - Show your information");
    println!();
    println!("💬 Chat Commands:");
    println!("   rus connect <ip:port>  - Start chat and connect to peer");
    println!("   rus send <message>     - Send quick message");
    println!("   rus peers              - List connected peers");
    println!("   rus nick <name>        - Set display name");
    println!();
    println!("🔧 Management:");
    println!("   rus reset      - Reset configuration");
    println!("   rus --help     - Show detailed help");
    println!();
    println!("💡 Example workflow:");
    println!("   1. rus setup");
    println!("   2. rus chat");
    println!("   3. In chat: /connect 192.168.1.100:5000");
    println!("   4. Start chatting!");
}
