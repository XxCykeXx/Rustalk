# Rustalk 🦀💬

A secure, peer-to-peer terminal chat application built with Rust and TypeScript.

> This project is in active development. Features and commands may change frequently.


## About

Rustalk enables secure, end-to-end encrypted communication between users without requiring a central server. The application features cross-platform compatibility and supports both native Rust binaries and Node.js integration via NAPI bindings.

## Architecture

Rustalk is built with a modular three-crate architecture:

- 📚 **`reach`**: Core P2P networking and encryption library
- 🦀 **`rustalk`**: Main chat application library (depends on `reach`)
- ⚡ **`rus`**: CLI wrapper and utilities (depends on both `reach` and `rustalk`)

## Features

- 🔐 **End-to-End Encryption**: All messages are encrypted using industry-standard cryptography
- 🌐 **Peer-to-Peer**: Direct communication between users without central servers
- 🚀 **Cross-Platform**: Works on Windows, macOS, and Linux
- 💻 **Terminal Interface**: Beautiful TUI built with Ratatui
- 🔑 **Identity Management**: Secure key generation and management
- 📡 **Network Discovery**: Automatic peer detection and connection
- ⚡ **High Performance**: Built with Rust for maximum speed and safety
- 📦 **Modular Design**: Each component can be used independently

## Usage

### Installation

#### Option 1: Install from NPM (Recommended)

```bash
# Install the complete Rustalk package (includes both rustalk and rus binaries)
npm install -g rustalk
```

This will install both:
- `rustalk` - Main chat application with TypeScript CLI
- `rus` - Rust CLI wrapper

#### Option 2: Install from Cargo (Individual Components)

```bash
# Install all components
cargo install reach rustalk rus

# Or install individually as needed:
cargo install reach      # Core networking library
cargo install rustalk    # Main chat application  
cargo install rus        # CLI wrapper and utilities
```

#### Option 3: Build from Source

```bash
# Clone the repository
git clone https://github.com/muhammad-fiaz/Rustalk.git
cd Rustalk

# Install dependencies and build
npm install
npm run build
```

## Development

The project uses local path dependencies during development:
- `rustalk` uses local `reach` from `../packages/reach`
- `rus` uses local `reach` and `rustalk` from their relative paths

This allows for easy development and testing. When publishing to crates.io, dependencies are temporarily changed to use published versions - see `PUBLISHING-ORDER.md` for details.

### Setup

```bash
# Initial setup (creates user configuration)
bun run src/index.ts setup
```

### Commands

#### Basic Commands
```bash
# Start the chat application
bun run src/index.ts start

# Connect to a peer
bun run src/index.ts connect <peer-address>

# Check peer status
bun run src/index.ts status

# Show help
bun run src/index.ts --help

# Show version
bun run src/index.ts --version

# Run tests
bun run src/index.ts test
```

#### PATH Management
```bash
# Add Rustalk to system PATH (Windows/Linux/macOS)
bun run src/index.ts add-path

# Remove Rustalk from system PATH
bun run src/index.ts remove-path

# Check if Rustalk is in system PATH
bun run src/index.ts check-path
```

#### User Management
```bash
# List all registered users
bun run src/index.ts list-users

# Switch to a different user
bun run src/index.ts switch-user <user-id>

# Remove a user from registry
bun run src/index.ts remove-user <user-id>
```

### Configuration

User configuration is stored in `~/.rustalk/config.json`. This includes:
- User identity and keys
- Display name
- Peer connections
- Application settings

**Multi-User Support**: Rustalk supports multiple user accounts:
- User registry stored in `~/.rustalk/users.json`
- Switch between different identities
- Each user has separate encryption keys

**Cross-Platform PATH Management**: 
- Automatically detects your OS (Windows/Linux/macOS)
- Adds Rustalk binary to system PATH
- Works with PowerShell, Bash, Zsh, and other shells

**Note**: Configuration persists across updates and reinstalls. If you forget your password, delete the `~/.rustalk` folder to create a new identity.

## License

Licensed under Apache 2.0. For more details, see the [LICENSE](LICENSE) file.

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed information about contributing to this project.
