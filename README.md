# Rustalk 🦀💬

**Modular P2P Communication Platform** - Secure terminal chat with end-to-end encryption

Rustalk is a secure, peer-to-peer terminal chat application built with a modular architecture featuring three main components:

- 🌐 **`reach`**: Core P2P networking library with end-to-end encryption
- ⚡ **`rus`**: CLI operations and user management interface  
- 🚀 **`rustalk`**: Binary installer and starter with npm integration

## Features

- 🔐 **End-to-End Encryption**: All messages are encrypted using industry-standard cryptography
- 🌐 **Peer-to-Peer**: Direct communication between users without central servers
- 🚀 **Cross-Platform**: Works on Windows, macOS, and Linux
- 💻 **Terminal Interface**: Beautiful TUI built with Ratatui
- 🔑 **Identity Management**: Secure key generation and management
- 📡 **Network Discovery**: Automatic peer detection and connection
- ⚡ **High Performance**: Built with Rust for maximum speed and safety
- 📦 **Modular Design**: Each component can be used independently

## 🚀 Installation & Usage

## 🚀 Installation & Quick Start

### Installation

Choose your preferred installation method:

#### 📦 NPM (Recommended)
```bash
npm install -g rustalk
```

#### 🦀 Cargo (Rust Native)  
```bash
cargo install rustalk
```

### Quick Start

```bash
# Install system-wide and add to PATH
rustalk install

# Quick start (auto-setup if needed)
rustalk start

# Or use direct CLI operations
rus setup          # Setup user credentials
rus chat           # Start chat session
rus connect <peer> # Connect to peer
```

### From Source

```bash
git clone https://github.com/muhammad-fiaz/rustalk.git
cd rustalk-workflow
bun install
bun run build
```

## 💻 Development
- **bin/rustalk.ts** - Main TypeScript CLI interface
- **packages/reach/** - Core P2P networking library (Rust)
- **packages/rus/** - Direct CLI operations (Rust)
- **rustalk/** - Advanced application (Rust)

### Local Development
```bash
# Clone and setup
git clone https://github.com/muhammad-fiaz/rustalk.git
cd rustalk
npm install

# Build Rust components
cargo build --release --workspace

# Run directly with TypeScript
npx tsx bin/rustalk.ts setup
npx tsx bin/rustalk.ts chat
```

### Commands

#### Basic Commands (TypeScript Interface)
```bash
# Setup your identity
rustalk setup

# Start P2P chat
rustalk chat

# Connect to a peer directly
rustalk connect <ip:port>

# Show user information
rustalk info

### Usage Examples

```bash
# System Management
rustalk install           # Install system-wide
rustalk start             # Quick start
rustalk start --port 6000 # Custom port

# Direct CLI Operations (via rus)
rustalk run setup         # Setup credentials
rustalk run chat          # Start chat
rustalk run users list    # List users
rustalk run connect <ip:port>
```

### Direct CLI Usage
```bash
# Use rus directly for all operations
rus setup                 # Initial setup
rus chat --port 5000     # Start chat session
rus connect <address>    # Connect to peer
rus users list           # User management
rus info                 # Show user info
```

### Package Information

Each package includes:
- **Individual README.md** with package-specific documentation
- **Apache 2.0 LICENSE** file
- **Modular architecture** allowing independent usage

**Configuration**: 
- User data stored in `~/.rustalk/config.json`
- Multi-user support with `~/.rustalk/users.json`
- Cross-platform compatibility (Windows, macOS, Linux)

## License

Licensed under Apache 2.0. For more details, see the [LICENSE](LICENSE) file.

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed information about contributing to this project.
