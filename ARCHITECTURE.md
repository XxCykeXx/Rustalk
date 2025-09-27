# Rustalk Architecture Overview

## 🏗️ Modular Design

Rustalk has been restructured into a clean three-tier modular architecture:

```
┌─────────────────────────────────────────────────┐
│                    rustalk                      │
│              (Unified Experience)               │
│          Advanced UI + System Management        │
├─────────────────────────────────────────────────┤
│                      rus                        │
│               (CLI Operations)                  │
│         Direct Command Interface + Chat         │
├─────────────────────────────────────────────────┤
│                     reach                       │
│           (Core P2P Networking)                 │
│    Crypto + Sessions + Networking + Identity    │
└─────────────────────────────────────────────────┘
```

## 📦 Package Breakdown

### 🌐 **reach** - Core P2P Networking Library
**Location**: `packages/reach/`  
**Purpose**: Foundation library providing all core functionality

**Key Modules**:
- `network.rs` - P2P networking and discovery  
- `crypto.rs` - End-to-end encryption  
- `session.rs` - Chat session management (`SessionManager`)
- `cli.rs` - Common CLI operations (`CliOperations`, `UserManager`, `PathManager`)
- `identity.rs` - User identity and key management
- `config.rs` - Configuration management
- `peer.rs` - Peer connection handling
- `message.rs` - Message types and serialization

**Exports**: All core functionality is available as public API

### ⚡ **rus** - Direct CLI Interface  
**Location**: `packages/rus/`  
**Purpose**: User-friendly CLI wrapper for reach functionality

**Features**:
- Interactive chat sessions
- Simple command interface  
- Direct access to all reach operations
- Minimal dependencies (uses reach only)

**Usage**: `rus setup`, `rus chat`, `rus connect <address>`, etc.

### 🚀 **rustalk** - Unified Experience
**Location**: `rustalk/`  
**Purpose**: Advanced application combining reach core with enhanced UI

**Features**:
- Advanced TUI interface
- System PATH management  
- Multi-user registry
- Enhanced chat application
- Full reach integration

**Usage**: `rustalk_cli setup`, `rustalk_cli chat`, etc.

## 🎯 Installation Model

### Single Installation Point
Users install once and get access to both interfaces:

```bash
# NPM Installation
npm install -g rustalk
# Provides: rustalk (TypeScript wrapper) + rus (direct CLI)

# Cargo Installation  
cargo install rustalk
# Provides: rustalk_cli (native binary) + rus (direct CLI)
```

### Dual Interface Strategy
- **🚀 rustalk**: TypeScript wrapper providing unified experience
  - Delegates all operations to `rus` backend
  - Enhanced help and user experience
  - Cross-platform compatibility via NPM
  
- **⚡ rus**: Direct Rust CLI for power users
  - Direct access to reach functionality
  - Lightweight and fast
  - No TypeScript overhead

## 🔄 Dependency Flow

```
rustalk (TypeScript) ──delegates──> rus (CLI) ──uses──> reach (Core)
        │                                                    ▲
        └──────────── for system management ─────────────────┘

rustalk_cli (Native) ──uses──> reach (Core)
```

**Key Principles**:
- ✅ Clean dependency chain: `rus` → `reach`, `rustalk` → `reach`
- ✅ No circular dependencies
- ✅ Each package can be used independently
- ✅ TypeScript wrapper delegates to native Rust for performance

## 🚀 User Experience

### Quick Start Flow
1. **Install**: `npm install -g rustalk` or `cargo install rustalk`
2. **Setup**: `rustalk setup` (or `rus setup`)  
3. **Chat**: `rustalk chat` (or `rus chat`)
4. **Connect**: `rustalk connect <address>` (or `rus connect <address>`)

### Command Equivalence
Every command works with both interfaces:

| TypeScript Interface | Direct CLI | Description |
|---------------------|------------|-------------|
| `rustalk setup`     | `rus setup` | Configure identity |
| `rustalk chat`      | `rus chat` | Start chat session |
| `rustalk connect`   | `rus connect` | Connect to peer |
| `rustalk info`      | `rus info` | Show user info |

### Advanced Features
- **PATH Management**: `rustalk add-path` / `rustalk remove-path`
- **User Registry**: `rustalk list-users` / `rustalk switch-user`
- **System Integration**: Automatic installation of both binaries

## 🏆 Benefits of This Architecture

### For Users
- **Simple Installation**: One command gets everything
- **Choice of Interface**: Use rustalk for enhanced UX or rus for direct access
- **Consistent Commands**: Same operations work with both interfaces
- **Cross-Platform**: NPM provides universal compatibility

### For Developers  
- **Modular Development**: Each package has a single responsibility
- **Clean Dependencies**: No circular references or tight coupling
- **Independent Testing**: Each package can be tested in isolation
- **Flexible Deployment**: Packages can be used independently if needed

### For Distribution
- **Unified Publishing**: Single NPM/Cargo package provides complete solution
- **Version Synchronization**: All packages stay in sync automatically
- **Binary Distribution**: Both TypeScript and native Rust binaries included

## 🔧 Development Workflow

### Building
```bash
# Build all packages
cargo build --release --workspace

# Build TypeScript wrapper  
npm run build:ts

# Build everything
npm run build
```

### Testing
```bash
# Test Rust packages
cargo test --workspace

# Test TypeScript integration
npm test
```

### Publishing
```bash
# NPM (includes all binaries)
npm publish

# Cargo (individual packages)
cargo publish -p reach
cargo publish -p rus  
cargo publish -p rustalk
```

This architecture successfully delivers on the user's vision: a modular platform where users can install once and use both `rustalk` and `rus` commands, with clean separation between networking core (reach), CLI operations (rus), and unified experience (rustalk).