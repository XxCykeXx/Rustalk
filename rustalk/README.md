# Rustalk 🚀

**Binary Installer and Starter for Rustalk**

Rustalk is the main binary package that serves as the installer and starter for the entire Rustalk ecosystem. It provides both Cargo and npm installation methods while delegating actual operations to the `rus` CLI.

## Features

- 📦 **Dual Installation**: Available via both Cargo and npm
- 🚀 **Easy Starter**: Simple commands to get up and running quickly
- 🛠️ **System Integration**: Automatic PATH management and system setup
- ⚡ **Delegation Architecture**: Delegates operations to optimized `rus` CLI
- 🌐 **Universal Access**: Works across different package managers
- 🔧 **Installation Management**: Handles system-wide installation and setup

## Installation

### Via Cargo (Recommended)
```bash
cargo install rustalk
rustalk install  # Add to system PATH
rustalk start    # Quick start
```

### Via npm
```bash
npm install -g rustalk
rustalk install  # Add to system PATH
rustalk start    # Quick start
```

## Usage

### Quick Start
```bash
# Install system-wide and add to PATH
rustalk install

# Start chat application (auto-setup if needed)
rustalk start

# Start on custom port
rustalk start --port 6000
```

### Advanced Usage
```bash
# Pass commands directly to rus CLI
rustalk run setup
rustalk run users list
rustalk run chat --port 5000
rustalk run connect 192.168.1.100:5000

# System management
rustalk run path add
rustalk run config show
```

### Library Usage (for npm integration)
```rust
use rustalk::{reach, rus};

// Access reach functionality
use reach::{ReachEngine, UserCredentials};

// Access rus functionality  
use rus::{add_to_path, UserRegistry};
```

## Commands

### Installation & Setup
- `rustalk install` - Install rustalk system-wide and add to PATH
- `rustalk start [--port PORT]` - Quick start with auto-setup

### Delegation to rus
- `rustalk run <args>` - Pass any arguments to the `rus` CLI
- `rustalk run setup` - Setup user credentials
- `rustalk run chat` - Start chat session
- `rustalk run users list` - List users

## Architecture

Rustalk acts as a convenient installer and starter that:

1. **Installs** the complete Rustalk system via Cargo or npm
2. **Manages** system PATH and installation location
3. **Delegates** actual operations to the `rus` CLI for optimal performance
4. **Provides** library access to both `reach` and `rus` components

```
rustalk (installer/starter)
    │
    ├─── Direct: install, start commands
    │
    └─── Delegates to: rus CLI
                    │
                    └─── Uses: reach library
```

## Library Access

When used as a library (especially for npm integration), rustalk re-exports both `reach` and `rus` modules:

- **`reach`**: Core P2P networking and encryption
- **`rus`**: CLI operations, user management, and system configuration

## Building

```bash
# Build binary
cargo build --release

# Build with TypeScript (for npm)
npm run build
```

## License

Apache-2.0 License. See [LICENSE](../LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

---

Part of the [Rustalk](../README.md) project.