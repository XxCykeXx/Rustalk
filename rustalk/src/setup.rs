use anyhow::Result;
use std::io::{self, Write};
use reach::{UserCredentials, Identity, Config, save_config, load_config, config_exists};

pub async fn setup_user() -> Result<()> {
    // Check if configuration already exists
    if config_exists() {
        println!("� Existing configuration found!");
        print!("Do you want to create a new configuration? This will replace your current identity. (y/N): ");
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        response = response.trim().to_lowercase();
        
        if response != "y" && response != "yes" {
            println!("✅ Using existing configuration.");
            let config = load_config()?;
            println!("🆔 Your unique ID: {}", config.identity.user_id);
            println!("👤 Display name: {}", config.identity.get_display_name());
            println!("🔑 Public key: {}", config.identity.keypair.public_key);
            println!("\n💡 Share your unique ID with friends to connect!");
            println!("🚀 Run 'rustalk start' to start chatting.");
            return Ok(());
        }
        println!("⚠️  Creating new configuration...");
    }

    println!("�🔐 Welcome to Rustalk Setup!");
    println!("Please provide your credentials for secure P2P communication.\n");

    // Get email (make it optional)
    print!("📧 Enter your email (optional, press Enter to skip): ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    email = email.trim().to_string();

    // Use default email if empty
    if email.is_empty() {
        email = format!("user-{}", uuid::Uuid::new_v4().to_string()[..8].to_lowercase());
    }

    // Get password (make it optional with default)
    print!("🔑 Enter your password (optional, press Enter for secure default): ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    password = password.trim().to_string();

    // Use secure default if empty
    if password.is_empty() {
        let uuid_str = uuid::Uuid::new_v4().to_string();
        password = format!("secure-{}", &uuid_str[..16]);
        println!("🔒 Generated secure password for you.");
    }

    // Get display name (optional) - ask first to use in credentials
    print!("👤 Enter display name (optional, press Enter to use email prefix): ");
    io::stdout().flush()?;
    let mut display_name = String::new();
    io::stdin().read_line(&mut display_name)?;
    let display_name = display_name.trim();
    let name = if display_name.is_empty() {
        email.split('@').next().unwrap_or("User").to_string()
    } else {
        display_name.to_string()
    };

    // Create identity
    let credentials = UserCredentials { 
        email: email.clone(),
        name,
        password 
    };
    let identity = Identity::new(credentials)?;

    let config = Config {
        identity,
        default_port: 5000,
        auto_accept_connections: false,
        max_peers: 10,
        log_level: "info".to_string(),
    };

    // Save configuration
    save_config(&config)?;

    // Register user in the user registry
    let mut registry = crate::user_manager::UserRegistry::load().unwrap_or_default();
    if let Err(e) = registry.add_user(&config) {
        eprintln!("⚠️  Warning: Failed to register user: {}", e);
    }

    println!("\n✅ Setup complete!");
    println!("🆔 Your unique ID: {}", config.identity.user_id);
    println!("👤 Display name: {}", config.identity.get_display_name());
    println!("🔑 Public key: {}", config.identity.keypair.public_key);
    println!("\n💡 Share your unique ID with friends to connect!");
    println!("🚀 Run 'rustalk start' to start chatting.");

    Ok(())
}

pub async fn setup_user_with_args(email: String, display_name: String, password: String) -> Result<()> {
    println!("🔐 Setting up Rustalk with provided credentials...");

    if email.is_empty() {
        return Err(anyhow::anyhow!("Email cannot be empty"));
    }

    if password.is_empty() {
        return Err(anyhow::anyhow!("Password cannot be empty"));
    }

    // Create identity
    let credentials = UserCredentials { 
        email: email.clone(),
        name: display_name.clone(),
        password 
    };
    let identity = Identity::new(credentials)?;

    let mut config = Config {
        identity,
        default_port: 5000,
        auto_accept_connections: false,
        max_peers: 10,
        log_level: "info".to_string(),
    };

    if !display_name.is_empty() {
        config.identity.set_display_name(display_name);
    }

    // Save configuration
    save_config(&config)?;

    // Register user in the user registry
    let mut registry = crate::user_manager::UserRegistry::load().unwrap_or_default();
    if let Err(e) = registry.add_user(&config) {
        eprintln!("⚠️  Warning: Failed to register user: {}", e);
    }

    println!("\n✅ Setup completed successfully!");
    println!("🆔 Your unique ID: {}", config.identity.user_id);
    println!("👤 Display name: {}", config.identity.get_display_name());
    println!("🔑 Public key: {}", config.identity.keypair.public_key);
    println!("\n💡 Share your unique ID with friends to connect!");
    
    Ok(())
}