# Rustalk 🦀💬

**Modular P2P Communication Platform** - Secure terminal chat with end-to-end encryption

> 🚀 **New Architecture**: Now featuring a clean three-tier modular design for maximum flexibility!

## 🏗️ Architecture Overview

Rustalk has been completely restructured into a modular platform:

```
┌─────────────────────────────────────────────────┐
│                    rustalk                      │
│              (Unified Experience)               │
├─────────────────────────────────────────────────┤
│                      rus                        │
│               (CLI Operations)                  │
├─────────────────────────────────────────────────┤
│                     reach                       │
│           (Core P2P Networking)                 │
└─────────────────────────────────────────────────┘
```

### 📦 **Packages**

- 🌐 **`reach`**: Core P2P networking, cryptography, and session management
- ⚡ **`rus`**: Direct CLI interface for all user operations  
- 🚀 **`rustalk`**: Unified experience combining both reach and rus capabilities

### 🎯 **Design Philosophy**

- **Single Install**: One command installs everything (`cargo install rustalk` or `npm install rustalk`)  
- **Dual Interface**: Use `rus` for direct CLI operations or `rustalk` for unified experience
- **Modular Core**: Each component can be used independently
- **Clean Dependencies**: rus → reach, rustalk → reach (no circular dependencies)

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

### Quick Install

Choose your preferred installation method:

#### 📦 NPM (Recommended - TypeScript)
```bash
npm install -g rustalk
```

#### 🦀 Cargo (Rust Native)  
```bash
cargo install rustalk
```

**After installation you get**:
- 🚀 **`rustalk`** - Main TypeScript interface (works everywhere)
- ⚡ **`rus`** - Direct Rust CLI (available after setup)

### 🏁 Quick Start

```bash
# 1. Set up your identity
rustalk setup
# or: rus setup

# 2. Start chatting
rustalk chat
# or: rus chat

# 3. Connect to friends
rustalk connect 192.168.1.100:5000
# or: rus connect 192.168.1.100:5000
```

### 🔧 Advanced Installation

#### From Source
```bash
git clone https://github.com/muhammad-fiaz/rustalk.git
cd rustalk
cargo build --release --workspace

# Install dependencies and build
npm install
npm run build
```

## 💻 Development

### Architecture
Rustalk uses a **TypeScript-first** approach:
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

# Send quick message
rustalk send <message> --to <peer>

# List connected peers
rustalk peers

# Show help
rustalk --help
```

#### Alternative: Direct Rust CLI
```bash
# After setup, you can also use rus directly
rus chat
rus connect <ip:port>
rus info
```

#### System Management
```bash
# Add to system PATH
rustalk add-path

# Remove from system PATH
rustalk remove-path

# Reset all configuration
rustalk reset
```

#### User Management
```bash
# List all registered users
rustalk list-users

# Switch to different user
rustalk switch-user <email>

# Set display name
rustalk nick <name>
```

### Configuration

**User Data**: Stored in `~/.rustalk/config.json`
- User identity and encryption keys
- Display name and preferences  
- Peer connections history

**Multi-User Support**: 
- Registry: `~/.rustalk/users.json`
- Switch between different identities with `rustalk switch-user`
- Each user has separate encryption keys

**Cross-Platform**: 
- Works on Windows, macOS, and Linux
- TypeScript interface provides universal compatibility
- Rust binaries optimized for each platform

**Two Ways to Use**:
- 🎨 **`rustalk`** - TypeScript interface with enhanced UX
- ⚡ **`rus`** - Direct Rust CLI for power users

**Note**: Configuration persists across updates. Reset with `rustalk reset` if needed.

## License

Licensed under Apache 2.0. For more details, see the [LICENSE](LICENSE) file.

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for detailed information about contributing to this project.
