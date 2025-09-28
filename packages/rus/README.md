# Rus ⚡

**CLI Operations Library and Binary for Rustalk**

Rus is the command-line interface and operations library that provides all user-facing functionality for Rustalk. It handles user management, system configuration, and serves as the primary interface for P2P chat operations.

## Features

- 💻 **Full CLI Interface**: Complete command-line interface for all operations
- 👥 **User Management**: Multi-user support with secure credential storage
- 🛠️ **System Integration**: PATH management and system configuration
- 🌐 **P2P Operations**: Direct peer-to-peer communication via reach library
- 📁 **Configuration Management**: Secure configuration storage and management
- 🎨 **Beautiful TUI**: Terminal user interface built with Ratatui
- 📦 **Library Support**: Can be used as library by other Rust projects

## Installation

### Via Cargo (Recommended)
```bash
# Install rus CLI directly
cargo install rus

# After installation, ensure ~/.cargo/bin is in PATH
rus setup
rus chat
```

### Via NPM (Full Rustalk Suite)
```bash
# Installs rustalk + rus with automatic PATH setup
npm install -g rustalk

# Both commands available:
rustalk setup  # TypeScript interface
rus setup      # Rust CLI
```

### From Source
```bash
git clone <repository>
cd rustalk-workflow
cargo build --release -p rus
```

### As Library
```toml
[dependencies]
rus = "0.0.1"
```

## Usage

### Command Line Interface

```bash
# Setup user credentials
rus setup

# Start chat session
rus chat --port 5000

# Connect to a peer
rus connect 192.168.1.100:5000

# User management
rus users list
rus users switch user@example.com

# System configuration
rus path add
rus config show
```

### Library Usage

```rust
use rus::{add_to_path, UserRegistry};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Add to system PATH
    add_to_path()?;
    
    // Use user management
    let mut registry = UserRegistry::new()?;
    registry.list_users()?;
    
    Ok(())
}
```

## Core Components

- **CLI Interface**: Full command-line interface with clap
- **UserRegistry**: Multi-user management and authentication
- **PathManager**: System PATH and installation management
- **Config Integration**: Direct integration with reach configuration
- **TUI Components**: Terminal user interface elements

## Commands

### Setup & Configuration
- `rus setup` - Initial user setup and credential configuration
- `rus config show` - Display current configuration
- `rus path add` - Add rus to system PATH

### Chat Operations
- `rus chat [--port PORT]` - Start chat session
- `rus connect ADDRESS` - Connect to peer directly
- `rus peers` - List connected peers

### User Management
- `rus users list` - List all registered users
- `rus users switch EMAIL` - Switch active user
- `rus users info` - Show current user information

## License

Apache-2.0 License. See [LICENSE](../../LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for contribution guidelines.

---

Part of the [Rustalk](../../README.md) project.