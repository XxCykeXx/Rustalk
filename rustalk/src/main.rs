use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "rustalk")]
#[command(about = "🦀 Rustalk - P2P secure chat powered by Reach\n   Complete feature set via delegation to rus CLI")]
#[command(version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure your credentials
    Setup,
    /// Start interactive chat
    Chat {
        /// Port to listen on
        #[arg(short, long, default_value = "5000")]
        port: u16,
    },
    /// Show your information
    Info,
    /// Start chat and auto-connect to peer
    Connect {
        /// Peer address (IP:PORT)
        address: String,
        /// Port to listen on
        #[arg(short, long, default_value = "5000")]
        port: u16,
    },
    /// Send quick message (interactive mode)
    Send {
        /// Message to send
        message: String,
    },
    /// List connected peers (interactive mode)
    Peers,
    /// Set display name
    Nick {
        /// Display name
        name: String,
    },
    /// Reset configuration
    Reset,
    /// PATH management commands
    Path {
        #[command(subcommand)]
        action: PathCommands,
    },
    /// User management commands
    Users {
        #[command(subcommand)]
        action: UsersCommands,
    },
}

#[derive(Subcommand)]
enum PathCommands {
    /// Add rustalk to system PATH
    Add,
    /// Remove rustalk from PATH
    Remove,
    /// Check PATH status
    Check,
}

#[derive(Subcommand)]
enum UsersCommands {
    /// List all registered users
    List,
    /// Switch to different user
    Switch {
        /// User ID to switch to
        id: String,
    },
    /// Show current user
    Current,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // All commands delegate to rus CLI with appropriate arguments
    let mut cmd = Command::new("rus");
    
    match cli.command {
        Some(Commands::Setup) => {
            cmd.arg("setup");
        }
        Some(Commands::Chat { port }) => {
            cmd.args(&["chat", "--port", &port.to_string()]);
        }
        Some(Commands::Info) => {
            cmd.arg("info");
        }
        Some(Commands::Connect { address, port }) => {
            cmd.args(&["connect", &address, "--port", &port.to_string()]);
        }
        Some(Commands::Send { message }) => {
            cmd.args(&["send", &message]);
        }
        Some(Commands::Peers) => {
            cmd.arg("peers");
        }
        Some(Commands::Nick { name }) => {
            cmd.args(&["nick", &name]);
        }
        Some(Commands::Reset) => {
            cmd.arg("reset");
        }
        Some(Commands::Path { action }) => {
            cmd.arg("path");
            match action {
                PathCommands::Add => cmd.arg("add"),
                PathCommands::Remove => cmd.arg("remove"),
                PathCommands::Check => cmd.arg("check"),
            };
        }
        Some(Commands::Users { action }) => {
            cmd.arg("users");
            match action {
                UsersCommands::List => cmd.arg("list"),
                UsersCommands::Switch { id } => cmd.args(&["switch", &id]),
                UsersCommands::Current => cmd.arg("current"),
            };
        }
        None => {
            show_help();
            return Ok(());
        }
    }

    // Execute the rus command
    let status = cmd.status()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn show_help() {
    println!("🦀 Welcome to Rustalk - Rustalk CLI!");
    println!("   Easy-to-use P2P secure chat powered by Reach");
    println!();
    println!("🚀 Quick Start:");
    println!("   rustalk setup              - Configure your credentials");
    println!("   rustalk chat               - Start interactive chat");
    println!("   rustalk info               - Show your information");
    println!();
    println!("💬 Chat Commands:");
    println!("   rustalk connect <ip:port>  - Start chat and auto-connect");
    println!("   rustalk send <message>     - Send quick message (interactive mode)");
    println!("   rustalk peers              - List connected peers (interactive mode)");
    println!("   rustalk nick <name>        - Set display name (interactive mode)");
    println!();
    println!("🔧 Management:");
    println!("   rustalk reset              - Reset configuration");
    println!("   rustalk path add           - Add rustalk to system PATH");
    println!("   rustalk path remove        - Remove rustalk from PATH");
    println!("   rustalk path check         - Check PATH status");
    println!("   rustalk users list         - List all registered users");
    println!("   rustalk users switch <id>  - Switch to different user");
    println!("   rustalk users current      - Show current user");
    println!("   rustalk --help             - Show detailed help");
    println!();
    println!("💡 Example workflow:");
    println!("   1. rustalk setup           # Set up your credentials");
    println!("   2. rustalk chat            # Start interactive chat");
    println!("   3. /connect 192.168.1.100:5000  # Connect to peer");
    println!("   4. Hello there!            # Send messages");
    println!("   5. /quit                   # Exit chat");
    println!();
    println!("🌟 Features:");
    println!("   • End-to-end encryption with AES-256-GCM");
    println!("   • Peer-to-peer networking with no central server");
    println!("   • Cross-platform support (Windows, macOS, Linux)");
    println!("   • User management and session persistence");
}
