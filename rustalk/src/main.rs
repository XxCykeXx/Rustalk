use anyhow::Result;
use clap::{Parser, Subcommand};
use rus;
use std::process::Command; // Import rus crate for delegation

#[derive(Parser)]
#[command(name = "rustalk")]
#[command(about = "Rustalk installer and starter - delegates operations to rus CLI")]
#[command(version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install rustalk system-wide and add to PATH
    Install,
    /// Run rustalk operations via rus CLI (delegation)
    Run {
        /// Arguments to pass to rus
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Quick start - setup and chat
    Start {
        /// Port to listen on
        #[arg(short, long, default_value = "5000")]
        port: u16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install) => {
            println!("📦 Installing rustalk system-wide...");

            // Use rus directly instead of subprocess
            match rus::add_to_path() {
                Ok(()) => {
                    println!("✅ Installation complete!");
                    println!("💡 Rustalk is now available system-wide");
                    println!("🚀 Use 'rustalk start' to begin chatting");
                }
                Err(e) => {
                    eprintln!("❌ Installation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Run { args }) => {
            let mut cmd = Command::new("rus");
            if !args.is_empty() {
                cmd.args(&args);
            }

            let mut child = cmd.spawn()?;
            let status = child.wait()?;

            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Some(Commands::Start { port }) => {
            println!("🚀 Starting rustalk...");

            // Check if setup is needed using reach config directly
            if !reach::config_exists() {
                println!("🔧 Setting up rustalk for first use...");
                // For initial setup, still delegate to rus CLI for interactive setup
                let setup_result = Command::new("rus").args(&["setup"]).status()?;

                if !setup_result.success() {
                    eprintln!("❌ Setup failed");
                    std::process::exit(1);
                }
            }

            // Start chat session via direct rus CLI delegation
            println!("💬 Starting chat session on port {}...", port);
            let mut child = Command::new("rus")
                .args(&["chat", "--port", &port.to_string()])
                .spawn()?;

            let status = child.wait()?;
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        None => {
            show_help();
        }
    }

    Ok(())
}

fn show_help() {
    println!(" Rustalk - P2P Chat Installer & Starter");
    println!("   Delegates operations to 'rus' CLI for actual functionality");
    println!();
    println!(" Installation:");
    println!("   rustalk install        - Install rustalk system-wide");
    println!();
    println!(" Quick Start:");
    println!("   rustalk start          - Setup and start chatting");
    println!("   rustalk start -p 6000  - Start on custom port");
    println!();
    println!(" Advanced Usage:");
    println!("   rustalk run <args>     - Pass commands to rus CLI");
    println!("   rustalk run setup      - Setup user credentials");
    println!("   rustalk run users list - List all users");
    println!("   rustalk run path add   - Add to system PATH");
    println!();
    println!(" Examples:");
    println!("   rustalk run chat --port 5000");
    println!("   rustalk run connect 192.168.1.100:5000");
    println!("   rustalk run users switch <user-id>");
    println!();
    println!("ℹ  All actual operations are handled by the 'rus' CLI");
    println!("   Make sure 'rus' is available in your PATH");
}
